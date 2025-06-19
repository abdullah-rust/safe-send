# 🔐 SafeSend

SafeSend is a blazing-fast, secure chat and file-sharing desktop application built with **Rust**, **Axum**, **React**, and **Tauri**. Designed for performance, privacy, and real-time communication.

![SafeSend UI Banner]([https://your-image-link-here.com/banner.png](https://videos.openai.com/vg-assets/assets%2Ftask_01jx8g4pa3e3zbr68mv7jayr15%2F1749410823_img_0.webp?st=2025-06-19T01%3A32%3A47Z&se=2025-06-25T02%3A32%3A47Z&sks=b&skt=2025-06-19T01%3A32%3A47Z&ske=2025-06-25T02%3A32%3A47Z&sktid=a48cca56-e6da-484e-a814-9c849652bcb3&skoid=8ebb0df1-a278-4e2e-9c20-f2d373479b3a&skv=2019-02-02&sv=2018-11-09&sr=b&sp=r&spr=https%2Chttp&sig=ym86Hk1QKiQFkfP%2BQc9Gk%2FedcfHQwGKLkiJOfzCJjM0%3D&az=oaivgprodscus)) <!-- optional image -->

---

## 🚀 Features

- ✅ JWT-based authentication system
- ✅ 6-digit email code verification
- 🔐 End-to-end encrypted chat
- ⚡ Ultra-fast file transfers
- 🖥️ Cross-platform Tauri desktop app + mobile (lightweight)
- 🌐 Built with a Rust-based backend (Axum)
- 🛠️ Real-time backend performance

---

## 🧱 Tech Stack

| Layer      | Technology        |
|------------|-------------------|
| Frontend   | React + Tauri     |
| Backend    | Rust + Axum       |
| Security   | JWT Authentication |
| State Mgmt | React Hooks       |
| API Comm   | Tauri invoke API  |

---

## 📁 Project Structure

safesend/
├── main/ # Rust + Axum backend
│ └── src/
├── safesend/ # React + Tauri frontend
│ └── src/
├── .gitignore
├── README.md
└── Cargo.toml # (if applicable)


---

## 🧪 How to Run (Dev Mode)

### 🖥️ Backend (Rust)
```bash
cd main
cargo run
🌐 Frontend (Tauri + React)
bash
Copy
Edit
cd safesend
npm install
npm run tauri dev
