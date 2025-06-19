const hre = require("hardhat");
const fs = require("fs");
const path = require("path");

async function main() {
  // 1. Deploy kontrak
  const dataRegistry = await hre.ethers.deployContract("DataRegistry");
  await dataRegistry.waitForDeployment();
  const contractAddress = dataRegistry.target;
  console.log(`DataRegistry deployed to: ${contractAddress} on network ${hre.network.name}`);

  // 2. Tentukan konfigurasi berdasarkan jaringan yang aktif
  let config;
  if (hre.network.name === "sepolia") {
    config = {
      ETH_RPC_URL: process.env.SEPOLIA_RPC_URL, // Ambil dari .env Hardhat
      GATEWAY_PRIVATE_KEY: process.env.WALLET_PRIVATE_KEY, // Ambil dari .env Hardhat
      CONTRACT_ADDRESS: contractAddress,
      INFLUXDB_TOKEN: "YOUR_API_TOKEN", 
    };
  } else { // Asumsikan localhost jika bukan sepolia
    config = {
      ETH_RPC_URL: "http://127.0.0.1:8545",
      GATEWAY_PRIVATE_KEY: "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
      CONTRACT_ADDRESS: contractAddress,
      INFLUXDB_TOKEN: "YOUR_API_TOKEN",
    };
  }
  
  // 3. Tulis konfigurasi ke file .env Rust dan React
  const rustEnvPath = '/home/adam/Documents/ISI_AAM/sensor-gateaway/.env';
  writeEnvFile(rustEnvPath, config);
  console.log(`\n✅ Konfigurasi untuk Backend Rust berhasil ditulis.`);

  const reactConfig = { REACT_APP_CONTRACT_ADDRESS: contractAddress };
  const reactEnvPath = '/home/adam/Documents/ISI_AAM/frontend-dapp/.env';
  writeEnvFile(reactEnvPath, reactConfig);
  console.log(`✅ Konfigurasi untuk Frontend React berhasil ditulis.`);
}

/**
 * Fungsi helper untuk membuat direktori (jika belum ada) dan menulis file .env.
 * @param {string} filePath - Path lengkap ke file .env yang akan ditulis.
 * @param {object} configObject - Objek yang berisi key-value untuk konfigurasi.
 */
function writeEnvFile(filePath, configObject) {
  const dirPath = path.dirname(filePath);
  if (!fs.existsSync(dirPath)) {
    fs.mkdirSync(dirPath, { recursive: true });
    console.log(`Direktori dibuat: ${dirPath}`);
  }

  const envFileContent = Object.entries(configObject)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

  fs.writeFileSync(filePath, envFileContent);
}


main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});