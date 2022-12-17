use crate::prisma;
use async_graphql::dataloader::*;
use async_graphql::*;
use std::collections::HashMap;

pub struct UserLoader();

#[async_trait::async_trait]
impl Loader<i32> for UserLoader {
    type Value = prisma::user::Data;
    type Error = FieldError;

    async fn load(&self, ids: &[i32]) -> Result<HashMap<i32, prisma::user::Data>> {
        let client = prisma::new_client().await.unwrap();
        let users = client
            .user()
            .find_many(vec![prisma::user::id::in_vec(ids.to_vec())])
            .exec()
            .await
            .unwrap();
        let user_hash_map = users
            .into_iter()
            .map(|user| (user.id, user))
            .collect::<HashMap<_, _>>();
        Ok(user_hash_map)
    }
}
