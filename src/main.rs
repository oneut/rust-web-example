use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use rust_web_example::graphql::graphql_schema;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let generated_schema = graphql_schema::generate_schema();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(generated_schema));

    println!("lisiteing on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    schema: Extension<Schema<graphql_schema::Query, graphql_schema::Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:3000")
            .finish(),
    )
}
