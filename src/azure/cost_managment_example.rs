pub async fn cost_management_example() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    // let tenant_id = "<your-tenant-id>";
    // let client_id = "<your-client-id>";
    // let client_secret = "<your-client-secret>";
    // let subscription_id = "<your-subscription-id>";

    // let token_credential = AzureCliCredential {};
    // let token = token_credential.get_token("https://management.azure.com/")?;
    // let access_token = token.token.as_str();

    // // Create HTTP client
    // let client = Client::new();
    // let mut headers = HeaderMap::new();
    // headers.insert(AUTHORIZATION, format!("Bearer {}", access_token).parse()?);

    // // Specify the time range for the query
    // let start_date = (Utc::now() - Duration::days(30))
    //     .format("%Y-%m-%d")
    //     .to_string();
    // let end_date = Utc::now().format("%Y-%m-%d").to_string();

    // // Retrieve the cost insights for the specified time range and group by resource group
    // let query_definition = query::UsageByResourceGroupQueryDefinition {
    //     type_: "ActualCost".to_string(),
    //     timeframe: format!("{}/{}", start_date, end_date),
    //     dataset: query::UsageByResourceGroupQueryDataset {
    //         granularity: "None".to_string(),
    //         group_by: vec!["ResourceGroupName".to_string()],
    //     },
    // };

    // let response = client
    //     .post(&format!(
    //         "https://management.azure.com/subscriptions/{}/providers/Microsoft.CostManagement/query?api-version=2019-11-01",
    //         subscription_id
    //     ))
    //     .headers(headers)
    //     .json(&query_definition)
    //     .send()
    //     .await?;

    // // Process the response and print the breakdown of costs by resource group
    // let query_response: query::QueryResponse = response.json().await?;

    // if let Some(rows) = query_response.properties.rows {
    //     for row in rows {
    //         let resource_group = row[0].to_string();
    //         let cost = row[1].as_f64().unwrap();

    //         println!("Resource Group: {}, Cost: {}", resource_group, cost);
    //     }
    // }

    Ok(())
}
