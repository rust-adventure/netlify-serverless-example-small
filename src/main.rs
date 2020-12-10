use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok("🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀
    __________________________
    < Hello fellow Rustaceans! >
     --------------------------
            \
             \
                _~^~^~_
            \\) /  o o  \\ (/
              '_   -   _'
              / '-----' \\")
}
