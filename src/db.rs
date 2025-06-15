use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> DatabaseConnection {
    let db = Database::connect("sqlite://movies.db?mode=rwc")
        .await
        .expect("Failed to connect to the database");

    db
}
