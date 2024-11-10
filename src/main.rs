mod api;
mod conf;
mod graphql;

use crate::api::graphql::index_graphql;
use crate::conf::conf;
use crate::graphql::hello_world::HelloWorldQuery;
use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let c = conf();

    HttpServer::new(|| {
        let schema = Schema::build(HelloWorldQuery, EmptyMutation, EmptySubscription).finish();

        App::new()
            .service(web::scope("/graphql").route("", web::get().to(index_graphql)))
            .route("/", web::post().to(GraphQL::new(schema)))
    })
    .bind((c.host.as_str(), c.port))?
    .run()
    .await
}
