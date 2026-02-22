# 🖼️ Filmstrip Photo Viewer (Windows XP Style)

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/e942dcdec1094503aebaee7c873ac719)](https://app.codacy.com/gh/mkaraki/xp-filmstrip-photo?utm_source=github.com&utm_medium=referral&utm_content=mkaraki/xp-filmstrip-photo&utm_campaign=Badge_Grade)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=mkaraki_xp-filmstrip-photo&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=mkaraki_xp-filmstrip-photo)

A high-fidelity Windows XP "Luna" style web-based image viewer and file explorer.

> [!IMPORTANT]
> **🤖 AI-First Project**: This project was conceived, designed, and implemented entirely by **Gemini CLI (AI Agent)**. It is intended to be maintained exclusively through AI interaction.
> 
> **🎮 Just for Fun**: This is a hobby/nostalgia project. The code and architectural quality are **not production-level**.
>
> **⚠️ Disclaimer**: As this project and its documentation (including this README) are entirely AI-generated, they **likely contain mistakes, inaccuracies, or outdated information**. Use with caution and a sense of humor.

## ✨ Features

- **Authentic XP UI**: Recreated Luna theme using vanilla CSS, 3D effects, and classic icons.
- **Multi-View Explorer**:
  - **Filmstrip View**: Large preview with a scrolling thumbnail strip.
  - **Thumbnails View**: 1:1 grid with aspect-ratio preservation.
  - **Details View**: Classic list with file metadata (Size, Type, Date).
- **Slideshow**: Fullscreen mode with autohiding controls.
- **Keyboard Navigation**: Full support for Arrows, Enter, Backspace, Home/End, and PageUp/Down.
- **Internationalization (i18n)**: Fully localized in English and 日本語 (Japanese).
- **Static Site Generation (SSG)**: A specialized script ("Pinner") to crawl the backend and generate a self-contained static version of your photo library.
- **High Performance**: Rust-based backend for fast image resizing and SHA-256 disk caching.

## 🛠️ Tech Stack

- **Frontend**: Nuxt 4 (Vue 3), Vite, @nuxtjs/i18n.
- **Backend**: Rust (Axum), fast-image-resize, image-rs.
- **SSG / Tooling**: Bun, Hono (for test server).

## 🚀 Getting Started

### Prerequisites

- Rust (Cargo)
- Bun (or Node.js/npm)

### 1. Running the Application (Dynamic Server)

The Rust backend is designed to serve the pre-built frontend files.

**Build the Frontend:**
```powershell
cd frontend
bun install
bun run generate
```

**Start the Backend:**
```powershell
cd ../backend
# Ensure a .env file exists with PHOTO_ROOT=path/to/your/photos
cargo run
```
The application will be available at the port specified in your `.env` (default: 8080).

### 2. Static Site Generation (Pinner)

**Generate the Static Build:**

1. Ensure the Backend is running.
2. Run the Generator:

```powershell
cd static-generator
bun install
bun run generate
```

**Serve the Static Build:**
```powershell
cd static-generator
bun install
bun run serve
```

## 📜 License

This project is licensed under the MIT License.

---
*Created with 💙 by Gemini CLI*
