use std::env;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use api::{query, query_tags};
use database::Database;
use log::{error, info};
use serde::Deserialize;

pub mod api;
pub mod database;

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    module_name: String,
    backend_address: String,
    mongo_address: String,
    mongo_database: String,
    mongo_collection: String,
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            error!("Couldn't parse environment variables: {}", e);
            return;
        }
    };

    info!("Loaded config: {:?}", config);

    let database = match Database::connect(
        config.mongo_address.clone(),
        config.mongo_database.clone(),
        config.mongo_collection.clone(),
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            error!(
                "Couldn't connect to MongoDB on {} with database '{}' and collection: '{}': {}",
                config.mongo_address, config.mongo_database, config.mongo_collection, e
            );
            return;
        }
    };

    let database: &'static Database = Box::leak(Box::new(database));

    info!(
        "Successfuly connected to MongoDB on {} with database '{}' and collection: '{}'",
        config.mongo_address, config.mongo_database, config.mongo_collection
    );

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .service(query)
            .service(query_tags)
            .app_data(Data::new(database))
            .wrap(cors)
    })
    .bind(config.address)
    .unwrap()
    .run()
    .await
    .unwrap();
}
