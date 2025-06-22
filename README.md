# 🚀 Fullstack Developer Setup: Blockchain, IoT, Rust, Monitoring
Panduan ini menjelaskan **langkah lengkap instalasi** environment untuk pengembangan proyek blockchain, sistem pemantauan IoT, dan backend modern menggunakan:
- Hardhat (Smart Contract Ethereum)
- InfluxDB 2 (Time Series Database)
- Grafana (Data Visualization)
- Node.js + npm (JavaScript Ecosystem)
- Rust (System Programming Language)
---
## 📦 1. Install Node.js dan npm
Node.js dan npm dibutuhkan untuk menjalankan Hardhat dan tool berbasis JavaScript lainnya.
sudo apt update
sudo apt install curl -y
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt install -y nodejs
## 2. Install Hardhat 
npm init -y
npm install --save-dev hardhat
npx hardhat
