# 🌦️ Weather Oracle (Soroban Smart Contract)

## 📌 Project Overview

Weather Oracle is a decentralized smart contract built on the Stellar network using Soroban. It enables applications to securely store and retrieve weather data on-chain, acting as a bridge between real-world weather information and blockchain-based systems.

This project demonstrates how real-world data (off-chain) can be brought on-chain in a simple, efficient, and tamper-resistant way.

---

## ⚙️ What It Does

The Weather Oracle contract allows trusted data providers (oracles) to:

* Submit weather data for a specific location
* Store the data permanently on-chain
* Allow anyone to fetch the latest weather data for that location

---

## 🚀 Key Features

* 🌍 Location-based weather storage
* 📊 Data includes temperature, humidity, and weather condition
* 🔐 Tamper-resistant on-chain storage
* ⚡ Fast and low-cost execution using Soroban
* 🔌 Easy integration with Web3 applications
* 🧩 Oracle-ready architecture for future expansion

---

## 🧠 How It Works

1. An external data provider (oracle) fetches weather data from APIs
2. The oracle calls the smart contract to store the data
3. The contract saves the latest weather data by location
4. Users and dApps can query the contract anytime

---

## 🔗 Deployed Smart Contract

**Network:** Stellar Testnet
**Contract ID:**
`CCYPO5XUAISZLN4RW7XGFVE4X5X34DRNHUMPJMP3Z3J7OSLUFCVZBTF4`

**Explorer Link:**
👉 https://stellar.expert/explorer/testnet/contract/CCYPO5XUAISZLN4RW7XGFVE4X5X34DRNHUMPJMP3Z3J7OSLUFCVZBTF4

---

## 🛠️ Contract Functions

### 🔹 set_weather

Stores weather data for a location.

**Parameters:**

* `location` (String)
* `temperature` (i32)
* `humidity` (u32)
* `condition` (Symbol)

---

### 🔹 get_weather

Retrieves stored weather data.

**Parameter:**

* `location` (String)

**Returns:**

* WeatherData struct (temperature, humidity, condition)

---

## 📦 Example Usage

### Set Weather Data

```bash
set_weather("Kolkata", 32, 70, "Sunny")
```

### Get Weather Data

```bash
get_weather("Kolkata")
```

---

## 💡 Use Cases

* 🌾 Crop & weather-based insurance
* 🎯 Prediction markets
* 📉 Climate analytics
* 🌐 Web3 applications needing real-world data
* 🏦 DeFi protocols reacting to weather conditions

---

## 🧪 Future Enhancements

* 🔗 Integration with real weather APIs (e.g., OpenWeather)
* 🧑‍🤝‍🧑 Multiple oracle providers (decentralized consensus)
* ⏱️ Timestamped updates
* 🔐 Access control (only authorized oracles can update data)
* 📡 Automated data feeds

---

## 🛠️ Tech Stack

* Rust 🦀
* Soroban SDK
* Stellar Blockchain

---

## 📜 License

MIT License

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you'd like to change.

---

## ⭐ Acknowledgements

Built as part of learning and exploring decentralized oracle systems on Soroban.
