# 🚀 Fullstack Developer Setup: Blockchain, IoT, Rust, Monitoring
Panduan ini menjelaskan **langkah lengkap instalasi** **dan** **menjalankan sistem blokchain** menggunakan:
- Hardhat (Smart Contract Ethereum)
- InfluxDB 2 (Time Series Database)
- Grafana (Data Visualization)
- Node.js + npm (JavaScript Ecosystem)/ WEB3
- Rust (System Programming Language)
- pyQT5 (dashboard QT)
---
## 📦 1. Install Node.js dan npm
Node.js dan npm dibutuhkan untuk menjalankan Hardhat dan tool berbasis JavaScript lainnya.
-sudo apt update
-sudo apt install curl -y
-curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
-sudo apt install -y nodejs

## 2. Install Hardhat 
-npm init -y
-npm install --save-dev hardhat
-npx hardhat

## 📊 3. Install InfluxDB 2
Tambahkan repository dan install:
-wget -qO- https://repos.influxdata.com/influxdb.key | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/influxdb.gpg
-echo "deb [arch=amd64 signed-by=/etc/apt/trusted.gpg.d/influxdb.gpg] https://repos.influxdata.com/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/influxdb.list
-sudo apt update
-sudo apt install influxdb2

## 📈 4. Install Grafana
-sudo apt install -y software-properties-common
-sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
-wget -q -O - https://packages.grafana.com/gpg.key | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/grafana.gpg
-sudo apt update
-sudo apt install grafana

## 🦀 5. Install Rust Programming Language
-curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
-source $HOME/.cargo/env
 rustc --version
 cargo --version

## 🐍 6. Install PyQt5
-sudo apt update
-sudo apt install python3 python3-pip -y
-pip3 install PyQt5 PyQt5-tools

# Cara Jalankan Sistem Blok Chain
--buka terminal di folder hardhat-sensor-contarct
dan jalankan
-npx hardhat node
untuk mendapatkan privat key
--tambah terminal lagi di folder yang sama 
dan jalankan 
-npx hardhat run scripts/deploy.js --network localhost
untuk mendeploy privat key menjadi smart contacrt
--tambah terminal lagi di folder sensor-gateaway
dan jalankan
-cargo run
untuk menjalankan program sensor sht20 dan akan mengirim ke TCP server, InfluxDB, dan ke server Etherium
--tambah terminal lagi di folder frontend-dapp
dan jalankan 
-npm start
setelah ada pilihan (y/n) ketik y
--tambah terminal lagi di folder qtbaru
dan jalankan
-python3 tcp_sensor_chart.py
untuk melihat dashboard gui nya 
## Jalankan InfluxDB:
-sudo systemctl enable influxdb
-sudo systemctl start influxdb
atau Buka di browser: http://localhost:8086
## Jalankan Grafana:
-sudo systemctl enable grafana-server
-sudo systemctl start grafana-server
atau buka di : http://localhost:3000
Default login: admin / admin

