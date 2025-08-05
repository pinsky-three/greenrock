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
use greenrock::{
    processor::tasks::reply_generation::ReplyGenerationTask,
    // strategy::{
    //     core::{MinimalStrategy, Strategy},
    //     utils::row_to_kline,
    // },
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

    // Create new session
    let session_id = Uuid::new_v4().to_string();
    let reply_task_id = std::any::type_name::<ReplyGenerationTask>();

    // Set up context with chat history limit
    let context = Context::with_max_chat_messages(50);

    context.set("user_input", params.query.clone()).await;
    context.set("retry_count", 0).await;

    let session = Session {
        id: session_id.clone(),
        graph_id: "".to_string(),
        current_task_id: reply_task_id.to_string(),
        status_message: None,
        context,
    };

    // Save initial session - FlowRunner will handle persistence during execution
    if let Err(e) = state.session_storage.save(session).await {
        error!("Failed to save session: {}", e);
        return internal_error("Failed to save session");
    }

    info!("Session created with ID: {}", session_id);

    // Execute workflow using FlowRunner - automatically handles session persistence
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

    // Create tasks
    let reply_task: Arc<dyn Task> = Arc::new(ReplyGenerationTask);
    // let search_task: Arc<dyn Task> = Arc::new(VectorSearchTask::new().await?);
    // let answer_task: Arc<dyn Task> = Arc::new(AnswerGenerationTask);
    // let validate_task: Arc<dyn Task> = Arc::new(ValidationTask);
    // let deliver_task: Arc<dyn Task> = Arc::new(DeliveryTask);

    // let reply_id = reply_task.id().to_string();
    // let search_id = search_task.id().to_string();
    // let answer_id = answer_task.id().to_string();
    // let validate_id = validate_task.id().to_string();
    // let deliver_id = deliver_task.id().to_string();

    // Build graph
    let graph = Arc::new(
        GraphBuilder::new("greenrock_main_flow")
            .add_task(reply_task)
            // .add_task(search_task)
            // .add_task(answer_task)
            // .add_task(validate_task)
            // .add_task(deliver_task)
            // .add_edge(reply_id.clone(), search_id.clone())
            // .add_edge(search_id.clone(), answer_id.clone())
            // .add_edge(answer_id.clone(), validate_id.clone())
            // // Conditional routing: if validation passes go to delivery, else back to answer generation
            // .add_conditional_edge(
            //     validate_id.clone(),
            //     |ctx| ctx.get_sync::<bool>("validation_passed").unwrap_or(false),
            //     deliver_id.clone(),
            //     answer_id.clone(), // Back to answer generation for retry
            // )
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
