use super::database::Database;
use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use log::error;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct TagQuery {
    hint: String,
    limit: usize,
}

#[get("/query_tags")]
async fn query_tags(
    database: Data<&'static Database>,
    tag_query: web::Query<TagQuery>,
) -> impl Responder {
    let tag_query = tag_query.into_inner();

    let tags = match database
        .find_matching_tags(&tag_query.hint, tag_query.limit as i64)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            error!("Couldn't execute tag search for {}: {}", tag_query.hint, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(json!({"tags": tags}))
}

#[derive(Debug, Deserialize)]
struct Query {
    query: Option<String>,
    tags: Vec<String>,
}

#[post("/query")]
async fn query(database: Data<&'static Database>, query: Json<Query>) -> HttpResponse {
    let query = query.into_inner();

    let response = if !query.tags.is_empty() && query.query.is_some() {
        database
            .search_combined(query.query.as_ref().unwrap(), &query.tags)
            .await
    } else if query.query.is_some() {
        database.search_in_text(query.query.as_ref().unwrap()).await
    } else {
        database.search_by_tags(&query.tags).await
    };

    let documents = match response {
        Ok(d) => d,
        Err(e) => {
            error!("Couldn't execute query {:?}: {}", query, e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(json!({
        "documents": documents
    }))
}
