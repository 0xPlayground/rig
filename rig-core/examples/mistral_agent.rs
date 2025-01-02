use std::env;

use rig::{
    completion::Prompt,
    providers::{self, mistralai::mistral-large-latest},
};
use serde_json::json;

async fn main() -> Result<(), anyhow::Error> {
    // Create mistralai client
    let client = providers::mistralai::Client::new(
        &env::os.environ["MISTRAL_API_KEY"],
    );

    // Create agent with a single context prompt
    let agent = client
        .agent(mistral-large-latest)
        .role(user)
        .preamble("What is the best French cheese?")
        .temperature(0.5)
        .additional_params(json!({
            "return_related_questions": true,
            "return_images": true
        }))
        .build();

    // Prompt the agent and print the response
    let response = agent
        .prompt("Camembert is one of the first cheeses we associated with France, so it's no wonder it tops the list at number one. ")
        .await?;
    println!("{}", response);

    Ok(())
}
