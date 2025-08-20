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

        let messages = context.get_all_messages().await;

        let agent = get_llm_agent(self.system_prompt.clone())
            .map_err(|e| TaskExecutionFailed(format!("Failed to initialize LLM agent: {e}")))?;

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

        info!("Answer generated: {}", answer);

        // Add the current answer attempt to chat history
        context.add_user_message(prompt).await;
        context
            .add_assistant_message(format!("Attempt {}: {}", retry_count + 1, answer))
            .await;

        context.set("answer", answer.clone()).await;

        Ok(TaskResult::new(Some(answer), NextAction::Continue))
    }
}
