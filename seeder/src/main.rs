use rust_web_example::prisma;

#[tokio::main]
async fn main() {
    let client = prisma::new_client().await.unwrap();
    client.comment().delete_many(vec![]).exec().await.unwrap();
    client.post().delete_many(vec![]).exec().await.unwrap();
    client.user().delete_many(vec![]).exec().await.unwrap();

    let john = client
        .user()
        .create("john".to_string(), vec![])
        .exec()
        .await
        .unwrap();
    let smith = client
        .user()
        .create("smith".to_string(), vec![])
        .exec()
        .await
        .unwrap();

    let post1 = client
        .post()
        .create("First Post.".to_string(), john.id, vec![])
        .exec()
        .await
        .unwrap();

    client
        .comment()
        .create_many(vec![
            prisma::comment::create_unchecked(
                "This is first message.".to_string(),
                john.id,
                post1.id,
                vec![],
            ),
            prisma::comment::create_unchecked(
                "This is second message.".to_string(),
                smith.id,
                post1.id,
                vec![],
            ),
        ])
        .exec()
        .await
        .unwrap();

    let post2 = client
        .post()
        .create("Second post.".to_string(), smith.id, vec![])
        .exec()
        .await
        .unwrap();
    client
        .comment()
        .create_many(vec![
            prisma::comment::create_unchecked(
                "This is third message.".to_string(),
                john.id,
                post2.id,
                vec![],
            ),
            prisma::comment::create_unchecked(
                "This is forth message.".to_string(),
                smith.id,
                post2.id,
                vec![],
            ),
        ])
        .exec()
        .await
        .unwrap();
}
