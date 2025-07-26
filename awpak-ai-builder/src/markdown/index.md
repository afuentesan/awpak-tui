**AwpakAI** lets you create and run powerful workflows modeled as **directed graphs**.
These graphs can orchestrate **artificial intelligence agents**, **system commands**, and **HTTP requests**.

You don’t need to be a programmer to use it:

* **Developers** can integrate <a href="https://github.com/afuentesan/awpak-tui/tree/main/awpak-ai" target="_blank" title="Github of awpak-ai rust lib">AwpakAI</a> as a Rust library for full control and advanced customization.
* **Non‑programmers** can create and execute graphs directly using the standalone command‑line tool <a href="https://github.com/afuentesan/awpak-tui/tree/main/awpak-ai-cmd-client" target="_blank" title="Github of awpak-ai-cmd-client command-line tool">awpak-ai-cmd-client</a>, without writing a single line of code.



## How it works
Each node in a graph can:

* Execute a **system command**
* Make an **HTTP request** (e.g., to a REST API)
* Invoke an **AI agent** (such as a language model or custom logic)

Graphs have an internal **context** where the output of every executed node is stored. Subsequent nodes can access outputs from previously executed nodes, enabling flexible and dynamic data flows.

## Use Cases
AwpakAI is ideal for:

* Coordinating language models (LLMs) or AI agents in complex workflows
* Automating pipelines combining AI, web services, and shell commands
* Defining flexible, declarative flows in JSON for easy maintenance and dynamic reconfiguration

## Creating Graphs Easily
Writing JSON graph definitions manually can be complex due to the available options.
To make this easier, we provide a <a href="/builder" target="_blank" title="Open graph builder in new tab">web-based editor</a> to build graphs interactively and export them as JSON.

The <a href="/docs/getting-started" title="Getting started with AwpakAI">Getting Started</a> guide shows how to create and customize a graph using the web editor, and how to run it either from your Rust project or directly with `awpak-ai-cmd-client`.