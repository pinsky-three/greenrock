// use apca::ApiInfo;
// use apca::Client;
// use apca::api::v2::order;

// use num_decimal::Num;

use std::{env, sync::atomic::AtomicBool};

use binance::{
    account::Account,
    api::Binance,
    market::Market,
    websockets::{WebSockets, WebsocketEvent},
};
use chrono::Utc;

// #[tokio::main]
fn main() {
    dotenvy::dotenv().ok();

    let market: Market = Binance::new(None, None);

    // Order book at default depth
    // match market.get_depth("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    // // Order book at depth 500
    // match market.get_custom_depth("BNBETH", 500) {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {}", e),
    // }

    // // Latest price for ALL symbols
    // match market.get_all_prices() {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Latest price for ONE symbol
    // match market.get_price("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Current average price for ONE symbol
    // match market.get_average_price("BNBETH") {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Best price/qty on the order book for ALL symbols
    // match market.get_all_book_tickers() {
    //     Ok(answer) => println!("{:?}", answer),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // Best price/qty on the order book for ONE symbol
    // match market.get_book_ticker("BNBETH") {
    //     Ok(answer) => println!(
    //         "Bid Price: {}, Ask Price: {}",
    //         answer.bid_price, answer.ask_price
    //     ),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // // 24hr ticker price change statistics
    match market.get_24h_price_stats("BTCUSDT") {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // // last 10 5min klines (candlesticks) for a symbol:
    // match market.get_klines("BNBETH", "1m", 10, None, None) {
    //     Ok(klines) => {
    //         match klines {
    //             binance::model::KlineSummaries::AllKlineSummaries(klines) => {
    //                 let kline: KlineSummary = klines[0].clone(); // You need to iterate over the klines
    //                 println!(
    //                     "Open: {}, High: {}, Low: {}",
    //                     kline.open, kline.high, kline.low
    //                 )
    //             }
    //         }
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    let api_key = Some(env::var("BINANCE_API_KEY").unwrap());
    let secret_key = Some(env::var("BINANCE_SECRET_KEY").unwrap());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account() {
        Ok(answer) => println!(
            "{:?}",
            answer
                .balances
                .iter()
                .filter(|instrument| instrument.locked.parse::<f64>().unwrap() > 0.0)
                .collect::<Vec<_>>()
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let kline = "btcusdt@kline_1s".to_string();

    let mut now = Utc::now();

    let mut web_socket = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::Kline(kline_event) = event {
            let current = Utc::now();
            let diff = current - now;
            now = current;
            // println!("diff: {:?}", diff);
            println!(
                "Symbol: {}, high: {}, low: {}, diff (ms): {}",
                kline_event.kline.symbol,
                kline_event.kline.low,
                kline_event.kline.high,
                // kline_event.kline.open,
                // kline_event.kline.close,
                // kline_event.kline.open_time,
                diff.num_milliseconds(),
            );
        };
        Ok(())
    });

    web_socket.connect(&kline).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }

    web_socket.disconnect().unwrap();
}

// use async_trait::async_trait;
// use graph_flow::{
//     Context, ExecutionStatus, FlowRunner, GraphBuilder, GraphStorage, InMemoryGraphStorage,
//     InMemorySessionStorage, NextAction, Session, SessionStorage, Task, TaskResult,
// };
// use rig::prelude::*;
// use rig::{completion::Prompt, providers};
// use std::env;
// use std::sync::Arc;

// // We have 2 tasks in this simple example:
// // 1. HelloTask - greets the user by name
// // 2. ExcitementTask - adds excitement to the greeting
// struct HelloTask;

// #[async_trait]
// impl Task for HelloTask {
//     async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
//         let name: String = context.get_sync("name").unwrap();
//         let greeting = format!("Hello, {}", name);
//         // Store result for next task
//         context.set("greeting", greeting.clone()).await;

//         let client = providers::openrouter::Client::new(
//             &env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set"),
//         );

