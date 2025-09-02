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
        info!("Starting reply generation task");

        let answer = context.get_sync::<String>("answer").unwrap();

        info!("Answer: {}", answer);

        Ok(TaskResult::new(
            Some("Reply generation completed".to_string()),
            NextAction::End,
        ))
    }
}
