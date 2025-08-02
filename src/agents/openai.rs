use rig::client::ProviderClient;
use rig::providers::openai;
use rig::agent::Agent;
use rig::providers::openai::responses_api::ResponsesCompletionModel;

use crate::agents::market_analysis::MarketAnalysis;


fn test() -> Agent<ResponsesCompletionModel> {
    let openai_client = openai::Client::from_env();

    let agent = openai_client.
        agent(openai::GPT_4O_MINI).
        preamble("").
        tool(MarketAnalysis).
        build();

    return agent;
}