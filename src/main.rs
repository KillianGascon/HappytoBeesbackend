mod schema;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être définie");

    // Utilisez database_url pour la connexion à la base de données
}