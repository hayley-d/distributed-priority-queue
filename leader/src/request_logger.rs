use log::info;
use rocket::fairing::Fairing;
use rocket::fairing::{Info, Kind};
use rocket::{Data, Request};
pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let method = request.method();
        let uri = request.uri();
        let client_ip = request
            .remote()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "Unknown IP".to_string());
        let user_agent = request
            .headers()
            .get_one("User-Agent")
            .unwrap_or_else(|| "Unknown User-Agent");
        info!(
            "method={} uri={} ip={} user_agent={}",
            method, uri, client_ip, user_agent
        );
    }
}
