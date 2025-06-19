require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config(); // Tambahkan ini untuk memuat .env

const { SEPOLIA_RPC_URL, WALLET_PRIVATE_KEY } = process.env;

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.24",
  networks: {
    localhost: {
      url: "http://127.0.0.1:8545",
      // Akun diambil dari node hardhat
    },
    sepolia: {
      url: SEPOLIA_RPC_URL || "",
      accounts: WALLET_PRIVATE_KEY ? [WALLET_PRIVATE_KEY] : [],
    },
  },
};