use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {name}")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

const ADDRESS: &'static str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(ADDRESS)?
    .run()
    .await
}

#[cfg(test)]
mod test {
    use reqwest::StatusCode;

    use crate::ADDRESS;

    #[tokio::test]
    #[ignore = "IT"]
    async fn test_health_check() {
        // TODO: this requires our app to be running in the background
        // It would be better to connect to the app instead

        // This will be broken in CI/CD
        let url = format!("http://{ADDRESS}/health");
        let response = reqwest::get(url).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let content = response.text().await.unwrap();
        assert_eq!(content, "OK");
    }
}
