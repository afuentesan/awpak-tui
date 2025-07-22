---
title: AwpakAI
---

**AwpakAI** is a Rust library for orchestrating the execution of **artificial intelligence agents**, **system commands**, and **HTTP requests**, modeled as a **directed graph**.

Each node in the graph can perform one of the following actions:

* Execute a **system command**.
* Make an **HTTP request** (e.g., to a REST API).
* Invoke an **AI agent** (such as a language model or custom logic).

This structure enables the creation of flexible and powerful pipelines, where each node's output can be passed as input to other nodes, and the execution path can depend on intermediate results.

---

## Use Cases

AwpakAI is ideal for:

* Coordinating language models (LLMs) or AI agents in complex workflows.
* Automating pipelines that combine AI, web services, and shell commands.
* Defining flexible, declarative flows in JSON for easy maintenance and dynamic reconfiguration.

---

* [Graph Builder](/builder)