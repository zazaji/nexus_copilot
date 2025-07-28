

### `README.md` (Root Directory - English Version)

<div align="center">
  <img src="https://raw.githubusercontent.com/zazaji/nexus_copilot/main/src-tauri/icons/128x128@2x.png" alt="Nexus Copilot Logo" width="128">
  <h1>Nexus Copilot</h1>
  <p><strong>A Local-First, Intent-Driven AI-Native Desktop Workflow System</strong></p>
  <p>
    <a href="https://github.com/zazaji/nexus_copilot/releases"><img src="https://img.shields.io/github/v/release/zazaji/nexus_copilot?display_name=tag&sort=semver" alt="Latest Release"></a>
    <a href="https://github.com/zazaji/nexus_copilot/blob/main/LICENSE"><img src="https://img.shields.io/github/license/zazaji/nexus_copilot" alt="License"></a>
    <a href="https://github.com/zazaji/nexus_copilot/actions/workflows/release.yml"><img src="https://github.com/zazaji/nexus_copilot/actions/workflows/release.yml/badge.svg" alt="Build Status"></a>
  </p>
</div>

**Nexus Copilot** is designed to redefine your interaction with AI. It's not just another chat application; it's an intelligent companion deeply integrated into your operating system, seamlessly blending into your daily workflow. Through a smart floating window and a powerful local knowledge base, Nexus brings the full power of AI to your fingertips, wherever you are.

[**Check out the Latest Release**](https://github.com/zazaji/nexus_copilot/releases)

---

## Core Features



#### 🤖 **Intent-Driven Copilot Window**
-   **System-Level Context Awareness**: Automatically captures your clipboard content (text, images, files) and understands your potential intent.
-   **Dynamic Action Suggestions**: No typing needed. Copilot intelligently suggests one-click actions like "Translate," "Explain Code," or "Summarize Content" based on the context.
-   **Global Shortcut**: Use `Cmd/Ctrl+Shift+C` to summon the Copilot instantly from any application. It automatically hides when you're done, never interrupting your flow.
-   **Powerful Clipboard History**: Your "digital memory" with full-text search, pinning, and one-click paste capabilities.

#### 🧠 **Agentic AI Task Automation**
-   **Multi-Mode Agent Engine**: Go beyond simple Q&A. Issue complex goals with commands like `/plan`, `/explore`, `/write`, `/research`, or `/debate`, and the AI Agent will autonomously plan and execute multi-step tasks.
-   **Transparent "Chain of Thought"**: Watch the Agent's every thought, action, and decision in real-time. No more black boxes.
-   **Long-Form Content Generation**: Specialized `write` and `research` modes are optimized for creating structured, coherent long-form articles and reports.

<p align="center">
  <img src="https://raw.githubusercontent.com/zazaji/nexus_copilot/main/docs/knowledge-base-demo.png" alt="Nexus Knowledge Base" width="700">
  <br>
  <em>Build a "second brain" that's truly yours, powered by AI.</em>
</p>

#### 📚 **Local-First Hybrid Knowledge Base**
-   **Your Data, Your Control**: All your notes, files, and indexes are stored securely on your local device, enabling full offline access.
-   **Hybrid Indexing**: Combines **metadata indexing (SQLite)** for precise lookups with **semantic vector indexing (ChromaDB)** for fuzzy, meaning-based exploration.
-   **AI-Powered Semantic Search**: Ask questions in natural language and find the most relevant information across all your documents (PDF, DOCX, MD...).
-   **Dynamic Knowledge Graph**: Use `[[WikiLinks]]` to connect your ideas into a network, and explore these connections visually in an interactive 3D graph.
-   **Retrieval-Augmented Generation (RAG)**: Enable your AI chat models to cite and summarize content directly from your personal knowledge base, providing highly accurate and personalized answers.

#### 🛠️ **High Performance & Extensibility**
-   **Lightweight & Efficient**: Built with **Tauri** and **Rust**, ensuring minimal memory and CPU usage, along with lightning-fast startup times.
-   **Cross-Platform**: Runs beautifully on Windows, macOS, and Linux.
-   **Flexible API Configuration**: Supports connecting to any OpenAI-compatible model service, including locally hosted models.

## Why Nexus Copilot?

| Feature | Nexus Copilot | Traditional Web AI Tools (ChatGPT, Claude) | Other Local AI Apps |
| :--- | :---: | :---: | :---: |
| **System-Level Integration** | ✅ **Yes** (Global shortcut, clipboard monitor) | ❌ No (Requires browser/app switching) | ⚠️ Limited |
| **Local-First & Offline** | ✅ **Yes** | ❌ No | ✅ Yes |
| **Agentic Task Automation** | ✅ **Yes** (Multi-mode Agents) | ⚠️ Limited (Via plugins only) | ❌ No |
| **Hybrid Knowledge Base (Semantic + Graph)** | ✅ **Yes** | ❌ No | ⚠️ Limited (Usually semantic only) |
| **Performance & Resource Usage** | 🟢 **Excellent** (Rust core) | 🟡 Medium (Browser-based) | 🟡 Medium (Often Electron-based) |

## Getting Started

1.  **Download**: Go to the [**Releases**](https://github.com/zazaji/nexus_copilot/releases) page and download the latest version for your operating system.
2.  **Install**:
    -   **Windows**: Run the `.msi` installer.
    -   **macOS**: Open the `.dmg` file and drag `NexusCopilot.app` to your "Applications" folder.
    -   **Linux**: Use the `.deb` or `.AppImage` file.
3.  **Initial Setup**:
    -   On first launch, navigate to **Settings -> API & Models**.
    -   Add your API Provider details (e.g., OpenAI API key and Base URL).
    -   In the **Model Assignments** section, select the models you just configured for features like "Chat," "Suggestion," etc.
    -   You're all set! Start using Nexus via the system tray icon or the global shortcut.

## Development Setup

Interested in contributing to the project? That's awesome!

#### **Prerequisites**
-   [Node.js](https://nodejs.org/) v20+
-   [Rust](https://www.rust-lang.org/tools/install) & Cargo
-   [Python](https://www.python.org/downloads/) 3.10+

#### **Installation & Running**
1.  **Clone the repository**:
    ```bash
    git clone https://github.com/zazaji/nexus_copilot.git
    cd nexus_copilot
    ```

2.  **Install frontend dependencies**:
    ```bash
    npm install
    ```

3.  **Install backend dependencies**:
    ```bash
    cd backend
    python -m venv venv
    # Activate virtual environment (macOS/Linux)
    source venv/bin/activate
    # Activate virtual environment (Windows)
    # venv\Scripts\activate
    pip install -r requirements.txt
    cd ..
    ```

4.  **Run the development environment**:
    This will start the Vite frontend dev server, the Tauri desktop app, and the FastAPI backend service concurrently.
    ```bash
    npm run tauri dev
    ```

## Architecture Overview

Nexus Copilot uses a local three-tier architecture to achieve the best balance of performance, functionality, and security:
1.  **Frontend (Vue 3 + TypeScript)**: Renders the user interface and manages UI state.
2.  **Tauri Core (Rust)**: Acts as the bridge between the frontend and the OS, handling high-performance, low-latency tasks like window management, global shortcuts, filesystem I/O, and the SQLite database.
3.  **AI Backend (FastAPI + Python)**: Runs as a separate local service, responsible for all compute-intensive tasks, including LLM API proxying, Agent logic execution, vector database management, and RAG processing.



## License

This project is licensed under the [MIT License](LICENSE).