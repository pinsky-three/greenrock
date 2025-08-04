use rig::client::CompletionClient;
use rig::providers::openai;
//use rig::providers::openai::ResponsesCompletionModel;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig::agent::Agent;
use crate::agents::market_analysis::MarketAnalysis;


fn NewAgent(api_key: &str) -> Agent<ResponsesCompletionModel> {
    
    let openai_client = openai::Client::new(api_key);

    let agent: Agent<openai::responses_api::ResponsesCompletionModel> = openai_client.
        agent(openai::GPT_4O_MINI).
        preamble("").
        tool(MarketAnalysis).
        build();

    return agent;
}