//         // Create agent with a single context prompt
//         let comedian_agent = client
//             .agent("deepseek/deepseek-chat-v3-0324:free")
//             .preamble("You are a comedian here to entertain the user using humour and jokes.")
//             .build();

//         // Prompt the agent and print the response
//         let response = comedian_agent.prompt("Entertain me!").await.unwrap();

//         println!("{response}");

//         // using NextAction::Continue to indicate we want to proceed to the next task,
//         // but we want to advance just one step and give control back to the workflow manager
//         // We can also use NextAction::ContinueAndExecute if we want to execute the next task immediately
//         Ok(TaskResult::new(Some(greeting), NextAction::Continue))
//     }
// }

// // Define a task that adds excitement
// struct ExcitementTask;

// #[async_trait]
// impl Task for ExcitementTask {
//     async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
//         let greeting: String = context.get_sync("greeting").unwrap();
//         let excited = format!("{} !!!", greeting);

//         Ok(TaskResult::new(Some(excited), NextAction::End))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenvy::dotenv().ok();

//     // Create storage instances
//     let session_storage = Arc::new(InMemorySessionStorage::new());
//     let graph_storage = Arc::new(InMemoryGraphStorage::new());

//     // Build a simple workflow
//     let hello_task = Arc::new(HelloTask);
//     let hello_task_id = hello_task.id().to_string();
//     let excitement_task = Arc::new(ExcitementTask);
//     let excitement_task_id = excitement_task.id().to_string();

//     let graph = Arc::new(
//         GraphBuilder::new("greeting_workflow")
//             .add_task(hello_task)
//             .add_task(excitement_task)
//             .add_edge(&hello_task_id, &excitement_task_id) // Connect the tasks
//             .build(),
//     );

//     // Store the graph in graph storage
//     graph_storage
//         .save("greeting_workflow".to_string(), graph.clone())
//         .await?;

//     // Create a session with initial context
//     let session_id = "session_001".to_string();
//     let session = Session::new_from_task(session_id.clone(), &hello_task_id);

//     // Set up context with input data
//     session.context.set("name", "Batman".to_string()).await;
//     // Save the session
//     session_storage.save(session.clone()).await?;

//     println!("Starting simple workflow with FlowRunner\n");
//     println!("Session ID: {}", session.id);
//     println!("Initial task: {}\n", session.current_task_id);

//     // Create a FlowRunner that hides the load / execute / save boilerplate
//     let runner = FlowRunner::new(graph.clone(), session_storage.clone());

//     // Execute until completion
//     loop {
//         let execution_result = runner.run(&session_id).await?;

//         if let Some(response) = &execution_result.response {
//             println!("Task response: {}", response);
//         }

//         println!("Execution status: {:?}", execution_result.status);

//         match execution_result.status {
//             ExecutionStatus::Completed => {
//                 println!("Workflow completed successfully!");
//                 break;
//             }
//             ExecutionStatus::Paused {
//                 next_task_id,
//                 reason,
//             } => {
//                 println!(
//                     "Workflow paused, will continue to task: {} (reason: {}) – continuing...\n",
//                     next_task_id, reason
//                 );
//                 continue;
//             }
//             ExecutionStatus::WaitingForInput => {
//                 println!("Workflow waiting for user input – continuing...\n");
//                 continue;
//             }
//             ExecutionStatus::Error(err) => {
//                 println!("Error occurred: {}", err);
//                 break;
//             }
//         }
//     }

//     // Demonstrate session persistence by retrieving final session
//     let final_session = session_storage
//         .get(&session_id)
//         .await?
//         .ok_or("Session not found")?;

//     println!("\nFinal session state:");
//     println!("Session ID: {}", final_session.id);
//     println!("Current task: {}", final_session.current_task_id);
//     if let Some(status) = &final_session.status_message {
//         println!("Final status: {}", status);
//     }

//     // Demonstrate retrieving stored values from context
//     if let Some(greeting) = final_session.context.get::<String>("greeting").await {
//         println!("Stored greeting: {}", greeting);
//     }

//     println!("\nWorkflow execution finished");
//     Ok(())
// }
