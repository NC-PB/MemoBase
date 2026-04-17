# MemoBase

MemoBase is a modern, responsive contact management desktop application built with [Tauri](https://tauri.app/), [SvelteKit](https://kit.svelte.dev/), and [Rust](https://www.rust-lang.org/). It provides a streamlined interface for managing company profiles, contact personnel, and tracking interactions, backed by a fast and reliable SQLite database.

## Features

- **Company Management**: Full CRUD operations for managing company profiles, including clickable website and social media integrations.
- **Contact Persons**: Keep track of individual contacts associated with companies.
- **Interaction Logs**: Record and view a history of interactions with your contacts.
- **Modern Dashboard**: Features a responsive UI built with Tailwind CSS, including a collapsible sidebar and custom branding elements.
- **Local First**: Powered by Tauri and an embedded SQLite database, ensuring your data is kept secure and local.

## Tech Stack

- **Frontend**: SvelteKit, TypeScript, Tailwind CSS
- **Backend**: Rust, Desktop Environment via Tauri
- **Database**: SQLite

## Recommended IDE Setup

We recommend using [VS Code](https://code.visualstudio.com/) for development, along with the following extensions:
- [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)

## Getting Started

### Prerequisites
Before you begin, ensure you have the required dependencies for Tauri installed:
- [Node.js](https://nodejs.org/en/) (LTS recommended)
- [Rust](https://www.rust-lang.org/tools/install)
- System requirements as per the [Tauri Prerequisites Guide](https://tauri.app/v1/guides/getting-started/prerequisites) (e.g., C++ build tools on Windows).

### Installation

1. Navigate to the project root directory:
   ```bash
   cd MemoBase
   ```

2. Install Node dependencies:
   ```bash
   npm install
   ```

### Development

To start the development server and automatically open the Tauri desktop app window:
```bash
npm run tauri dev
```

### Build for Production

To build the executable application installer for your operating system:
```bash
npm run tauri build
```
The compiled binaries and installers will fall under `src-tauri/target/release/bundle/`.
