use async_trait::async_trait;
use graph_flow::{Context, NextAction, Task, TaskResult};
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;
use rig::providers::openrouter::CompletionModel;
use tracing::info;
use anyhow::Result;
use crate::rig_tools::market_analysis::MarketAnalysis;

pub fn get_binance_agent() -> Result<Agent<CompletionModel>> {
    let api_key = std::env::var("OPENROUTER_API_KEY")    
        .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY not set"))?;

    let client = rig::providers::openrouter::Client::new(&api_key);

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("")
        .tool(MarketAnalysis)
        .build();

    Ok(agent)
}

pub struct BinanceOperationsTask;

impl BinanceOperationsTask {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BinanceOperationsTask {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Task for BinanceOperationsTask {
    fn id(&self) -> &str {
        std::any::type_name::<Self>()
    }

    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        info!("Starting Binance operations task");

        // TODO: Implement Binance operations logic
        // This task should handle Binance exchange operations and reporting

        Ok(TaskResult::new(
            Some("Binance operations completed".to_string()),
            NextAction::End,
        ))
    }
}
