# 🏗️ Open Source Engineering/Construction (PH) Monitoring System - Web API Server

> **Status**: Actively in Development 🚧  
> *Contributions welcome! We're building this to help improve construction monitoring in the Philippines.*

## 📌 Table of Contents
- [Features](#-features)
- [Tech Stack](#-tech-stack)
- [Setup](#-setup)
- [Configuration](#-configuration)
- [Development](#-development)
- [Contributing](#-contributing)

## ✨ Features
- Real-time project monitoring dashboard
- JWT authentication 🔐
- API analytics integration 📊
- Construction-specific metrics tracking
- High-performance Rust backend ⚡

## 🛠️ Tech Stack
**Backend:**
- 🦀 Rust (for safety and performance)
- ⚡ Axum web framework
- 🐬 MySQL database
- 📡 API Analytics integration

## 🚀 Setup

### Prerequisites
- Rust 1.70+ (`rustup install stable`)
- MySQL 8.0+
- Cargo (comes with Rust)

### Installation
```bash
# 1. Clone repository
git clone https://github.com/your-repo/construction-monitoring.git
cd construction-monitoring

# 2. Setup environment (copy .env.example to .env)
cp .env.example .env

# 3. Run the server
cargo run