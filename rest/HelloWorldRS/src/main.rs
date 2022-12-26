//use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
use std::env;

/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
 /*    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config); */

    // Register the Lambda handler
    //
    // We use a closure to pass the `dynamodb_client` and `table_name` as arguments
    // to the handler function.
    lambda_http::run(service_fn(|request: Request| {
     // put_item(&dynamodb_client, &table_name, request)
       //Ok(Response::builder().status(200).body("item saved"))

       async move {

        Ok(Response::builder().status(200).body("Hello World5")? )
       }
    }))
    .await?;

    Ok(())
}

