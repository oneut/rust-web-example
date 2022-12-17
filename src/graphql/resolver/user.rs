use crate::graphql::objects::User;
use crate::prisma;
use async_graphql::{InputObject, Object};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self) -> Vec<User> {
        let client = prisma::new_client().await.unwrap();
        let users = client.user().find_many(vec![]).exec().await.unwrap();
        users.into_iter().map(|user| User(user)).collect()
    }
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, create_user_input: CreateUserInput) -> User {
        let client = prisma::new_client().await.unwrap();
        let user = client
            .user()
            .create(create_user_input.name, vec![])
            .exec()
            .await
            .unwrap();
        User(user)
    }
}

#[derive(InputObject)]
struct CreateUserInput {
    name: String,
}
