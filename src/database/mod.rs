use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn init_connection() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let check_migrate = sqlx::migrate!("./src/database/migrations")
        .run(&pool)
        .await;

    match check_migrate {
        Ok(_) => println!("Migrated successfully"),
        Err(e) => println!("Error migrating: {}", e)
    }

    pool
}