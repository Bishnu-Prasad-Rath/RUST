# 🛡️ Windows Process Sentinel (`daemon`)

> **The High-Performance Developer Workstation Orchestrator, Process Monitor & Mini Defender.**

Windows Process Sentinel is a lightweight, low-latency developer productivity suite built natively in Rust. It eliminates repetitive terminal workflows by automating multi-tier development environments, monitoring system RAM consumption, terminating frozen processes, and safeguarding your workstation against rogue memory leaks.

---

## 📑 Table of Contents

* [✨ Key Features](#-key-features)
* [📦 Installation](#-installation)
* [🚀 Quick Start Guide](#-quick-start-guide)

  * [1. Universal Multi-Target Launcher](#1-universal-multi-target-launcher)
  * [2. Automated Project Staging (`sentinel.yml`)](#2-automated-project-staging-sentinelyml)
  * [3. Real-Time TUI Dashboard & Monitor](#3-real-time-tui-dashboard--monitor)
  * [4. Workspace Snapshot & Restoration](#4-workspace-snapshot--restoration)
  * [5. System Maintenance & Package Management](#5-system-maintenance--package-management)
* [⌨️ Global Emergency Panic Hotkey](#️-global-emergency-panic-hotkey)
* [📋 Complete Command Reference](#-complete-command-reference)
* [🔧 Troubleshooting & PATH Setup](#-troubleshooting--path-setup)
* [📄 License](#-license)

---

## ✨ Key Features

* **⚡ Universal Launcher:** Launch multiple local applications, native URI protocols, and web portals concurrently with intelligent desktop-vs-browser routing.
* **🏗️ Context-Aware Workspace Staging:** Automatically trigger full-stack environments such as databases, backend servers, frontend clients, and documentation using a declarative `sentinel.yml` configuration.
* **📊 Real-Time TUI Dashboard:** Color-coded terminal interface displaying active processes categorized by User Apps, Background Services, and System Core with dynamic RAM thresholds.
* **🚨 Mini Defender Engine:** Proactively flags suspicious process keywords, monitors runaway script execution environments such as PowerShell/CMD consuming more than 300 MB, and isolates memory leaks.
* **❄️ Frozen Window Auto-Reaper:** Detects hung applications using native Win32 APIs and automatically terminates them after 15 seconds of unresponsiveness.
* **🧹 Deep Disk & Build Cache Cleaner:** Interactively purges Windows Temp, NPM Global Cache, and system crash dumps with safe file-lock skips.
* **📦 Windows Package Manager Bridge:** Run batch software installs, global system-wide software upgrades, and repository searches directly from your CLI.

---

## 📦 Installation

Sentinel is designed to run globally across all terminal sessions. Install it using the `-g` flag:

```bash
npm install -g win-process-sentinel
```

> **Note:** If typing `daemon` outputs `command not found`, see the [Troubleshooting & PATH Setup](#-troubleshooting--path-setup) section below.

---

## 🚀 Quick Start Guide

### 1. Universal Multi-Target Launcher

Launch multiple apps, databases, and portals simultaneously using comma-separated or space-separated lists:

```bash
# Launch VS Code, MongoDB Compass, RedisInsight, and GitHub in one command
daemon run code, mongo, redis, github

# Force a web portal to open in the browser instead of the desktop app
daemon run notion.web

# Force a desktop application to open via its native protocol
daemon run github.app

# Open custom web URLs directly
daemon run localhost:3000, https://cloud.mongodb.com
```

---

### 2. Automated Project Staging (`sentinel.yml`)

Automate entire development pipelines within your project repository.

#### Step 1: Initialize Configuration

Run the init command in your project root:

```bash
daemon stage init
```

This generates a boilerplate `sentinel.yml` file:

```yaml
project_name: "My-Project-Env"
browser_profile: "Default"

stages:
  dev:
    description: "Boots backend database, dev server, and web portals"

    daemons:
      - "redis-server"

    tasks:
      - "npm run server"
      - "npm run client"

    web_links:
      - "http://localhost:3000"
      - "https://cloud.mongodb.com"
      - "code ."

  build:
    description: "Builds production bundles"

    tasks:
      - "npm run build"

events:
  on_open: "dev"
  on_git_push: "build"
```

#### Step 2: Execute Stages or Trigger Events

```bash
# Run the 'dev' pipeline stage
daemon stage dev

# Manually trigger an event hook
daemon trigger on_git_push
```

---

### 3. Real-Time TUI Dashboard & Monitor

Launch the interactive live terminal dashboard to view real-time system memory metrics, running process trees, and threat detection status:

```bash
daemon
```

#### Background Daemon Mode

You can run Sentinel as a background process that continuously monitors system health and auto-detects active workspace folders:

```bash
# Start Sentinel invisibly in the background
daemon start

# Stop all background Sentinel daemon instances
daemon stop

# Check a fast snapshot of running processes and total RAM load
daemon status
```

---

### 4. Workspace Snapshot & Restoration

Save your entire running workstation state — including open IDEs, browsers, tools, and editors consuming more than 15 MB of RAM — and restore them anytime:

```bash
# Save current desktop state as a snapshot named 'fullstack_session'
daemon snapshot fullstack_session

# Restore and relaunch all applications from that snapshot
daemon restore fullstack_session
```

---

### 5. System Maintenance & Package Management

#### Interactive Disk & Cache Cleaner

Deep clean build artifacts, temp files, and caches with real-time size calculations and folder exclusion prompts:

```bash
daemon clean
```

#### Windows Package Manager (`winget`) Integration

Manage system-level software and development runtimes directly from your terminal:

```bash
# Upgrade all installed software and dependencies on your Windows machine
daemon package update --all

# Install a development tool such as NodeJS, Python, or Git
daemon package install NodeJS.NodeJS

# Search the Microsoft repository
daemon package search rust

# Uninstall a package
daemon package uninstall spotify
```

#### Fuzzy Process Termination

Kill unresponsive or high-memory processes by name or PID:

```bash
# Target by process name
daemon kill chrome

# Target by process ID
daemon kill 14220
```

---

## ⌨️ Global Emergency Panic Hotkey

When Sentinel is running, either in live dashboard mode or background daemon mode, a global emergency keyboard shortcut is registered:

### `Ctrl` + `Alt` + `Shift` + `Esc`

Pressing this key combination immediately force-terminates the application currently focused in the foreground, providing an instant recovery switch when full-screen applications or heavy processes freeze your machine.

---

## 📋 Complete Command Reference

| Subcommand   | Syntax                             | Description                                                             |
| ------------ | ---------------------------------- | ----------------------------------------------------------------------- |
| `run`        | `daemon run <targets>`             | Launches apps, protocols, and URLs concurrently.                        |
| `stage init` | `daemon stage init`                | Generates a boilerplate `sentinel.yml` file.                            |
| `stage`      | `daemon stage <name>`              | Executes a defined workflow stage from `sentinel.yml`.                  |
| `trigger`    | `daemon trigger <event>`           | Triggers a lifecycle event mapped in `sentinel.yml`.                    |
| *(default)*  | `daemon`                           | Launches the live color-coded TUI monitor.                              |
| `start`      | `daemon start`                     | Spawns Sentinel detached in background daemon mode.                     |
| `stop`       | `daemon stop`                      | Stops all running background Sentinel daemon instances.                 |
| `status`     | `daemon status`                    | Prints total managed RAM and top memory-consuming apps.                 |
| `snapshot`   | `daemon snapshot <name>`           | Captures current open applications into a saved profile.                |
| `restore`    | `daemon restore <name>`            | Relaunches all applications saved inside a snapshot.                    |
| `clean`      | `daemon clean`                     | Opens interactive cleaner for temp files and NPM cache.                 |
| `package`    | `daemon package <action> [target]` | Manages software packages (`install`, `update`, `uninstall`, `search`). |
| `inspect`    | `daemon inspect`                   | Queries the Windows registry for installed applications and services.   |
| `kill`       | `daemon kill <name/pid>`           | Fuzzy-kills active processes matching a name query or PID.              |

---

## 🔧 Troubleshooting & PATH Setup

### `daemon` is not recognized as an internal or external command

If running `daemon` returns a `CommandNotFoundException` after installation, your global npm directory may be missing from your Windows `PATH` environment variable.

#### Resolution

1. Press the **Windows Key**, type **Environment Variables**, and select **Edit the system environment variables**.
2. Click the **Environment Variables...** button.
3. Under **User variables**, select the **Path** row and click **Edit**.
4. Click **New** and add:

```text
%USERPROFILE%\AppData\Roaming\npm
```

5. Click **OK** to close all dialogs.
6. Restart your terminal or VS Code session.
7. Run:

```bash
daemon status
```

---

## 📄 License

Distributed under the **MIT License**.

Free and open-source for personal and commercial developer workflows.
