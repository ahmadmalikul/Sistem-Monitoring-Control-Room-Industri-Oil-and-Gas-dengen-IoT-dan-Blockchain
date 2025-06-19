import { useState, useEffect, useRef } from 'react';
import { ethers } from 'ethers';
import DataRegistryABI from './abi/DataRegistry.json';
import {
  LineChart, Line, XAxis, YAxis, Tooltip, CartesianGrid, Legend, ResponsiveContainer
} from 'recharts';

const contractAddress = process.env.REACT_APP_CONTRACT_ADDRESS;
const DEVICE_ID = "device-01";

function App() {
  const [account, setAccount] = useState(null);
  const [readings, setReadings] = useState([]);
  const [contract, setContract] = useState(null);
  const [provider, setProvider] = useState(null);
  const readingsRef = useRef([]);

  const connectWallet = async () => {
    if (window.ethereum) {
      try {
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        const provider = new ethers.BrowserProvider(window.ethereum);
        const signer = await provider.getSigner();
        const contractInstance = new ethers.Contract(contractAddress, DataRegistryABI, signer);

        setAccount(accounts[0]);
        setProvider(provider);
        setContract(contractInstance);
        console.log("🔗 Terhubung ke kontrak:", contractAddress);
      } catch (error) {
        console.error("❌ Gagal menghubungkan dompet:", error);
      }
    } else {
      alert("❗ Harap instal MetaMask!");
    }
  };

  useEffect(() => {
    if (!contract || !provider) return;

    console.log("🟢 Mendengarkan event DataRecorded...");

    const onDataRecorded = async (deviceId, temperature, humidity, event) => {
      try {
        if (deviceId !== DEVICE_ID) return;

        const block = await provider.getBlock(event.blockNumber);
        const newReading = {
          temp: Number(temperature) / 100.0,
          hum: Number(humidity) / 100.0,
          time: new Date(block.timestamp * 1000).toLocaleTimeString('id-ID'),
        };

        readingsRef.current = [...readingsRef.current, newReading];
        setReadings([...readingsRef.current]);

        console.log("📥 Data baru:", newReading);
      } catch (error) {
        console.error("❌ Gagal memproses data event:", error);
      }
    };

    contract.on("DataRecorded", onDataRecorded);

    return () => {
      contract.off("DataRecorded", onDataRecorded);
    };
  }, [contract, provider]);

  return (
    <div className="App" style={{ fontFamily: 'sans-serif', maxWidth: '900px', margin: 'auto', padding: '20px' }}>
      <header>
        <h1>📊 Dashboard Sensor Blockchain (Realtime)</h1>
        {!account ? (
          <button onClick={connectWallet} style={{ padding: '10px 15px', fontSize: '16px' }}>
            Hubungkan Dompet MetaMask
          </button>
        ) : (
          <div>
            <p><strong>Dompet Terhubung:</strong> {account}</p>
            <p><strong>Alamat Kontrak:</strong> {contractAddress}</p>
          </div>
        )}
      </header>

      <main style={{ marginTop: '30px' }}>
        <h2>Data Sensor Masuk (Real-Time)</h2>

        <div style={{ height: '300px', marginBottom: '30px' }}>
          {readings.length > 0 ? (
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={readings}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="time" />
                <YAxis yAxisId="left" domain={['auto', 'auto']} />
                <YAxis yAxisId="right" orientation="right" domain={['auto', 'auto']} />
                <Tooltip />
                <Legend />
                <Line yAxisId="left" type="monotone" dataKey="temp" stroke="#ff7300" name="Suhu (°C)" />
                <Line yAxisId="right" type="monotone" dataKey="hum" stroke="#387908" name="Kelembapan (%RH)" />
              </LineChart>
            </ResponsiveContainer>
          ) : (
            <p>⏳ Menunggu data masuk...</p>
          )}
        </div>

        <div style={{ border: '1px solid #ccc', padding: '10px', background: '#f9f9f9' }}>
          {readings.length === 0 ? (
            <p>Menunggu data dari blockchain...</p>
          ) : (
            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
              <thead>
                <tr style={{ background: '#eee' }}>
                  <th style={{ padding: '8px', border: '1px solid #ddd' }}>Waktu</th>
                  <th style={{ padding: '8px', border: '1px solid #ddd' }}>Suhu (°C)</th>
                  <th style={{ padding: '8px', border: '1px solid #ddd' }}>Kelembapan (%RH)</th>
                </tr>
              </thead>
              <tbody>
                {readings.map((reading, index) => (
                  <tr key={index}>
                    <td style={{ padding: '8px', border: '1px solid #ddd' }}>{reading.time}</td>
                    <td style={{ padding: '8px', border: '1px solid #ddd' }}>{reading.temp.toFixed(2)}</td>
                    <td style={{ padding: '8px', border: '1px solid #ddd' }}>{reading.hum.toFixed(2)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;
