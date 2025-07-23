# `awpak-ai`

**`awpak-ai`** is a Rust library for orchestrating the execution of **artificial intelligence agents**, **system commands**, and **HTTP requests**, modeled as a **directed graph**.

Each node in the graph can perform one of the following actions:

* Execute a **system command**.
* Make an **HTTP request** (e.g., to a REST API).
* Invoke an **AI agent** (such as a language model or custom logic).

This structure enables the creation of flexible and powerful pipelines. Each graph has an internal **context**, where the output of every executed node is stored. Subsequent nodes can access the outputs of one or more previously executed nodes from this context, enabling flexible and dynamic data flow throughout the execution.

---

## Building a Graph

There are two main methods for creating a graph from a JSON definition:

```rust
pub fn graph_from_json_file_path(path: impl AsRef<str>) -> Result<Graph, Error>
```

Builds a graph from a JSON file located at the given file path.

```rust
pub fn graph_from_json_str(json: impl AsRef<str>) -> Result<Graph, Error>
```

Builds a graph from a JSON string provided directly in memory.

Both methods return a `Graph` instance ready to be executed.

---

## Running the Graph

To execute a graph asynchronously, use the following method:

```rust
pub async fn run_graph(input: String, graph: Graph) -> AwpakResult<Graph, Error>
```

* `input`: a string input passed to the graph at the start of execution.
* `graph`: the graph instance previously built using one of the `graph_from_*` methods.

The result is the updated `Graph` after execution, along with any errors encountered during the process.

---

## Example Usage

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the graph from a JSON file
    let graph = awpak_ai::graph_from_json_file_path("pipeline.json")?;

    // Run the graph with an input string
    let result = awpak_ai::run_graph("initial input".to_string(), graph).await;

    // Collect the final graph and check for errors
    let graph = match result.collect() {
        (g, None) => {
            println!("Output: {:?}", g.final_output);
            g
        },
        (g, Some(e)) => {
            println!("Error: {}", e);
            g
        }
    };

    Ok(())
}
```

This pattern ensures that:

* You always retain access to the final state of the graph, even if an error occurred during execution.
* You can inspect the final output if successful (`graph.final_output`).
* You can log or handle the error if one occurred, without losing the graph context.

---

## Use Cases

`awpak-ai` is ideal for:

* Coordinating language models (LLMs) or AI agents in complex workflows.
* Automating pipelines that combine AI, web services, and shell commands.
* Defining flexible, declarative flows in JSON for easy maintenance and dynamic reconfiguration.

## Creating the JSON Definition

Manually writing a JSON graph definition can be complex because of the number of available options.
To simplify this process, we provide a **web-based editor** that helps you build graphs interactively and export them as JSON.

➡ **[Getting Started with Graph JSON](http://awpak.com/docs/getting-started)**

The `Getting Started` guide will walk you through how to generate and customize a graph using the web editor and how to integrate it into your Rust project.