use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init env vars
    dotenv::dotenv().ok();
    // init tracing subscriber
    let tracing = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env());

    if cfg!(debug_assertions) {
        tracing.pretty().init();
    } else {
        tracing.json().init();
    }

    // building address
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);

    // repository
    let repo = get_repo().await.expect("Couldn't get the repository");
    let repo = web::Data::new(repo);
    tracing::info!("Repository initialized");

    // starting the server
    tracing::info!("🚀🚀🚀 Starting Actix server at {}", address);

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(repo.clone())
                .configure(api_lib::health::service)
                .configure(
                    api_lib::v1::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!(
            "🔥🔥🔥 Couldn't start the server in port {}: {:?}",
            port, err
        )
    })
    .run()
    .await
}

async fn get_repo() -> Result<impl api_lib::film_repository::FilmRepository, sqlx::Error> {
    let conn_str =
        std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
    let pool = sqlx::PgPool::connect(&conn_str).await?;
    Ok(api_lib::film_repository::PostgresFilmRepository::new(pool))
}