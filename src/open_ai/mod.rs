#[async_trait::async_trait]
pub trait LlmChat {
    async fn chat(&self, chat: String) -> String;
}
// USe the Azure Open AI one here, https://lib.rs/crates/az-openai-rs
pub struct OpenAi {
    clinet: openai_rust::Client,
}
impl OpenAi {
    pub fn new(api_key: &str) -> Self {
        let client = openai_rust::Client::new(api_key);
        Self { clinet: client }
    }
}
#[async_trait::async_trait]
impl LlmChat for OpenAi {
    async fn chat(&self, chat: String) -> String {
        let args = openai_rust::chat::ChatArguments::new(
            "gpt-3.5-turbo",
            vec![openai_rust::chat::Message {
                role: "user".to_owned(),
                content: chat,
            }],
        );
        let res = self.clinet.create_chat(args).await.unwrap();
        res.choices[0].message.content.clone()
    }
}
pub fn get_chat_service() -> impl LlmChat {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    OpenAi::new(&api_key)
}
pub async fn example() {
    println!("Hello, world!");
    let client = openai_rust::Client::new(&std::env::var("OPENAI_API_KEY").unwrap());
    let args = openai_rust::chat::ChatArguments::new(
        "gpt-3.5-turbo",
        vec![openai_rust::chat::Message {
            role: "user".to_owned(),
            content: "Hello GPT!, what is the Capital of Russia?".to_owned(),
        }],
    );
    let res = client.create_chat(args).await.unwrap();
    println!("{}", res);
}
