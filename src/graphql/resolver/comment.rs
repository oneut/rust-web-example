use crate::graphql::objects::Comment;
use crate::prisma;
use async_graphql::{InputObject, Object};

#[derive(Default)]
pub struct CommentQuery;

#[Object]
impl CommentQuery {
    async fn comments(&self) -> Vec<Comment> {
        let client = prisma::new_client().await.unwrap();
        let comments = client.comment().find_many(vec![]).exec().await.unwrap();
        comments
            .into_iter()
            .map(|comment| Comment(comment))
            .collect()
    }
}

#[derive(Default)]
pub struct CommentMutation;

#[Object]
impl CommentMutation {
    async fn create_comment(&self, create_comment_input: CreateCommentInput) -> Comment {
        let client = prisma::new_client().await.unwrap();
        let comment = client
            .comment()
            .create(
                create_comment_input.message,
                create_comment_input.user_id,
                prisma::post::id::equals(create_comment_input.post_id),
                vec![],
            )
            .exec()
            .await
            .unwrap();
        Comment(comment)
    }
}

#[derive(InputObject)]
struct CreateCommentInput {
    message: String,
    user_id: i32,
    post_id: i32,
}
