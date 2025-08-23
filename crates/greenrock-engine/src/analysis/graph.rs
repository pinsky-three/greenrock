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
    info!("Setting up greenrock workflow graph");

    let entry_interaction_task: Arc<dyn Task> = Arc::new(EntryInteractionTask::new("".to_string()));

    let regimen_reporting_task: Arc<dyn Task> = Arc::new(RegimenReportingTask);
    let regimen_evaluation_task: Arc<dyn Task> = Arc::new(RegimenEvaluationTask);
    let regimen_switching_task: Arc<dyn Task> = Arc::new(RegimenSwitchingTask);
    let regimen_aggregation_task: Arc<dyn Task> = Arc::new(RegimenAggregationTask);
    let regimen_selection_task: Arc<dyn Task> = Arc::new(RegimenSelectionTask);

    let binance_reporting_task: Arc<dyn Task> = Arc::new(BinanceReportingTask);
    let binance_operations_task: Arc<dyn Task> = Arc::new(BinanceOperationsTask);

    let portfolio_reporting_task: Arc<dyn Task> = Arc::new(PortfolioReportingTask);
    let portfolio_aggregation_task: Arc<dyn Task> = Arc::new(PortfolioAggregationTask);
    let portfolio_selection_task: Arc<dyn Task> = Arc::new(PortfolioSelectionTask);

    let reply_generation_task: Arc<dyn Task> = Arc::new(ReplyGenerationTask);

    //

    let entry_interaction_task_id = entry_interaction_task.id().to_string();
    let reply_generation_task_id = reply_generation_task.id().to_string();

    let regimen_reporting_task_id = regimen_reporting_task.id().to_string();
    let regimen_evaluation_task_id = regimen_evaluation_task.id().to_string();
    let regimen_switching_task_id = regimen_switching_task.id().to_string();
    let regimen_aggregation_task_id = regimen_aggregation_task.id().to_string();
    let regimen_selection_task_id = regimen_selection_task.id().to_string();

    let binance_reporting_task_id = binance_reporting_task.id().to_string();
    let binance_operations_task_id = binance_operations_task.id().to_string();

    let portfolio_reporting_task_id = portfolio_reporting_task.id().to_string();
    let portfolio_aggregation_task_id = portfolio_aggregation_task.id().to_string();
    let portfolio_selection_task_id = portfolio_selection_task.id().to_string();

    // Build graph
    let graph = Arc::new(
        GraphBuilder::new("greenrock_main_flow")
            .add_task(entry_interaction_task)
            //
            .add_task(reply_generation_task)
            //
            .add_task(regimen_reporting_task)
            .add_task(regimen_evaluation_task)
            .add_task(regimen_switching_task)
            .add_task(regimen_aggregation_task)
            .add_task(regimen_selection_task)
            //
            .add_task(binance_reporting_task)
            .add_task(binance_operations_task)
            //
            .add_task(portfolio_reporting_task)
            .add_task(portfolio_aggregation_task)
            //
            .add_task(portfolio_selection_task)
            //
            .add_conditional_edge(
                entry_interaction_task_id.clone(),
                {
                    let reply_generation_task_id = reply_generation_task_id.clone();
                    let regimen_reporting_task_id = regimen_reporting_task_id.clone();

                    move |ctx| {
                        (ctx.get_sync::<String>("next_task")
                            .unwrap_or(reply_generation_task_id.clone()))
                            == regimen_reporting_task_id
                    }
                },
                regimen_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_conditional_edge(
                entry_interaction_task_id.clone(),
                {
                    let reply_generation_task_id = reply_generation_task_id.clone();
                    let binance_reporting_task_id = binance_reporting_task_id.clone();

                    move |ctx| {
                        (ctx.get_sync::<String>("next_task")
                            .unwrap_or(reply_generation_task_id.clone()))
                            == binance_reporting_task_id
                    }
                },
                binance_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_conditional_edge(
                entry_interaction_task_id.clone(),
                {
                    let reply_generation_task_id = reply_generation_task_id.clone();
                    let portfolio_reporting_task_id = portfolio_reporting_task_id.clone();

                    move |ctx| {
                        (ctx.get_sync::<String>("next_task")
                            .unwrap_or(reply_generation_task_id.clone()))
                            == portfolio_reporting_task_id
                    }
                },
                portfolio_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_edge(
                regimen_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_edge(
                binance_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_edge(
                portfolio_reporting_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_edge(
                regimen_selection_task_id.clone(),
                reply_generation_task_id.clone(),
            )
            .add_edge(
                binance_operations_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                portfolio_selection_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                portfolio_aggregation_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                regimen_aggregation_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                regimen_switching_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                regimen_evaluation_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .add_edge(
                regimen_reporting_task_id.clone(),
                regimen_selection_task_id.clone(),
            )
            .build(),
    );

    graph_storage.save("".to_string(), graph).await?;

    info!("Graph built and saved successfully");
    Ok(())
}
