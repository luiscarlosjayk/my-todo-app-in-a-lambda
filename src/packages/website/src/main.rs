use actix_files::Files;
use actix_web::{App, HttpServer, middleware, web};
use std::io;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;
use website::{routes, state::AppState};

#[actix_web::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(EnvFilter::from_default_env())
        .with_current_span(false)
        .with_ansi(true)
        .init();

    let app_state = AppState {/* @todo: add db repository */};

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(TracingLogger::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .service(Files::new("/assets", "./packages/website/assets").prefer_utf8(true))
            .service(routes::home::handler)
            .service(
                // Note: More specific routes (/todo/search) should be registered before parameterized routes (/todo/{id}).
                web::scope("/todo"), // .service(routes::todo::create::handler)
            )
    })
    .bind(("127.0.0.1", 5173))? // vite port 😉
    .run()
    .await
}
