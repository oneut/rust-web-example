use crate::graphql::dataloader::user::UserLoader;
use crate::prisma;
use async_graphql::dataloader::DataLoader;
use async_graphql::{Context, Object};

pub struct Comment(pub prisma::comment::Data);

#[Object]
impl Comment {
    async fn id(&self) -> i32 {
        self.0.id
    }
    async fn message(&self) -> String {
        self.0.message.to_string()
    }
    async fn post_id(&self) -> i32 {
        self.0.post_id
    }
    async fn user(&self, ctx: &Context<'_>) -> Option<User> {
        let option_user = ctx
            .data_unchecked::<DataLoader<UserLoader>>()
            .load_one(self.0.user_id)
            .await
            .unwrap();
        option_user.map(|user| User(user))
    }
}

pub struct Post(pub prisma::post::Data);

#[Object]
impl Post {
    async fn id(&self) -> i32 {
        self.0.id
    }
    async fn title(&self) -> String {
        self.0.title.to_string()
    }
    async fn comment(&self) -> Vec<Comment> {
        let comments = self.0.comments().unwrap().clone();
        comments
            .into_iter()
            .map(|comment| Comment(comment))
            .collect()
    }
    async fn user(&self, ctx: &Context<'_>) -> Option<User> {
        let option_user = ctx
            .data_unchecked::<DataLoader<UserLoader>>()
            .load_one(self.0.user_id)
            .await
            .unwrap();
        option_user.map(|user| User(user))
    }
}

pub struct User(pub prisma::user::Data);

#[Object]
impl User {
    pub async fn id(&self) -> i32 {
        self.0.id
    }
    pub async fn name(&self) -> String {
        self.0.name.to_string()
    }
}
