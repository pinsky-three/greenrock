use std::{env, sync::Arc};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use graph_flow::{
    Context, ExecutionStatus, FlowRunner, GraphBuilder, GraphStorage, InMemoryGraphStorage,
    PostgresSessionStorage, Session, SessionStorage, Task,
};
use greenrock::processor::tasks::{
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
// use polars::prelude::{IntoLazy, col};
use serde::{Deserialize, Serialize};
use tracing::{Level, error, info};
use uuid::Uuid;

async fn health_check() -> &'static str {
    "OK"
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    query: String,
    session_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    session_id: String,
    answer: String,
    status: String,
}

fn internal_error(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

async fn chat(State(state): State<AppState>, Json(params): Json<ChatRequest>) -> Response {
    info!("Received recommendation request: {}", params.query);

    let session_id = if params.session_id.is_some()
        && let Ok(Some(session)) = state.session_storage.get(&params.session_id.unwrap()).await
    {
        info!("Session found: {}", session.id);
        session.id
    } else {
        let new_session_id = Uuid::new_v4().to_string();

        info!(
            "Session not found, creating new session: {}",
            new_session_id
        );

        new_session_id
    };

    let reply_task_id = std::any::type_name::<EntryInteractionTask>();

    // Set up context with chat history limit
    let context = Context::with_max_chat_messages(50);

    context.set("user_input", params.query.clone()).await;
    context.set("session_id", session_id.clone()).await;
    context.set("retry_count", 0).await;

    let session = Session {
        id: session_id.clone(),
        graph_id: "default".to_string(),
        current_task_id: reply_task_id.to_string(),
        status_message: None,
        context,
    };

    if let Err(e) = state.session_storage.save(session).await {
        error!("Failed to save session: {}", e);
        return internal_error("Failed to save session");
    }

    info!("Session created with ID: {}", session_id);

    let execution = match state.flow_runner.run(&session_id).await {
        Ok(execution) => execution,
        Err(e) => {
            error!("Failed to execute session: {}", e);
            return internal_error(&format!("Workflow execution failed: {e}"));
        }
    };

    // Handle execution result
    match execution.status {
        ExecutionStatus::Completed => {
            info!("Workflow completed successfully");

            let final_answer = execution
                .response
                .unwrap_or_else(|| "No answer generated".to_string());

            Json(ChatResponse {
                session_id,
                answer: final_answer,
                status: "completed".to_string(),
            })
            .into_response()
        }
        ExecutionStatus::Paused {
            next_task_id,
            reason,
        } => {
            info!(
                "Workflow unexpectedly paused at task: {} (reason: {})",
                next_task_id, reason
            );
            internal_error("Workflow is paused, which is not expected in this flow")
        }
        ExecutionStatus::WaitingForInput => {
            info!("Workflow unexpectedly waiting for input");
            internal_error("Workflow is waiting for input, which is not expected in this flow")
        }
        ExecutionStatus::Error(e) => {
            error!("Workflow error: {}", e);
            internal_error(&format!("Workflow failed: {e}"))
        }
    }
}

#[derive(Clone)]
struct AppState {
    flow_runner: Arc<FlowRunner>,
    session_storage: Arc<dyn SessionStorage>,
}

async fn setup_graph(
    graph_storage: Arc<dyn GraphStorage>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up recommendation workflow graph");

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
            .build(),
    );

    graph_storage.save("".to_string(), graph).await?;

    info!("Graph built and saved successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("Starting greenrock chat service");

    // let df = load_btc_data("processed_btc_data/2025-01.parquet");

    // let mut strategy = MinimalStrategy::new(df.clone());

    // let df = df
    //     .clone()
    //     .lazy()
    //     .select([
    //         col("timestamp"),
    //         col("open"),
    //         col("high"),
    //         col("low"),
    //         col("close"),
    //         col("volume"),
    //     ])
    //     .collect()
    //     .unwrap();

    // let start = Instant::now();

    // for i in 0..df.shape().0 {
    //     let tick = row_to_kline(&df, i);
    //     strategy.state = strategy.tick(&mut strategy.state.clone(), Some(tick));
    // }

    // println!(
    //     "evaluated {} klines in {:.3}s",
    //     df.shape().0,
    //     start.elapsed().as_secs_f64()
    // );

    // let bot = Bot::from_env();

    // Command::repl(bot, answer).await;

    // Setup storage
    let database_url =
        env::var("DATABASE_URL").map_err(|_| "DATABASE_URL environment variable not set")?;

    let session_storage: Arc<dyn SessionStorage> =
        Arc::new(PostgresSessionStorage::connect(&database_url).await?);

    let graph_storage: Arc<dyn GraphStorage> = Arc::new(InMemoryGraphStorage::new());

    setup_graph(graph_storage.clone()).await?;

    // Get the graph for FlowRunner
    let graph = graph_storage.get("").await?.ok_or(" graph not found")?;

    // Create FlowRunner
    let flow_runner = Arc::new(FlowRunner::new(graph, session_storage.clone()));

    let state = AppState {
        flow_runner,
        session_storage,
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Greenrock chat service is running on: http://localhost:8000");

    Ok(())
}

// #[derive(BotCommands, Clone)]
// #[command(
//     rename_rule = "lowercase",
//     description = "These commands are supported:"
// )]
// enum Command {
//     #[command(description = "display this text.")]
//     Help,
//     #[command(description = "handle a username.")]
//     Username(String),
//     #[command(description = "handle a username and an age.", parse_with = "split")]
//     UsernameAndAge { username: String, age: u8 },
// }

// async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
//     match cmd {
//         Command::Help => {
//             bot.send_message(msg.chat.id, Command::descriptions().to_string())
//                 .await?
//         }
//         Command::Username(username) => {
//             bot.send_message(msg.chat.id, format!("Your username is @{username}."))
//                 .await?
//         }
//         Command::UsernameAndAge { username, age } => {
//             bot.send_message(
//                 msg.chat.id,
//                 format!("Your username is @{username} and age is {age}."),
//             )
//             .await?
//         }
//     };

//     Ok(())
// }
