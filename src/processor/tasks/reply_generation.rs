use async_trait::async_trait;
use graph_flow::GraphError::TaskExecutionFailed;
use graph_flow::{Context, MessageRole, NextAction, Task, TaskResult};
use rig::completion::Chat;
use rig::message::Message;
use tracing::info;

use anyhow::Result;
use rig::prelude::*;

pub const MAX_RETRIES: u32 = 3;

pub fn get_llm_agent() -> Result<rig::agent::Agent<rig::providers::openrouter::CompletionModel>> {
    let api_key = std::env::var("OPENROUTER_API_KEY")
        .map_err(|_| anyhow::anyhow!("OPENROUTER_API_KEY not set"))?;
    let client = rig::providers::openrouter::Client::new(&api_key);
    Ok(client.agent("google/gemini-2.0-flash-001").build())
}

pub struct ReplyGenerationTask;

#[async_trait]
impl Task for ReplyGenerationTask {
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

        let ctx: String = "foo".to_string();

        info!(
            "Generating answer (attempt {} of {})",
            retry_count + 1,
            MAX_RETRIES + 1
        );

        // Get the full chat history for conversational memory
        let history = context.get_all_messages().await;
        // .into_iter()
        // .map(|m| rig::completion::Message::)
        // .collect();

        let agent = get_llm_agent()
            .map_err(|e| TaskExecutionFailed(format!("Failed to initialize LLM agent: {e}")))?;

        let prompt = if history.is_empty() {
            format!(
                r#"
            You are a movie recommendation assistant.
            Use the following information to answer the user request for a movie recommendation.
            If the information is not sufficient, answer as best you can.
            Information:
            {ctx}
            Question: {user_input}"#
            )
        } else {
            info!(retry_count = %retry_count, "running a retry attempt");
            format!(
                r#"
            You are a movie recommendation assistant.
            The user asked: "{user_input}"
            
            Based on the validation feedback in our conversation above, and the context above, provide an improved movie recommendation.
            Focus on the specific issues mentioned in the feedback.
            Provide a complete recommendation without referring to previous attempts.
            "#
            )
        };

        let answer = agent
            .chat(
                &prompt,
                history
                    .iter()
                    .map(|m| match m.role {
                        MessageRole::User => Message::user(m.content.clone()),
                        MessageRole::Assistant => Message::assistant(m.content.clone()),
                        MessageRole::System => Message::assistant(m.content.clone()),
                    })
                    .collect(),
            )
            .await
            .map_err(|e| TaskExecutionFailed(format!("LLM chat failed: {e}")))?;

        info!("Answer generated: {}", answer);

        // Add the current answer attempt to chat history
        context.add_user_message(prompt).await;
        context
            .add_assistant_message(format!("Attempt {}: {}", retry_count + 1, answer))
            .await;
        context.set("answer", answer.clone()).await;

        Ok(TaskResult::new(
            Some(answer),
            NextAction::ContinueAndExecute,
        ))
    }
}
