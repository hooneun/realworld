use dotenv::dotenv;
use mongodb::bson::Document;
use mongodb::options::{ClientOptions, Compressor};
use mongodb::{Client, Collection, Database};
use std::env;
use std::time::Duration;

pub struct MongoDBConfig {
    pub mongodb_url: String,
    pub connection_timeout: Option<Duration>,
    pub max_pool_size: Option<u32>,
    pub min_pool_size: Option<u32>,
    pub compressors: Option<Vec<Compressor>>,
}

impl MongoDBConfig {
    pub fn from_env() -> MongoDBConfig {
        dotenv().ok();

        MongoDBConfig {
            mongodb_url: env::var("MONGODB_URL").expect("MONGODB_URL must be set"),
            connection_timeout: None,
            min_pool_size: None,
            max_pool_size: None,
            compressors: None,
        }
    }
}

pub struct MongoDB {
    pub database: Database,
    pub collection: Collection<Document>,
}

impl MongoDB {
    pub async fn new(
        config: MongoDBConfig,
        db_name: &'static str,
        collection_name: &'static str,
    ) -> Collection<Document> {
        let mut client_options = ClientOptions::parse(config.mongodb_url.as_str())
            .await
            .unwrap();

        client_options.connect_timeout = config.connection_timeout;
        client_options.max_pool_size = config.max_pool_size;
        client_options.min_pool_size = config.min_pool_size;
        client_options.compressors = config.compressors;

        let client = Client::with_options(client_options).unwrap();
        let database = client.database(db_name);

        database.collection::<Document>(collection_name)
    }
}
