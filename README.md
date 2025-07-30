# Greenrock

A Rust-based workflow execution framework demonstration that showcases AI-powered task orchestration using graph-flow and rig-core.

## Overview

Greenrock is a sample project that demonstrates how to build and execute workflows with AI integration. It features a simple two-task workflow where tasks can communicate through shared context, persist sessions, and integrate with large language models via the OpenRouter API.

## Features

- 🔄 **Workflow Orchestration**: Build and execute task graphs with dependencies
- 🤖 **AI Integration**: Seamless integration with OpenRouter API for LLM-powered tasks  
- 💾 **Session Persistence**: Maintain workflow state across executions
- 🔗 **Task Communication**: Share data between tasks via context
- ⚡ **Async Execution**: Built on Tokio for high-performance async operations
- 🛠️ **Extensible**: Easy to add new tasks and modify workflows

## Prerequisites

- Rust 1.70+ (edition 2024)
- OpenRouter API key

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd greenrock
```

2. Install dependencies:
```bash
cargo build
```

3. Set up environment variables:
```bash
# Create a .env file in the project root
echo "OPENROUTER_API_KEY=your_openrouter_api_key_here" > .env
```

## Usage

### Basic Execution

Run the demonstration workflow:

```bash
cargo run
```

This will execute a simple workflow with two tasks:
1. **HelloTask**: Greets a user (Batman) and generates entertaining content via AI
2. **ExcitementTask**: Adds excitement to the greeting

### Expected Output

```
Starting simple workflow with FlowRunner

Session ID: session_001
Initial task: <hello_task_id>

[AI-generated entertainment content]
Task response: Hello, Batman
Execution status: Paused { next_task_id: "<excitement_task_id>", reason: "Continue" }
Workflow paused, will continue to task: <excitement_task_id> (reason: Continue) – continuing...

Task response: Hello, Batman !!!
Execution status: Completed
Workflow completed successfully!

Final session state:
Session ID: session_001
Current task: <excitement_task_id>
Stored greeting: Hello, Batman

Workflow execution finished
```

## Architecture

### Core Components

- **Tasks**: Individual units of work that implement the `Task` trait
- **Graph**: Defines task dependencies and execution order
- **Context**: Shared data store for communication between tasks
- **Session**: Maintains workflow state and current execution position
- **FlowRunner**: Orchestrates task execution and handles state management

### Task Implementation

```rust
#[async_trait]
impl Task for YourTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Your task logic here
        Ok(TaskResult::new(Some(result), NextAction::Continue))
    }
}
```

### Workflow Creation

```rust
let graph = GraphBuilder::new("your_workflow")
    .add_task(task1)
    .add_task(task2)
    .add_edge(&task1_id, &task2_id)
    .build();
```

## Dependencies

- **graph-flow** (0.2.3): Workflow orchestration framework
- **rig-core** (0.15.1): AI/LLM integration library
- **tokio** (1.47.0): Async runtime
- **async-trait** (0.1.88): Async trait support
- **dotenvy** (0.15.7): Environment variable management

## Configuration

### Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `OPENROUTER_API_KEY` | Your OpenRouter API key for AI model access | Yes |

### Model Configuration

The project currently uses `deepseek/deepseek-chat-v3-0324:free` via OpenRouter. You can modify this in the `HelloTask` implementation:

```rust
let comedian_agent = client
    .agent("your-preferred-model")
    .preamble("Your custom prompt")
    .build();
```

## Extending the Project

### Adding New Tasks

1. Create a struct implementing the `Task` trait
2. Add async task logic in the `run` method
3. Register the task with your graph builder
4. Connect it to other tasks via edges

### Custom Workflows

```rust
let custom_graph = GraphBuilder::new("custom_workflow")
    .add_task(Arc::new(YourCustomTask))
    .add_task(Arc::new(AnotherTask))
    .add_edge(&task1_id, &task2_id)
    .build();
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source. Please check the LICENSE file for details.

## Support

For questions, issues, or contributions, please open an issue on the project repository.

---

*Built with ❤️ using Rust, graph-flow, and rig-core*
