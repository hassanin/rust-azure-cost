use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use url::Url;
mod cost_managment_example;
// use serde_json::{Result, Value};
use std::env;
use std::error::Error;

pub async fn do_stuff() -> Result<String, Box<dyn Error>> {
    let credential = DefaultAzureCredential::default();
    let response = credential.get_token("https://management.azure.com").await?;

    let subscription_id = env::var("AZURE_SUBSCRIPTION_ID")?;
    let url = Url::parse(&format!(
        "https://management.azure.com/subscriptions/{}/providers/Microsoft.CostManagement/Query?api-version=2019-11-01",
        subscription_id))?;
    // let data = r#"
    //    {
    //     "type": "Usage",
    //     "timeframe": "MonthToDate",
    //     "dataset": {
    //         "granularity": "Daily"
    //     }
    //     }"#;
    let data = r#"{
        "type": "ActualCost",
        "timeframe": "MonthToDate",
        "dataset": {
            "granularity": "Daily",
            "aggregation": {
      "totalCost": {
        "name": "PreTaxCost",
        "function": "Sum"
      }
    },
            "grouping":[{
            "type": "Dimension",
            "name": "ResourceGroup"
            }]
        }
        }"#;

    // Parse the string of data into serde_json::Value.
    let body: serde_json::Value = serde_json::from_str(data)?;
    print!("body: {:?}", body);
    let response = reqwest::Client::new()
        .post(url)
        .body(data)
        .header(
            "Authorization",
            format!("Bearer {}", response.token.secret()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", response);
    let my_value: serde_json::Value = serde_json::from_str(&response)?;
    println!("{:?}", my_value);
    Ok(response)
}
