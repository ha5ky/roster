use serde::Deserialize;
use sqlx::mysql::MySqlPoolOptions;
use std::fs;

#[derive(Debug, Deserialize)]
struct SourceData {
    data: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    id: String,
    title: String,
    preview_image_url: String,
    poster_url: String,
    orientation: String,
    card_type: String,
    card_effect: String,
    background: String,
    aspect_ratio: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load .env
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);

    // 2. Connect to DB
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 3. Create table `roster`
    println!("Dropping table 'roster' if exists to ensure schema matches...");
    sqlx::query("DROP TABLE IF EXISTS roster")
        .execute(&pool)
        .await?;

    println!("Creating table 'roster'...");
    sqlx::query(
        r#"
        CREATE TABLE roster (
            id BIGINT PRIMARY KEY AUTO_INCREMENT,
            template_id VARCHAR(50) NOT NULL UNIQUE,
            title VARCHAR(255),
            preview_image_url VARCHAR(500),
            model_url VARCHAR(500),
            poster_url VARCHAR(500),
            orientation VARCHAR(50),
            card_type VARCHAR(100),
            card_effect VARCHAR(100),
            background VARCHAR(100),
            aspect_ratio VARCHAR(50)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // 4. Read JSON file
    println!("Reading t.json...");
    let content = fs::read_to_string("t.json")?;
    let source: SourceData = serde_json::from_str(&content)?;

    // 5. Insert data
    println!("Inserting {} items...", source.data.len());
    for item in source.data {
        sqlx::query(
            r#"
            INSERT INTO roster (
                template_id, title, preview_image_url, poster_url, 
                orientation, card_type, card_effect, background, aspect_ratio
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                title = VALUES(title),
                preview_image_url = VALUES(preview_image_url),
                poster_url = VALUES(poster_url),
                orientation = VALUES(orientation),
                card_type = VALUES(card_type),
                card_effect = VALUES(card_effect),
                background = VALUES(background),
                aspect_ratio = VALUES(aspect_ratio)
            "#,
        )
        .bind(&item.id) // map id -> template_id
        .bind(&item.title)
        .bind(&item.preview_image_url)
        .bind(&item.poster_url)
        .bind(&item.orientation)
        .bind(&item.card_type)
        .bind(&item.card_effect)
        .bind(&item.background)
        .bind(&item.aspect_ratio)
        .execute(&pool)
        .await?;

        println!("Inserted/Updated: {}", item.id);
    }

    println!("Done!");
    Ok(())
}
