use std::{collections::HashMap, env, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State, WebSocketUpgrade, ws::WebSocket},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, get_service, post},
};

use chrono::DateTime;
use graph_flow::{
    Context, ExecutionStatus, FlowRunner, GraphStorage, InMemoryGraphStorage,
    PostgresSessionStorage, Session, SessionStorage,
};

use greenrock::{
    analysis::graph::setup_graph,
    brokers::binance::BinanceBroker,
    models::timeseries::Candle,
    processor::tasks::entry_interaction_task::EntryInteractionTask,
    runner::core::{RunConfig, Runner},
    strategy::core::{MinimalStrategy, Strategy},
};

use serde_json::json;

use polars::frame::DataFrame;
// use polars::prelude::{IntoLazy, col};
use serde::{Deserialize, Serialize};

use tracing::{Level, error, info};
use uuid::Uuid;

use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    flow_runner: Arc<FlowRunner>,
    session_storage: Arc<dyn SessionStorage>,
    live_loop_runner: Arc<Runner<HashMap<String, f64>, BinanceBroker, MinimalStrategy>>,
    greenrock_session: Arc<GreenrockSession>,
}

fn internal_error(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

async fn health_check(State(_state): State<AppState>) -> &'static str {
    &"OK"
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

#[derive(Debug, Serialize)]
struct PauseResponse {
    session_id: String,
    status: String,
    next_task: String,
    reason: String,
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
        graph_id: "".to_string(),
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
        } => Json(PauseResponse {
            session_id,
            status: "paused".to_string(),
            next_task: next_task_id.to_string(),
            reason: reason.to_string(),
        })
        .into_response(),
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

async fn get_balance(State(state): State<AppState>) -> Response {
    match tokio::task::spawn_blocking(move || state.live_loop_runner.balance()).await {
        Ok(balance) => Json(balance).into_response(),
        Err(e) => {
            error!("Failed to get balance: {}", e);
            internal_error("Failed to get balance")
        }
    }
}

#[derive(Deserialize)]
struct OpenOrdersQuery {
    symbol: String,
}

async fn get_open_orders(
    State(state): State<AppState>,
    Query(params): Query<OpenOrdersQuery>,
) -> Response {
    match tokio::task::spawn_blocking(move || state.live_loop_runner.open_orders(&params.symbol))
        .await
    {
        Ok(orders) => Json(orders).into_response(),
        Err(e) => {
            error!("Failed to get open orders: {}", e);
            internal_error("Failed to get open orders")
        }
    }
}

#[derive(Deserialize)]
struct TradeHistoryQuery {
    symbol: String,
}

async fn get_trade_history(
    State(state): State<AppState>,
    Query(params): Query<TradeHistoryQuery>,
) -> Response {
    match tokio::task::spawn_blocking(move || state.live_loop_runner.trade_history(&params.symbol))
        .await
    {
        Ok(history) => Json(history).into_response(),
        Err(e) => {
            error!("Failed to get trade history: {}", e);
            internal_error("Failed to get trade history")
        }
    }
}

// #[derive(Clone)]
struct GreenrockSession {
    _id: Uuid,
    symbol: String,
    interval: String,
    _candles: Vec<Candle>,
    _balance: HashMap<String, f64>,
}

async fn get_portfolio(State(state): State<AppState>) -> Response {
    match tokio::task::spawn_blocking(move || state.live_loop_runner.portfolio()).await {
        Ok(portfolio) => Json(portfolio).into_response(),
        Err(e) => {
            error!("Failed to get portfolio: {}", e);
            internal_error("Failed to get portfolio")
        }
    }
}

#[derive(Debug, Deserialize)]
struct CandlesQuery {
    symbol: String,
    interval: String,
    start: Option<String>,
    end: Option<String>,
}

async fn get_candles(
    State(state): State<AppState>,
    Query(params): Query<CandlesQuery>,
) -> Response {
    let candles = state
        .live_loop_runner
        .candles(
            &params.symbol,
            &params.interval,
            500,
            params
                .start
                .as_ref()
                .map(|s| DateTime::parse_from_rfc3339(s).unwrap().to_utc()),
            params
                .end
                .as_ref()
                .map(|e| DateTime::parse_from_rfc3339(e).unwrap().to_utc()),
        )
        .await;

    Json(json!({
        "candles": candles,
    }))
    .into_response()
}

async fn get_candle_stream(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_candles_socket_stream(socket, state))
}

async fn get_order_book_stream(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_depth_socket_stream(socket, state))
}

#[derive(Debug, Deserialize)]
struct OrderBookQuery {
    symbol: String,
    depth: u64,
}

async fn get_order_book(
    State(state): State<AppState>,
    Query(params): Query<OrderBookQuery>,
) -> Response {
    match tokio::task::spawn_blocking(move || {
        state
            .live_loop_runner
            .order_book(&params.symbol, params.depth)
    })
    .await
    {
        Ok(order_book) => Json(order_book).into_response(),
        Err(e) => {
            error!("Failed to get order book: {}", e);
            internal_error("Failed to get order book")
        }
    }
}

