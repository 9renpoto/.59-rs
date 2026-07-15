use application::{create_greeting, CreateGreeting};
use serde::Serialize;
use worker::*;

#[event(fetch)]
async fn fetch(mut req: Request, env: Env, _ctx: Context) -> Result<Response> {
    match (req.method(), req.path().as_str()) {
        (Method::Get, "/health") => health(&env).await,
        (Method::Post, "/api/greetings") => create_greeting_response(&mut req).await,
        _ => Response::error("Not Found", 404),
    }
}

async fn health(env: &Env) -> Result<Response> {
    let result = async {
        let database = env.d1("DB")?;
        database
            .prepare("SELECT 1 AS healthy")
            .first::<serde_json::Value>(None)
            .await?;
        Ok::<(), Error>(())
    }
    .await;

    match result {
        Ok(()) => json_response(&HealthResponse::ok(), 200),
        Err(error) => {
            console_error!("D1 health check failed: {error}");
            json_response(&HealthResponse::unavailable(), 503)
        }
    }
}

async fn create_greeting_response(req: &mut Request) -> Result<Response> {
    let command = match req.json::<CreateGreeting>().await {
        Ok(command) => command,
        Err(_) => return json_response(&ErrorResponse::new("invalid JSON body"), 400),
    };

    match create_greeting(command) {
        Ok(response) => json_response(&response, 200),
        Err(_) => json_response(&ErrorResponse::new("name must be 1 to 100 characters"), 422),
    }
}

fn json_response<T: Serialize>(body: &T, status: u16) -> Result<Response> {
    let mut response = Response::from_json(body)?;
    response = response.with_status(status);
    Ok(response)
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

impl HealthResponse {
    const fn ok() -> Self {
        Self { status: "ok" }
    }

    const fn unavailable() -> Self {
        Self {
            status: "unavailable",
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

impl ErrorResponse {
    const fn new(error: &'static str) -> Self {
        Self { error }
    }
}
