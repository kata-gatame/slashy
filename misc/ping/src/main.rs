use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(ping)).await?;
    Ok(())
}

async fn ping(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({
        "message": "Pong!"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping_handles() {
        let request = Request::default();
        let expected = json!({
            "message": "Pong!"
        })
        .into_response();
        let response = ping(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
