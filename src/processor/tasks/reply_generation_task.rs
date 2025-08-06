use async_trait::async_trait;
use graph_flow::{Context, NextAction, Task, TaskResult};
use tracing::info;

pub struct ReplyGenerationTask;

impl ReplyGenerationTask {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReplyGenerationTask {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Task for ReplyGenerationTask {
    fn id(&self) -> &str {
        std::any::type_name::<Self>()
    }

    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        info!("Starting portfolio selection task");

        // TODO: Implement portfolio selection logic
        // This task should select optimal portfolio configurations

        Ok(TaskResult::new(
            Some("Portfolio selection completed".to_string()),
            NextAction::End,
        ))
    }
}
