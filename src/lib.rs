use worker::*;

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("Hello World!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, 10);
    }
}
