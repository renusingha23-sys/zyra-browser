# 🌐 Zeon Browser

Zeon Browser is a lightweight browser built with **Rust 🦀** and **Tauri 🪶**, designed to run smoothly even on low-end devices.

---

## ✨ Features

- 🌍 **Fast Web Loading** – Optimized for speed on modest hardware.
- 🧹 **Built-in Cleaner** – Instant cookie and cache clearing on exit.
- 🛡️ **Privacy Focused** – Automatic third-party cookie blocking.
- 🧩 **Minimalist UI** – A distraction-free interface that stays out of your way.

---

## 💻 Supported Platforms

- 🐧 **Linux** (`.deb`, `AppImage`)

---

## 🛠️ Built With

- **Language:** [Rust](https://www.rust-lang.org/)
- **Framework:** [Tauri](https://tauri.app/)
- **Engine:** WebKitGTK (Linux Native via `webkit2gtk`)

---

## 🧠 Philosophy

Modern browsers often assume everyone has powerful hardware. Many people still rely on low-end devices, where heavy browsers struggle to run.

Zeon Browser exists to bridge that gap — providing a simple, lightweight way for people on modest hardware to access the modern web.

> ⚡ **The web should be accessible to everyone, not just powerful machines.**

---

## Compare to modern browsers

Lets compare our Zeon Browser to modern browsers.

| Browser | Cold start time | RAM usage |
| ------- | --------------- | --------- |
| Chrome  | 1.8s            | 420 MB    |
| Firefox | 1.5s            | 350 MB    |
| Zeon    | 0.4s            | 110 MB    |

---

## 🏗️ Development (Linux)

To build Zeon Browser locally, ensure you have the following dependencies installed:

```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev
```

then run
```bash
	cargo run tauri
```
 
 **NOTE: You should install Rust for this.**

 ## 🔗 Learn More
 Visit our official website to learn more about the project:
 👉 [kuroi.dev](https://kuroi.dev/)

 
