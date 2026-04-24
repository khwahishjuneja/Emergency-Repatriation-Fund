# ✈️ Emergency Flight Pool (Soroban Smart Contract)

## 📌 Project Description

Emergency Flight Pool is a decentralized mutual-aid smart contract built using Stellar's Soroban smart contract platform.  

It enables a community-driven funding pool where members contribute funds and can request financial support for emergency travel — such as flights back home during crises (medical emergencies, political instability, family issues, etc.).

This project demonstrates how blockchain can be used to create transparent, trustless social safety nets without relying on centralized institutions.

---

## ⚙️ What It Does

The smart contract allows users to:

- Register as members of the emergency pool
- Contribute funds to build a shared reserve
- Request emergency payouts when needed
- Store and track member contributions on-chain
- Emit events for transparency of payouts

Each member’s contribution and status is stored individually on-chain using Soroban’s instance storage model.

---

## 🚀 Features

### 👥 Membership Registration
- Users can register as members of the pool
- Prevents duplicate registrations
- Tracks active status of each member

---

### 💰 Contribution System
- Members can contribute funds to the pool
- Contributions are tracked per user
- Ensures only active members can contribute

---

### 🆘 Emergency Fund Requests
- Members can request emergency payouts
- Requires authentication (secure access)
- Basic eligibility check (must have contributed)
- Emits payout events for transparency

---

### 🔐 On-Chain Storage (Efficient Design)
- Uses per-user storage instead of large maps
- Reduces complexity and improves scalability
- Follows Soroban best practices

---

### 📡 Event Logging
- Emits events when payouts are requested
- Enables off-chain tracking and UI integration

---

## 🧱 Contract Design
wallet address GAKK6RQCZU6G7XJKYQMSFKPBMGET26GH2WWVTFHN5GIVWTWX45EFC5X4

contract address CALMOM7EADIWM4JXS37PNLPGQDOQ6VRQKJ2Z5XC5R35QAILUM3GVOQZ6

<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/b0211613-ca06-470a-bb56-590dfdbead74" />



Instead of storing all users in a single map, each member is stored individually using a composite storage key:
