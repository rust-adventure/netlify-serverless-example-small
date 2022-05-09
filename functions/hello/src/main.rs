use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyResponse,
    },
};
use http::HeaderMap;
use lambda_runtime::{service_fn,LambdaEvent,  Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = service_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (event, _) = event.into_parts();
    let world = "world".to_string();
    let first_name = event
        .query_string_parameters
        .first("firstName")
        .unwrap_or(&world);

    dbg!(&first_name);
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!(
            "Hello, {}!",
            first_name
        ))),
        is_base64_encoded: Some(false),
    })
}
