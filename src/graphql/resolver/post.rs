use crate::graphql::objects::Post;
use crate::prisma;
use async_graphql::*;

#[derive(Default)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn posts(&self) -> Vec<Post> {
        let client = prisma::new_client().await.unwrap();
        let posts = client
            .post()
            .find_many(vec![])
            .with(prisma::post::comments::fetch(vec![]))
            .exec()
            .await
            .unwrap();
        posts.into_iter().map(|post| Post(post)).collect()
    }
}

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    async fn create_post(&self, create_post_input: CreatePostInput) -> Post {
        let client = prisma::new_client().await.unwrap();
        let post = client
            .post()
            .create(create_post_input.title, create_post_input.user_id, vec![])
            .exec()
            .await
            .unwrap();
        Post(post)
    }
}

#[derive(InputObject)]
struct CreatePostInput {
    title: String,
    user_id: i32,
}
