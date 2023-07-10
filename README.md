# Azure Cost Analyzer
## Introduction
This is a simple tool to analyze the cost of Azure resources. It is based on the Azure Billing API and the Azure Resource Graph API. The tool is written in Rust.

## Required Env variables
copy the file .env.template to .env and fill in the required values

## Example Usage
```bash
cargo run analyze -t current-month -q "What is the most expensive Azure resource group I have used this month"

```