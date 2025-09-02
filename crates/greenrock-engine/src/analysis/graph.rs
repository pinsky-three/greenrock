use std::sync::Arc;

use graph_flow::{GraphBuilder, GraphStorage, Task};

use crate::processor::tasks::{
    binance_operations_task::BinanceOperationsTask, binance_reporting_task::BinanceReportingTask,
    entry_interaction_task::EntryInteractionTask,
    portfolio_aggregation_task::PortfolioAggregationTask,
    portfolio_reporting_task::PortfolioReportingTask,
    portfolio_selection_task::PortfolioSelectionTask,
    regimen_aggregation_task::RegimenAggregationTask,
    regimen_evaluation_task::RegimenEvaluationTask, regimen_reporting_task::RegimenReportingTask,
    regimen_selection_task::RegimenSelectionTask, regimen_switching_task::RegimenSwitchingTask,
    reply_generation_task::ReplyGenerationTask,
};

use tracing::info;

pub async fn setup_graph(
    graph_storage: Arc<dyn GraphStorage>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up recommendation workflow graph");

    let entry_interaction_task: Arc<dyn Task> = Arc::new(EntryInteractionTask::new("".to_string()));

    let analysis_market_task: Arc<dyn Task> = Arc::new(RegimenReportingTask);
    let analysis_social_media_task: Arc<dyn Task> = Arc::new(RegimenEvaluationTask);
    let analysis_news_task: Arc<dyn Task> = Arc::new(RegimenSwitchingTask);
    let analysis_blockchain_task: Arc<dyn Task> = Arc::new(RegimenAggregationTask);
    let regimen_evaluation_task: Arc<dyn Task> = Arc::new(RegimenSelectionTask);
    let portfolio_selection_task: Arc<dyn Task> = Arc::new(PortfolioSelectionTask);
    let reply_generation_task: Arc<dyn Task> = Arc::new(ReplyGenerationTask);




    let entry_interaction_task_id = entry_interaction_task.id().to_string();
    let analysis_market_task_id = analysis_market_task.id().to_string();
    let analysis_social_media_task_id = analysis_social_media_task.id().to_string();
    let analysis_news_task_id = analysis_news_task.id().to_string();
    let analysis_blockchain_task_id = analysis_blockchain_task.id().to_string();
    let regimen_evaluation_task_id = regimen_evaluation_task.id().to_string();
    let portfolio_selection_task_id = portfolio_selection_task.id().to_string();
    let reply_generation_task_id = reply_generation_task.id().to_string();

    // Build graph
    let graph = Arc::new(
        GraphBuilder::new("greenrock_main_flow")
            .add_task(entry_interaction_task)
            
            .add_task(analysis_market_task)
            .add_task(analysis_social_media_task)
            .add_task(analysis_news_task)
            .add_task(analysis_blockchain_task)

            .add_task(regimen_evaluation_task)
            .add_task(portfolio_selection_task)
            .add_edge(
                entry_interaction_task_id.clone(),
                analysis_market_task_id.clone()
            )
            .add_edge(
                analysis_market_task_id.clone(),
                analysis_social_media_task_id.clone()
            )
            .add_edge(
                analysis_social_media_task_id.clone(),
                analysis_news_task_id.clone()
            )
            .add_edge(
                analysis_news_task_id.clone(),
                analysis_blockchain_task_id.clone(),
            )
            .add_edge(
                analysis_news_task_id.clone(),
                regimen_evaluation_task_id.clone()
            )
            .add_edge(
                regimen_evaluation_task_id.clone(),
                 portfolio_selection_task_id.clone()
            )
            .add_edge(
                portfolio_selection_task_id.clone(),
                reply_generation_task_id.clone()
            )
            .build(),
    );

    graph_storage.save("".to_string(), graph).await?;

    info!("Graph built and saved successfully");
    Ok(())
}