use askama::Template;
use async_trait::async_trait;
use graph_flow::GraphError::TaskExecutionFailed;
use graph_flow::{Context, MessageRole, NextAction, Task, TaskResult};
use rig::client::CompletionClient;
use rig::completion::Chat;
use rig::providers::openai;

use rig::message::Message;
use tracing::info;

use anyhow::Result;
// use rig::prelude::*;

use crate::processor::prompts::main_system::EntryInteractionUserInputTemplate;

pub const MAX_RETRIES: u32 = 3;

pub fn get_llm_agent(
    system_prompt: String,
) -> Result<rig::agent::Agent<rig::providers::openrouter::CompletionModel>> {
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY not set"))?;

    let client = rig::providers::openrouter::Client::new(&api_key);
    // let vector_store = rig::vector_store::in_memory_store::InMemoryVectorStore::default();

    // let a = client
    //     .extractor::<u32>("google/gemini-2.0-flash-001")
    //     .build();

    // let f = a.extract("".to_string()).await.unwrap();

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble(&system_prompt)
        .build();

    Ok(agent)
}

pub struct EntryInteractionTask {
    system_prompt: String,
}

impl EntryInteractionTask {
    pub fn new(system_prompt: String) -> Self {
        Self { system_prompt }
    }
}

#[async_trait]
impl Task for EntryInteractionTask {
    fn id(&self) -> &str {
        std::any::type_name::<Self>()
    }

    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        info!("Starting answer generation task");

        // let user_query: String = context
        //     .get_sync("user_query")
        //     .ok_or_else(|| TaskExecutionFailed("user_query not found in context".into()))?;

        // let ctx: String = context
        //     .get_sync("retrieved_context")
        //     .ok_or_else(|| TaskExecutionFailed("retrieved_context not found in context".into()))?;

        let retry_count: u32 = context
            .get_sync("retry_count")
            .ok_or_else(|| TaskExecutionFailed("retry_count not found in context".into()))?;

        let user_input: String = context
            .get_sync("user_input")
            .ok_or_else(|| TaskExecutionFailed("user_input not found in context".into()))?;

        info!(
            "Generating answer (attempt {} of {})",
            retry_count + 1,
            MAX_RETRIES + 1
        );

        // Get the full chat history for conversational memory
        let messages = context.get_all_messages().await;

        // chat_history.last_messages(n)

        // .into_iter()
        // .map(|m| rig::completion::Message::)
        // .collect();

        let agent = get_llm_agent(self.system_prompt.clone())
            .map_err(|e| TaskExecutionFailed(format!("Failed to initialize LLM agent: {e}")))?;

        // let prompt = if history.is_empty() {
        //     format!(
        //         r#"
        //     You are a movie recommendation assistant.
        //     Use the following information to answer the user request for a movie recommendation.
        //     If the information is not sufficient, answer as best you can.
        //     Information:
        //     {ctx}
        //     Question: {user_input}"#
        //     )
        // } else {
        //     info!(retry_count = %retry_count, "running a retry attempt");
        //     format!(
        //         r#"
        //     You are a movie recommendation assistant.
        //     The user asked: "{user_input}"

        //     Based on the validation feedback in our conversation above, and the context above, provide an improved movie recommendation.
        //     Focus on the specific issues mentioned in the feedback.
        //     Provide a complete recommendation without referring to previous attempts.
        //     "#
        //     )
        // };

        let context_json = serde_json::to_string(&context).unwrap();

        let prompt = EntryInteractionUserInputTemplate {
            user_input,
            history: messages.clone(),
            context: context_json,
        }
        .render()
        .unwrap();

        // let c = context.get_rig_messages().await;

        let answer = agent
            .chat(
                prompt.clone(),
                messages
                    .iter()
                    .map(|m| match m.role {
                        MessageRole::User => Message::user(m.content.clone()),
                        MessageRole::Assistant => Message::assistant(m.content.clone()),
                        MessageRole::System => Message::assistant(m.content.clone()),
                    })
                    .collect(),
            )
            .await
            .map_err(|e| TaskExecutionFailed(format!("LLM prompt failed: {e}")))?;

        // let answer = agent
        //     .chat(
        //         &prompt,
        //         messages
        //             .iter()
        //             .map(|m| match m.role {
        //                 MessageRole::User => Message::user(m.content.clone()),
        //                 MessageRole::Assistant => Message::assistant(m.content.clone()),
        //                 MessageRole::System => Message::assistant(m.content.clone()),
        //             })
        //             .collect(),
        //     )
        //     .await
        //     .map_err(|e| TaskExecutionFailed(format!("LLM chat failed: {e}")))?;

        info!("Answer generated: {}", answer);

        // Add the current answer attempt to chat history
        context.add_user_message(prompt).await;
        context
            .add_assistant_message(format!("Attempt {}: {}", retry_count + 1, answer))
            .await;

        context.set("answer", answer.clone()).await;

        Ok(TaskResult::new(Some(answer), NextAction::End))
    }
}
