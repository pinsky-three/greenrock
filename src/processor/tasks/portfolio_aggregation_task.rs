use async_trait::async_trait;
use graph_flow::{Context, NextAction, Task, TaskResult};
use tracing::info;

pub struct PortfolioAggregationTask;

impl PortfolioAggregationTask {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PortfolioAggregationTask {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Task for PortfolioAggregationTask {
    fn id(&self) -> &str {
        std::any::type_name::<Self>()
    }

    async fn run(&self, _context: Context) -> graph_flow::Result<TaskResult> {
        info!("Starting portfolio aggregation task");

        // TODO: Implement portfolio aggregation logic
        // This task should aggregate portfolio data from multiple sources

        Ok(TaskResult::new(
            Some("Portfolio aggregation completed".to_string()),
            NextAction::End,
        ))
    }
}
