use async_trait::async_trait;
use graph_flow::{Context, NextAction, Task, TaskResult};
use tracing::info;

pub struct RegimenAggregationTask;

impl RegimenAggregationTask {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RegimenAggregationTask {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Task for RegimenAggregationTask {
    fn id(&self) -> &str {
        std::any::type_name::<Self>()
    }

    async fn run(&self, _context: Context) -> graph_flow::Result<TaskResult> {
        info!("Starting regimen aggregation task");

        // TODO: Implement regimen aggregation logic
        // This task should aggregate data from multiple trading regimens

        Ok(TaskResult::new(
            Some("Regimen aggregation completed".to_string()),
            NextAction::End,
        ))
    }
}
