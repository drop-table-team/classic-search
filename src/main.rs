use actix_web::{web::Data, App, HttpServer};
use api::query;
use database::Database;
use log::{error, info};
use module::Module;
use serde::Deserialize;

pub mod api;
pub mod database;
pub mod module;

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    module_name: String,
    backend_address: String,
}

#[tokio::main]
async fn main() {
    let config = match envy::from_env::<Config>() {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't parse environment variables{}", e);
        }
    };

    info!("Loaded config: {:?}", config);

    let module = Module::new(config.module_name.clone());

    let response = match module.register(&config.backend_address).await {
        Ok(r) => r,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!(
        "Successfuly registered module '{}' on backend '{}'",
        config.module_name, config.backend_address
    );

    let database = match Database::connect(
        response.mongo_address.clone(),
        response.mongo_database.clone(),
        response.mongo_collection.clone(),
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            error!(
                "Couldn't connect to MongoDB on {} with database '{}' and collection: '{}': {}",
                response.mongo_address, response.mongo_database, response.mongo_collection, e
            );
            return;
        }
    };

    let database: &'static Database = Box::leak(Box::new(database));

    info!(
        "Successfuly connected to MongoDB on {} with database '{}' and collection: '{}'",
        response.mongo_address, response.mongo_database, response.mongo_collection
    );

    HttpServer::new(move || App::new().service(query).app_data(Data::new(database)))
        .bind(config.address)
        .unwrap()
        .run()
        .await
        .unwrap();
}
