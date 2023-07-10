use crate::{
    azure,
    my_cli::AnalyzeQuestion,
    open_ai::{get_chat_service, LlmChat},
};

use super::{Cli, Commands};

pub async fn handle_cli(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Free(free) => {
            println!("free: {:?}", free);
            Ok(())
        }
        Commands::Analyze(analyze) => {
            println!("analyze: {:?}", analyze);
            let AnalyzeQuestion {
                question,
                time_domain,
            } = analyze;
            println!("question: {:?}", question);
            println!("time_domain: {:?}", time_domain);
            let azure_cost = azure::do_stuff().await?;
            let my_chat_service = get_chat_service();
            let full_question = format!(
                "Based on the below Azure costs {}, please answer this question {} ",
                azure_cost, question
            );
            let response = my_chat_service.chat(full_question).await;
            println!("response2 : {:?}", response);
            Ok(())
        }
    }
}
