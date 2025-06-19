use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use anyhow::{Context as AnyhowContext, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::env;

use influxdb2::Client;
use influxdb2::models::DataPoint;
use chrono::{Utc, DateTime, NaiveDateTime};
use futures_util::stream::iter;

use ethers::prelude::*;
use ethers::abi::Abi;

use tokio_serial::{SerialPortBuilderExt, DataBits, Parity, StopBits};
use tokio_modbus::prelude::*;
use tokio_modbus::client::rtu;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const TCP_SERVER_ADDRESS: &str = "127.0.0.1:8080";
const INFLUXDB_URL: &str = "http://localhost:8086";
const INFLUXDB_ORG: &str = "monitoring";
const INFLUXDB_BUCKET: &str = "datasensor";
const INFLUXDB_TOKEN: &str = "CBnS419-R1vL9EOWQkTXTnkOLLkTBoCBxF17g3EpKyvPfeb6qR1pDc1QziOxGvcr_bCT3ZpMk2jSJH6qG9c3OQ==";
const DEVICE_ID: &str = "device-01";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct SensorData {
    temperature: f32,
    humidity: f32,
    timestamp: i64,
}

#[derive(Serialize)]
struct InfluxFormatJson {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

fn save_to_format_json(data: &SensorData) -> Result<()> {
    let naive = NaiveDateTime::from_timestamp_opt(data.timestamp / 1_000_000_000, 0)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp(0, 0));
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);

    let json = InfluxFormatJson {
        timestamp: datetime.to_rfc3339(),
        sensor_id: DEVICE_ID.to_string(),
        location: "Gudang Fermentasi 1".to_string(),
        process_stage: "Fermentasi".to_string(),
        temperature_celsius: data.temperature,
        humidity_percent: data.humidity,
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("format.json")?;

    write!(file, "{}", serde_json::to_string_pretty(&json)?)?;
    println!("[Writer] 📄 format.json disimpan.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    println!("✅ File .env telah dimuat.");

    let use_dummy = false;

    let shared_data = Arc::new(Mutex::new(SensorData::default()));
    let tcp_task_data = Arc::clone(&shared_data);
    let writer_task_data = Arc::clone(&shared_data);

    let sensor_handle = if use_dummy {
        let dummy_data = Arc::clone(&shared_data);
        Some(tokio::spawn(async move {
            run_dummy_sensor(dummy_data).await?;
            Ok::<(), anyhow::Error>(())
        }))
    } else {
        let real_data = Arc::clone(&shared_data);
        Some(tokio::spawn(run_serial_reader(real_data)))
    };

    let tcp_server_handle = tokio::spawn(run_tcp_server(tcp_task_data));
    let writer_handle = tokio::spawn(run_database_writer(writer_task_data));

    println!("🚀 Semua service telah dimulai.");
    if use_dummy {
        println!("🧪 Mode: Dummy Sensor Aktif");
    } else {
        println!("📡 Mode: Pembacaan Sensor Asli Aktif pada /dev/ttyUSB0");
    }
    println!("🔌 TCP Server mendengarkan di {}", TCP_SERVER_ADDRESS);
    println!("💾 Database writer berjalan.");

    let _ = tokio::try_join!(sensor_handle.unwrap(), tcp_server_handle, writer_handle)?;

    Ok(())
}

async fn run_dummy_sensor(data: Arc<Mutex<SensorData>>) -> Result<()> {
    let mut rng = StdRng::from_entropy();

    loop {
        let dummy = SensorData {
            temperature: rng.gen_range(20.0..30.0),
            humidity: rng.gen_range(40.0..60.0),
            timestamp: Utc::now().timestamp_nanos_opt().unwrap_or(0),
        };

        {
            let mut guard = data.lock().await;
            *guard = dummy.clone();
        }

        println!("[Dummy] ✅ Data dummy diperbarui: {:?}", dummy);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn run_serial_reader(data: Arc<Mutex<SensorData>>) -> Result<()> {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;
    let slave_id = Slave(0x01);

    let builder = tokio_serial::new(port_name, baud_rate)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_secs(1));

    let port = builder.open_native_async().context("Gagal membuka port serial")?;
    let mut ctx = rtu::attach_slave(port, slave_id);

    println!("[Serial] ✅ Listening Modbus RTU pada {}", port_name);

    loop {
        match ctx.read_input_registers(1, 2).await {
            Ok(response) if response.len() == 2 => {
                let temp = response[0] as f32 / 10.0;
                let hum = response[1] as f32 / 10.0;

                let parsed = SensorData {
                    temperature: temp,
                    humidity: hum,
                    timestamp: Utc::now().timestamp_nanos_opt().unwrap_or(0),
                };

                let mut guard = data.lock().await;
                *guard = parsed;
                println!("[Serial] ✅ Data diterima: {:?}", *guard);
            }
            Ok(response) => {
                println!("[Serial] ⚠️ Response tidak lengkap: {:?}", response);
            }
            Err(e) => {
                eprintln!("[Serial] ❌ Gagal baca dari sensor: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

async fn run_tcp_server(data: Arc<Mutex<SensorData>>) -> Result<()> {
    let listener = TcpListener::bind(TCP_SERVER_ADDRESS).await?;
    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("[TCP Server] 🔗 Koneksi dari: {}", addr);
        let data_clone = Arc::clone(&data);

        tokio::spawn(async move {
            let data_guard = data_clone.lock().await;
            let response = serde_json::to_string(&*data_guard).unwrap_or_default();

            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("[TCP Server] ❌ Gagal kirim: {}", e);
            } else {
                println!("[TCP Server] ✅ Data dikirim ke {}", addr);
            }
        });
    }
}

async fn run_database_writer(data: Arc<Mutex<SensorData>>) -> Result<()> {
    let eth_rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL harus diset di .env");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS harus diset di .env");
    let gateway_private_key = env::var("GATEWAY_PRIVATE_KEY").expect("GATEWAY_PRIVATE_KEY harus diset di .env");

    let influx_client = Client::new(INFLUXDB_URL, INFLUXDB_ORG, INFLUXDB_TOKEN);

    let provider = Provider::<Http>::try_from(eth_rpc_url)?;
    let wallet: LocalWallet = gateway_private_key.parse::<LocalWallet>()?.with_chain_id(provider.get_chainid().await?.as_u64());
    let signer = SignerMiddleware::new(provider, wallet.clone());

    let abi_file = File::open("src/abi/DataRegistry.json")?;
    let reader = BufReader::new(abi_file);
    let abi: Abi = serde_json::from_reader(reader)?;

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, Arc::new(signer));
    let mut last_written_time = 0i64;

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;

        let current_data: SensorData;
        {
            let data_guard = data.lock().await;
            if data_guard.timestamp > last_written_time {
                current_data = data_guard.clone();
                last_written_time = current_data.timestamp;
            } else {
                println!("[Writer] ⏸ Menunggu data sensor baru...");
                continue;
            }
        }

        // Simpan format.json
        if let Err(e) = save_to_format_json(&current_data) {
            eprintln!("[Writer] ❌ Gagal simpan format.json: {}", e);
        }

        let point = DataPoint::builder("sensor_data")
            .tag("device_id", DEVICE_ID)
            .field("temperature", current_data.temperature as f64)
            .field("humidity", current_data.humidity as f64)
            .timestamp(current_data.timestamp)
            .build()?;

        match influx_client.write(INFLUXDB_BUCKET, iter(vec![point])).await {
            Ok(_) => {
                println!(
                    "[Writer] ✅ Data ditulis ke InfluxDB: {:.2}°C, {:.2}%RH @ {}",
                    current_data.temperature, current_data.humidity, current_data.timestamp
                );
            }
            Err(e) => {
                eprintln!("[Writer] ❌ Gagal tulis ke InfluxDB: {}", e);
            }
        }

        println!("[Writer] 🚀 Mengirim ke Blockchain...");
        let temp_for_chain = (current_data.temperature * 100.0) as i64;
        let hum_for_chain = (current_data.humidity * 100.0) as i64;

        let call = contract.method::<_, ()>("recordData", (DEVICE_ID.to_string(), temp_for_chain, hum_for_chain))?;

        match call.send().await {
            Ok(pending_tx) => {
                println!("[Writer] ✅ Transaksi Blockchain terkirim! Hash: {:?}", pending_tx.tx_hash());
            }
            Err(e) => {
                eprintln!("[Writer] ❌ Gagal kirim Blockchain: {}", e);
            }
        }
    }
}
