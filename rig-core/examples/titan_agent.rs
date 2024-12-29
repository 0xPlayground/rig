use std::env;

use rig::{
    completion::Prompt,
    providers::polly::{self, polly},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create OpenAI client
    let client = polly::ClientBuilder::new(
        &env::var("POLLY_API_KEY").expect("POLLY_API_KEY not set"),
    )
    .build();

    // Create agent with a single context prompt
    let agent = client
        .agent(polly)
        .preamble("Be precise and concise.")
        .temperature(0.5)
        .build();

    // Prompt the agent and print the response
    let response = agent
        .prompt("Hello, my name is Joanna. I learned about the W3C on 10/3 of last year.")
        .await?;
    println!("{}", response);

    Ok(())
}
