# awpak-ai-cmd-client

`awpak-ai-cmd-client` is a command-line client for executing AI workflows defined as graphs using the [awpak-ai](https://awpak.com) library.  
This tool allows anyone, even without programming experience, to create and run AI-driven workflows.

---

## Features

- Execute workflows (graphs) defined in JSON.
- Interactive chat mode with LLMs.
- Detailed trace options to control execution output.
- Support for streaming agent responses.

---

## Installation

You can install it from source using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install awpak-ai-cmd-client
````

---

## Usage

```bash
awpak-ai-cmd-client --path="/path_to_graph.json" [OPTIONS]
```

### Required Arguments

* `--path <FILE>`
  Path to the JSON file containing the graph definition.

### Optional Arguments

* `--input <INPUT>`
  Graph input value. Ignored when using `--chat`.

* `--trace <TRACE_OPTIONS>`
  Comma-separated trace options to control output visibility.
  **Default**: `graph_output_ok,graph_output_err`.

  Available trace options:

  * `graph_input` → Prints the graph input
  * `graph_output_ok` → Prints the graph output if execution succeeds
  * `graph_output_err` → Prints the graph output if execution fails
  * `agent_prompt` → Prints the prompt for Agent nodes
  * `agent_stream` → Shows streaming output from Agent nodes (if enabled)
  * `agent_sync` → Shows synchronous output from Agent nodes
  * `agent_tool_call` → Shows MCP tool calls made by Agent nodes
  * `agent_tool_result` → Shows the result of MCP tool calls
  * `command_and_args` → Shows the command and arguments for Command nodes
  * `command_result` → Shows the result of Command nodes
  * `web_client_request` → Shows the URL and method for WebClient nodes
  * `web_client_request_body` → Shows the body of WebClient requests
  * `web_client_request_headers` → Shows headers of WebClient requests
  * `web_client_request_query_params` → Shows query parameters of WebClient requests
  * `web_client_response` → Shows version and status code of WebClient responses
  * `web_client_response_headers` → Shows headers of WebClient responses
  * `web_client_response_body` → Shows body of WebClient responses
  * `node_destination` → Shows each node executed and the chosen destination
  * `node_execution` → Shows node IDs before execution
  * `node_output` → Shows output of each node

* `--chat`
  Start interactive chat mode. Input is read from a prompt instead of `--input`.

---

## Example

### Running a Graph with Static Input

```bash
awpak-ai-cmd-client --path="./graph.json" --input="Hello world" --trace="agent_stream"
```

### Running in Chat Mode

```bash
awpak-ai-cmd-client --path="./graph.json" --trace="agent_stream" --chat
```

This starts an interactive shell where each message you type is sent as graph input.

---

## Related Projects

* [awpak-ai](https://crates.io/crates/awpak-ai): Core Rust library for building and executing AI workflows.
* [awpak-ai-graph-builder](https://awpak.com/builder): Web-based tool to easily create workflow graphs.

---

## License

This project is licensed under the [MIT license](LICENSE).

---
