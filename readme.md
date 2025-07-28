// readme.md
# Nexus Copilot

Nexus Copilot is a local-first AI-native application designed to augment your workflow and knowledge management. It combines a powerful local knowledge base, dynamic tool execution, and seamless AI integration to create a truly intelligent assistant that runs on your machine.

## ✨ Features

-   **Local-First Knowledge Base:** Index your local files (Markdown, PDF, DOCX, code, etc.) and perform semantic searches.
-   **AI-Powered Chat:** Engage in conversations with state-of-the-art language models, augmented with your local knowledge or real-time web search results.
-   **Dynamic Tool System:** Create, configure, and execute custom Python scripts or shell commands directly from the UI. Tools can be enhanced with AI pre-processing (natural language to command) and post-processing (raw output to summary).
-   **Intelligent Copilot Window:** A floating, always-on-top window that provides context-aware suggestions based on your clipboard content.
-   **Clipboard History:** A powerful clipboard manager that remembers text and images.
-   **Cross-Platform:** Built with Tauri, running on macOS, Windows, and Linux.
-   **Customizable:** Configure everything from AI models, global shortcuts, to appearance and execution environments.
-   **Backup & Restore:** Easily export and import all your application data.

## 🚀 Getting Started & Development

This guide will walk you through setting up the development environment for both the frontend (Tauri + Vue) and the backend (Python FastAPI).

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Node.js](https://nodejs.org/) (v18+ recommended)
-   [Python](https://www.python.org/downloads/) (v3.9+ recommended)
-   OS-specific dependencies for Tauri (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

### 1. Clone the Repositories

First, clone the main application repository and the backend service repository. It's recommended to place them in the same parent directory.

```bash
# Clone the main Tauri application
git clone https://github.com/zazaji/nexus_copilot.git
cd nexus_copilot

# Clone the FastAPI backend service
git clone https://github.com/zazaji/nexus_copilot_fastapi.git backend_service
```

### 2. Frontend Setup (Tauri + Vue)

The frontend is a standard Vue 3 application managed by `npm`.

```bash
# Navigate to the frontend directory
cd frontend

# Install dependencies
npm install

# Run the development server
npm run dev
```

This will start the Vue development server. In a separate terminal, run the Tauri application in development mode:

```bash
# From the root of the nexus_copilot project
cargo tauri dev
```

### 3. Backend Setup (Python FastAPI)

The backend provides vector database services and an LLM proxy.

```bash
# Navigate to the backend service directory
cd backend_service

# Create and activate a virtual environment (recommended)
python -m venv venv
source venv/bin/activate  # On Windows, use `venv\Scripts\activate`

# Install Python dependencies
pip install -r requirements.txt

# Run the backend server
uvicorn app.main:app --host 127.0.0.1 --port 8008 --reload
```

Now, both the frontend and backend are running in development mode.

## 📦 Building & Cross-Platform Compilation

Tauri makes cross-compilation straightforward, but it requires setting up the correct toolchains and linkers.

### General Setup

Ensure you have the necessary Rust targets installed. For example, to build for Windows from macOS:
`rustup target add x86_64-pc-windows-msvc`

### On macOS

#### Building for macOS (Native)

```bash
cargo tauri build
```

#### Cross-compiling for Windows (x86_64)

1.  **Install Toolchain:**
    ```bash
    rustup target add x86_64-pc-windows-msvc
    ```
2.  **Install Linker:** `osxcross` is a popular choice.
    ```bash
    brew install osxcross
    ```
3.  **Configure Cargo:** Create or edit `~/.cargo/config.toml` and add:
    ```toml
    [target.x86_64-pc-windows-msvc]
    linker = "x86_64-w64-mingw32-gcc"
    ar = "x86_64-w64-mingw32-ar"
    ```
4.  **Build:**
    ```bash
    cargo tauri build --target x86_64-pc-windows-msvc
    ```

#### Cross-compiling for Linux (x86_64 & aarch64)

1.  **Install Toolchains:**
    ```bash
    rustup target add x86_64-unknown-linux-gnu
    rustup target add aarch64-unknown-linux-gnu
    ```
2.  **Install Linkers:**
    ```bash
    brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu
    brew install messense/macos-cross-toolchains/aarch64-unknown-linux-gnu
    ```
3.  **Build for x86_64:**
    ```bash
    CC_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-gcc" \
    CXX_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-g++" \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-unknown-linux-gnu-gcc" \
    cargo tauri build --target x86_64-unknown-linux-gnu
    ```
4.  **Build for aarch64 (Linux ARM):**
    ```bash
    CC_aarch64_unknown_linux_gnu="aarch64-unknown-linux-gnu-gcc" \
    CXX_aarch64_unknown_linux_gnu="aarch64-unknown-linux-gnu-g++" \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-unknown-linux-gnu-gcc" \
    cargo tauri build --target aarch64-unknown-linux-gnu
    ```

### On Windows

#### Building for Windows (Native)

Ensure you have the "Desktop development with C++" workload installed via the Visual Studio Installer.

```powershell
cargo tauri build
```

### On Linux

#### Building for Linux (Native)

Ensure you have the necessary dependencies installed (e.g., `build-essential`, `libwebkit2gtk-4.0-dev`, `librsvg2-dev`).

```bash
cargo tauri build
```

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).