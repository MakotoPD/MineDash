![banner](https://i.imgur.com/KWatIFf.png)

# Server Creator (VoidLink)

A modern, powerful, and easy-to-use desktop application for creating and managing Minecraft servers. Built with **Tauri 2.0**, **Nuxt 3**, and **TypeScript**.

##  Images

![main menu](https://i.imgur.com/SW6SLiH.png)

<details>
  <summary>Server Dashboard</summary>

  ![server performance](https://i.imgur.com/s5PjaC9.png)
  
  ![server console](https://i.imgur.com/sdbFnFh.png)

  ![server settings](https://i.imgur.com/cJO2yr1.png)

  ![server mods/plugins](https://i.imgur.com/K7C2wRa.png)

  ![server mods/plugins download](https://i.imgur.com/viQHTyq.png)

  ![server players](https://i.imgur.com/lZQhGpH.png)
</details>


<details>
  <summary>App Settings</summary>

  ![app settings](https://i.imgur.com/qWaLMWY.png)

</details>

<details>
  <summary>Accound & Tunnels</summary>

  ![main](https://i.imgur.com/2Dx6CmH.png)

  ![create tunnel](https://i.imgur.com/1o9B3VI.png)

</details>


<details>
  <summary>Create Server</summary>

  ![select engine](https://i.imgur.com/WT4MkwN.png)

  ![selected engine](https://i.imgur.com/nIaLQBj.png)

  ![download modpack](https://i.imgur.com/BlgE13j.png)

  ![world](https://i.imgur.com/HXLIa5O.png)

  ![server settings](https://i.imgur.com/yHIZauR.png)

  ![launch server](https://i.imgur.com/E8Znz8D.png)

</details>

<details>
  <summary>System Tray</summary>

  ![system tray](https://i.imgur.com/GLJoCnn.png)

</details>

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

### 🌍 Built-in Tunneling (VoidLink Tunnels)
Safe and easy way to expose your local Minecraft server to the internet without port forwarding.
This application uses our custom tunneling solution: [VoidLink-Tunnels](https://github.com/MakotoPD/VoidLink-Tunnels).

- **No Port Forwarding needed**: Works behind NAT and CGNAT.
- **Secure**: Uses token-based authentication.
- **TCP & UDP Support**: Supports both game traffic and Voice Chat.


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
- [X] **Advanced Network**: Tunnel integration.
- [X] **Multi-Version Support**: Better handling of Java versions manager for older Minecraft versions.

## 🍎 macOS Troubleshooting (App is damaged)

If you see a message saying **"VoidLink is damaged and can't be opened"**, this is a normal security check for apps not signed with a paid Apple Developer certificate.

To fix this, run the following command in your terminal:
```bash
sudo xattr -cr /Applications/VoidLink.app
```
(Make sure the app is in your Applications folder first).
