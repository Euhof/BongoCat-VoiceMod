# 🐾 BongoCat VoiceMod

<div align="center">
  <div>
    <p><b>Modified by: Euhof 🇧🇷</b></p>
    <img width="1600" height="900" alt="bongo_show" src="https://github.com/user-attachments/assets/91babfb2-a9c4-4e62-80ca-9ef22921ec98" />
    <br />
  </div>
    <br />
  <div>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img alt="Windows" src="https://img.shields.io/badge/-Windows-blue?style=flat-square&logo=windows&logoColor=white" /></a>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img alt="MacOS" src="https://img.shields.io/badge/-MacOS-black?style=flat-square&logo=apple&logoColor=white" /></a>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img alt="Linux X11" src="https://img.shields.io/badge/-Linux_(X11)-yellow?style=flat-square&logo=linux&logoColor=white" /></a>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img alt="Linux Wayland" src="https://img.shields.io/badge/-Linux_(Wayland)-orange?style=flat-square&logo=wayland&logoColor=white" /></a>
  </div>

  <p>
    <a href="./LICENSE"><img src="https://img.shields.io/github/license/Euhof/BongoCat-VoiceMod?style=flat-square" /></a>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img src="https://img.shields.io/github/release/Euhof/BongoCat-VoiceMod?style=flat-square"/></a>
    <a href="https://github.com/Euhof/BongoCat-VoiceMod/releases"><img src="https://img.shields.io/github/downloads/SEU_USUARIO/BongoCat/total?style=flat-square"/></a>
  </p>
  
  <p><b>An enhanced, cross-platform Bongo Cat overlay with reactive audio, voice effects, and full Wayland & X11 support.</b></p>
</div>

| macOS | Windows | Linux (X11) | Linux (Wayland) |
| :---: | :---: | :---: | :---: |
| ![macOS](https://i0.hdslb.com/bfs/openplatform/dff276b96d49c5d6c431b74b531aab72191b3d87.png) | ![Windows](https://i0.hdslb.com/bfs/openplatform/a4149b753856ee7f401989da902cf3b5ad35b39e.png) | ![Linux X11](https://i0.hdslb.com/bfs/openplatform/3b49f961819d3ff63b2b80251c1cc13c27e986b0.png) | ![Linux Wayland](https://github.com/user-attachments/assets/d4c839dc-b7e4-4e98-93af-7868a87af5fa) |

---

## What is BongoCat VoiceMod?

**BongoCat VoiceMod** is a fork of the cross-platform Tauri project [ayangweb/BongoCat](https://github.com/ayangweb/BongoCat). It enhances the desktop companion experience with:
- **Audio & Voice Reactivity:** Makes the cat react to your microphone/voice input and custom sound effects.
- **Wayland Support:** Expanded Linux compatibility to run seamlessly on modern Wayland desktop environments in addition to X11.

---

## Features

- 🔊 **Voice & Audio Mod (New):** Integrated audio features (voice-activated mouth movements, audio-reactive triggers, and custom sound effects).
- 🐧 **Full Linux Support (New):** Compatible with both **Wayland** and **X11** display servers.
- 🎮 **Real-time Tracking:** Accurately synchronizes paws and movement with your **Keyboard**, **Mouse**, or **Gamepad**.
- 💻 **Cross-Platform:** Works natively on macOS (Apple Silicon & Intel), Windows and Linux.
- 🎨 **Custom Models & Skins:** Easily import, convert, and customize cat models.
- ⚡ **Lightweight & Efficient:** Built with Rust and Tauri for minimal CPU and RAM usage.
- 🔒 **Privacy First:** 100% open-source, runs completely offline, and collects zero personal data.

---

## Download

Grab the latest pre-built binaries from the **[GitHub Releases](https://github.com/Euhof/BongoCat-VoiceMod/releases)** page.

For detailed platform-specific instructions, see the **[Download & Installation Guide](.github/DOWNLOAD_GUIDE.md)**.

---

##  Custom Models & Tools

### Community Models
Explore and share community-made models:
- 📦 **[Awesome-BongoCat Repository](https://github.com/ayangweb/Awesome-BongoCat)**

---

## Development & Build

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- [Rust & Cargo](https://rustup.rs/)

### Setup

```bash
# 1. Clone this repository
git clone https://github.com/Euhof/BongoCat-VoiceMod.git
cd BongoCat

# 2. Install dependencies
pnpm install

# 3. Run in development mode
pnpm tauri dev

# 4. Build for production
pnpm tauri build
