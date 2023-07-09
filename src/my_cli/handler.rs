use crate::{
    my_cli::AnalyzeQuestion,
    open_ai::{get_chat_service, LlmChat},
};

use super::{Cli, Commands};

pub async fn handle_cli(cli: Cli) {
    match cli.command {
        Commands::Free(free) => {
            println!("free: {:?}", free);
        }
        Commands::Analyze(analyze) => {
            println!("analyze: {:?}", analyze);
            let AnalyzeQuestion {
                question,
                time_domain,
            } = analyze;
            println!("question: {:?}", question);
            println!("time_domain: {:?}", time_domain);
            let my_chat_service = get_chat_service();
            let response = my_chat_service.chat(question).await;
            println!("response2 : {:?}", response);
        }
    }
}
