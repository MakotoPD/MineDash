# Server Creator (MineDash)

A modern, powerful, and easy-to-use desktop application for creating and managing Minecraft servers. Built with **Tauri 2.0**, **Nuxt 3**, and **TypeScript**.

##  Images

![main menu](https://i.imgur.com/6A74AtV.png)
![server console](https://i.imgur.com/LVqVzQN.png)
![server settings](https://i.imgur.com/X4EoS9h.png)
![server players](https://i.imgur.com/O8R8ILk.png)
![server tray](https://i.imgur.com/sFD1PTp.png)

## 🚀 Features

### 🖥️ Dashboard & Server Management
- **Intuitive UI**: Beautiful, dark-mode ready interface built with Nuxt UI and TailwindCSS.
- **Server Overview**: Monitor all your servers, their status, and key metrics at a glance.
- **One-Click Start**: Start, stop, and restart servers instantly.
- **System Tray**: Minimize to tray and manage servers from the context menu.

### ⚙️ Enhanced Configuration
- **Unified Settings**: Manage `server.properties` and system settings in one place.
- **Java Management**:
  - Configurable RAM allocation (1GB - 32GB).
  - Custom JVM startup flags (e.g., Aikar's flags).
  - Java path selection.
- **MOTD Editor**: Visual editor with real-time preview and support for Minecraft color/formatting codes.
- **Gameplay Toggles**: Easily switch Gamemode, Difficulty, PVP, Flight, and Command Blocks.
- **Port Management**: Simple numeric input for server port with safety validation.

### 📦 Addons & Mods
- **Modrinth Integration**: Browse, search, and install plugins/mods directly from the app.
- **Automatic Dependencies**: Resolves and installs required dependencies automatically.

### 💻 Console
- **Live Console**: Real-time server logs with command input support.

## 🛠️ Technology Stack

- **Frontend**: Nuxt 3, Vue 3, Nuxt UI, TailwindCSS
- **Backend (OS Integration)**: Tauri 2.0 (Rust)
- **State Management**: Pinia
- **Icons**: Lucide Icons

## 🔮 Future Roadmap

Here is what we plan to add in upcoming updates:

- [X] **Modpack Installer**: One-click installation of popular modpacks (CurseForge/Modrinth).
- [ ] **Backup System**: Automated and manual world backups with restore functionality.
- [X] **Player Manager**: GUI for managing Whitelist, Ops, and Banned players.
- [ ] **Scheduled Tasks**: Auto-restart and scheduled commands.
- [ ] **Advanced Network**: Port forwarding helper or tunnel integration (e.g., playit.gg).
- [ ] **Multi-Version Support**: Better handling of Java versions manager for older Minecraft versions.

## 🍎 macOS Troubleshooting (App is damaged)

If you see a message saying **"MineDash is damaged and can't be opened"**, this is a normal security check for apps not signed with a paid Apple Developer certificate.

To fix this, run the following command in your terminal:
```bash
sudo xattr -cr /Applications/MineDash.app
```
(Make sure the app is in your Applications folder first).
