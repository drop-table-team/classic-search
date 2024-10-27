use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Error, options::FindOptions, Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentPreview {
    uuid: String,
    title: String,
    tags: Vec<String>,
    short: String,
}

pub struct Database {
    client: Client,
    database: String,
    collection: String,
}

impl Database {
    pub async fn connect<S: AsRef<str>>(
        uri: S,
        database: String,
        collection: String,
    ) -> Result<Self, Error> {
        let client = Client::with_uri_str(uri).await?;

        client
            .database(&database)
            .run_command(doc! {"ping": 1})
            .await?;

        Ok(Database {
            client,
            database,
            collection,
        })
    }

    fn get_collection(&self) -> Collection<DocumentPreview> {
        self.client
            .database(&self.database)
            .collection::<DocumentPreview>(&self.collection)
    }

    /// Sucht nach Dokumenten, die alle angegebenen Tags enthalten
    pub async fn search_by_tags(&self, tags: &[String]) -> Result<Vec<DocumentPreview>, Error> {
        let collection = self.get_collection();

        // Erstelle einen MongoDB-Query für Tags
        let query = doc! {
            "tags": {
                "$in": tags
            }
        };

        let options = FindOptions::builder()
            .sort(doc! { "title": 1 })
            .projection(doc! {
                "uuid": 1,
                "title": 1,
                "short": 1,
                "tags": 1,
                "_id": 0  // _id wird standardmäßig zurückgegeben, hier explizit ausgeschlossen
            })
            .build();

        let mut cursor = collection.find(query).with_options(options).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    /// Sucht nach Dokumenten, deren Transkription den Suchtext enthält (Case-insensitive)
    pub async fn search_in_text(&self, search_text: &str) -> Result<Vec<DocumentPreview>, Error> {
        let collection = self.get_collection();

        // Erstelle einen MongoDB-Query mit Textsuche
        let query = doc! {
            "$or": [
                {
                "transcription": {
                    "$regex": search_text,
                    "$options": "i"  // Case-insensitive
                },
            }, {
                "short": {
                    "$regex": search_text,
                    "$options": "i"  // Case-insensitive
                }
            }
            ]
        };

        let options = FindOptions::builder()
            .sort(doc! { "title": 1 })
            .projection(doc! {
                "uuid": 1,
                "title": 1,
                "short": 1,
                "tags": 1,
                "_id": 0  // _id wird standardmäßig zurückgegeben, hier explizit ausgeschlossen
            })
            .build();

        let mut cursor = collection.find(query).with_options(options).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    /// Kombinierte Suche nach Tags und Text in der Transkription
    pub async fn search_combined(
        &self,
        search_text: &str,
        tags: &[String],
    ) -> Result<Vec<DocumentPreview>, Error> {
        let collection = self.get_collection();

        // Kombinierter Query mit Tags und Textsuche
        let query = doc! {
            "$and": [
                {
                    "tags": {
                        "$in": tags
                    }
                },
                {
                    "transcription": {
                        "$regex": search_text,
                        "$options": "i"
                    }
                }
            ]
        };

        let options = FindOptions::builder()
            .sort(doc! { "title": 1 })
            .projection(doc! {
                "uuid": 1,
                "title": 1,
                "short": 1,
                "tags": 1,
                "_id": 0  // _id wird standardmäßig zurückgegeben, hier explizit ausgeschlossen
            })
            .build();

        let mut cursor = collection.find(query).with_options(options).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            results.push(doc);
        }

        Ok(results)
    }

    /// Findet alle einzigartigen Tags, die einen bestimmten Text enthalten
    pub async fn find_matching_tags(
        &self,
        search_text: &str,
        limit: i64,
    ) -> Result<Vec<String>, Error> {
        let collection = self.get_collection();

        // Aggregation Pipeline erstellen
        let pipeline = vec![
            // Entpackt das tags Array
            doc! {
                "$unwind": "$tags"
            },
            // Filtert Tags, die den Suchtext enthalten
            doc! {
                "$match": {
                    "tags": {
                        "$regex": search_text,
                        "$options": "i"
                    }
                }
            },
            // Gruppiert die übereinstimmenden Tags
            doc! {
                "$group": {
                    "_id": "$tags"
                }
            },
            doc! {
                "$limit": &limit
            },
        ];

        let mut cursor = collection.aggregate(pipeline).await?;
        let mut matching_tags = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            if let Some(tag) = doc.get("_id").and_then(|t| t.as_str()) {
                matching_tags.push(tag.to_string());
            }
        }

        Ok(matching_tags)
    }
}