async fn handle_candles_socket_stream(mut socket: WebSocket, state: AppState) {
    info!("WebSocket client connected");
    let mut stream = state
        .live_loop_runner
        .candles_stream(
            &state.greenrock_session.symbol.clone(),
            &state.greenrock_session.interval.clone(),
        )
        .await;

    loop {
        tokio::select! {
            // Handle incoming candle data
            recv_result = stream.recv() => {
                match recv_result {
                    Ok(candle) => {
                        match serde_json::to_string(&candle) {
                            Ok(msg) => {
                                if socket.send(axum::extract::ws::Message::Text(msg.into())).await.is_err() {
                                    info!("WebSocket client disconnected");
                                    return;
                                }
                            }
                            Err(e) => {
                                error!("Failed to serialize candle: {}", e);
                                continue;
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(count)) => {
                        info!("WebSocket lagged by {} messages, continuing", count);
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        info!("Candle stream closed");
                        return;
                    }
                }
            }
            // Handle incoming WebSocket messages (for ping/pong, close, etc.)
            msg_result = socket.recv() => {
                match msg_result {
                    Some(Ok(axum::extract::ws::Message::Close(_))) => {
                        info!("WebSocket client sent close message");
                        return;
                    }
                    Some(Ok(axum::extract::ws::Message::Ping(data))) => {
                        if socket.send(axum::extract::ws::Message::Pong(data)).await.is_err() {
                            return;
                        }
                    }
                    Some(Err(_)) => {
                        info!("WebSocket client connection error");
                        return;
                    }
                    None => {
                        info!("WebSocket client disconnected");
                        return;
                    }
                    _ => {
                        // Ignore other message types
                    }
                }
            }
        }
    }
}

async fn handle_depth_socket_stream(mut socket: WebSocket, state: AppState) {
    info!("Depth WebSocket client connected");
    let mut stream = state
        .live_loop_runner
        .order_book_stream(&state.greenrock_session.symbol.clone());

    loop {
        tokio::select! {
            // Handle incoming order book data
            recv_result = stream.recv() => {
                match recv_result {
                    Ok(order_book) => {
                        match serde_json::to_string(&order_book) {
                            Ok(msg) => {
                                if socket.send(axum::extract::ws::Message::Text(msg.into())).await.is_err() {
                                    info!("Depth WebSocket client disconnected");
                                    return;
                                }
                            }
                            Err(e) => {
                                error!("Failed to serialize order book: {}", e);
                                continue;
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(count)) => {
                        info!("Depth WebSocket lagged by {} messages, continuing", count);
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        info!("Depth stream closed");
                        return;
                    }
                }
            }
            // Handle incoming WebSocket messages (for ping/pong, close, etc.)
            msg_result = socket.recv() => {
                match msg_result {
                    Some(Ok(axum::extract::ws::Message::Close(_))) => {
                        info!("Depth WebSocket client sent close message");
                        return;
                    }
                    Some(Ok(axum::extract::ws::Message::Ping(data))) => {
                        if socket.send(axum::extract::ws::Message::Pong(data)).await.is_err() {
                            return;
                        }
                    }
                    Some(Err(_)) => {
                        info!("Depth WebSocket client connection error");
                        return;
                    }
                    None => {
                        info!("Depth WebSocket client disconnected");
                        return;
                    }
                    _ => {
                        // Ignore other message types
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    info!("Starting greenrock chat service");

    let database_url =
        env::var("DATABASE_URL").map_err(|_| "DATABASE_URL environment variable not set")?;

    let session_storage: Arc<dyn SessionStorage> =
        Arc::new(PostgresSessionStorage::connect(&database_url).await?);

    let graph_storage: Arc<dyn GraphStorage> = Arc::new(InMemoryGraphStorage::new());

    setup_graph(graph_storage.clone()).await?;

    let graph = graph_storage.get("").await?.ok_or(" graph not found")?;

    let flow_runner = Arc::new(FlowRunner::new(graph.clone(), session_storage.clone()));

    let strategy = MinimalStrategy::new(DataFrame::new(vec![]).unwrap());
    let initial_state = strategy.initial_state();

    let binance_broker = BinanceBroker::default();

    let runner = Arc::new(Runner::new(binance_broker, strategy));

    let state = AppState {
        flow_runner,
        session_storage,
        live_loop_runner: runner.clone(),
        greenrock_session: Arc::new(GreenrockSession {
            _id: Uuid::new_v4(),
            symbol: "BTCUSDT".to_string(),
            interval: "1m".to_string(),
            _candles: vec![],
            _balance: HashMap::new(),
        }),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let app: Router = Router::new()
        .route("/health", get(health_check))
        .route("/chat", post(chat))
        //
        .route("/strategy/portfolio", get(get_portfolio))
        //
        .route("/broker/balance", get(get_balance))
        .route("/broker/open_orders", get(get_open_orders))
        .route("/broker/trade_history", get(get_trade_history))
        .route("/broker/candles", get(get_candles))
        .route("/broker/candle_stream", get(get_candle_stream))
        .route("/broker/order_book", get(get_order_book))
        .route("/broker/order_book_stream", get(get_order_book_stream))
        .fallback_service(get_service(ServeDir::new("web-ui/dist")))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(state);

    info!("Starting both web server and trading runner...");

    // Spawn the web server task
    let web_server_handle = tokio::spawn(async move {
        info!("Greenrock chat service is running on: http://localhost:4200");
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4200").await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    // Spawn the trading runner task
    let trading_runner_handle = tokio::spawn(async move {
        info!("Starting trading runner for BTCUSDT...");
        runner
            .run_until_ctrl_c(
                &RunConfig {
                    symbol: "BTCUSDT".to_string(),
                    interval: "1m".to_string(),
                },
                initial_state,
            )
            .await;
    });

    // Wait for Ctrl+C or either task to complete
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
        result = web_server_handle => {
            if let Err(e) = result {
                error!("Web server task failed: {}", e);
            }
        }
        result = trading_runner_handle => {
            if let Err(e) = result {
                error!("Trading runner task failed: {}", e);
            }
        }
    }

    info!("shutting down");

    Ok(())
}
