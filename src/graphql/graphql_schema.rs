use super::dataloader::user::UserLoader;
use super::resolver::{comment, post, user};
use async_graphql::dataloader::DataLoader;
use async_graphql::{EmptySubscription, MergedObject, Schema};

#[derive(MergedObject, Default)]
pub struct Query(comment::CommentQuery, user::UserQuery, post::PostQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(
    comment::CommentMutation,
    post::PostMutation,
    user::UserMutation,
);

pub fn generate_schema() -> Schema<Query, Mutation, EmptySubscription> {
    return Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(DataLoader::new(UserLoader(), tokio::spawn))
        .finish();
}
