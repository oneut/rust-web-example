# 構成
- [axum](https://github.com/tokio-rs/axum)
  - Web フレームワーク
- [async-graphql](https://github.com/async-graphql/async-graphql)
  - GraphQL Server ライブラリ
- [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust)
  - ORM

# 実行環境
- コンテナを構築
```
docker-compose up -d
```

- マイグレーションを実行
```
docker-compose exec app cargo prisma db push
```

- データベースに初期データを登録
```
docker-compose exec app cargo seeder
```

- Webアプリケーションを実行
```
docker-compose exec app cargo run
```

ファイル監視して再コンパイルしたい場合は下記

```
docker-compose exec app cargo watch -x run
```

- アクセス
http://localhost:3000

# GraphQL Schema
- ファイルパス
`/resource/graphql`
- スキーマファイル作成
```
docker-compose exec app cargo graphql-sdl
```