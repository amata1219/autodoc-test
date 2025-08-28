# netmate-api

## 概要
このプロジェクトは、APIキーの発行、ユーザー登録、メール認証、ログイン、ログアウト、名義の作成および共有数のカウントを行うエンドポイントを提供するWebアプリケーションです。ユーザーのプロフィール情報やタグの管理を行うための機能も含まれています。

## ディレクトリ構成
### .

```text
/
├── src/
│   ├── common/
│   │   ├── api_key/
│   │   │   ├── expiration.rs
│   │   │   ├── key.rs
│   │   │   ├── mod.rs
│   │   │   └── refreshed_at.rs
│   │   ├── auth/
│   │   │   ├── mod.rs
│   │   │   ├── one_time_token.rs
│   │   │   └── password.rs
│   │   ├── consensus/
│   │   │   ├── mod.rs
│   │   │   ├── proposal.rs
│   │   │   └── stability.rs
│   │   ├── email/
│   │   │   ├── address.rs
│   │   │   ├── mod.rs
│   │   │   ├── resend.rs
│   │   │   └── send.rs
│   │   ├── handle/
│   │   │   ├── id.rs
│   │   │   ├── mod.rs
│   │   │   ├── name.rs
│   │   │   └── share_count.rs
│   │   ├── profile/
│   │   │   ├── account_id.rs
│   │   │   ├── birth_year.rs
│   │   │   ├── language.rs
│   │   │   ├── mod.rs
│   │   │   └── region.rs
│   │   ├── session/
│   │   │   ├── cookie.rs
│   │   │   ├── mod.rs
│   │   │   ├── refresh_pair_expiration.rs
│   │   │   ├── refresh_token.rs
│   │   │   ├── session_expiration.rs
│   │   │   ├── session_id.rs
│   │   │   └── session_series.rs
│   │   ├── tag/
│   │   │   ├── hierarchy.rs
│   │   │   ├── language_group.rs
│   │   │   ├── mod.rs
│   │   │   ├── non_top_tag.rs
│   │   │   ├── proposal_operation.rs
│   │   │   ├── redis_tag_info.rs
│   │   │   ├── relation.rs
│   │   │   ├── tag_id.rs
│   │   │   ├── tag_info.rs
│   │   │   ├── tag_name.rs
│   │   │   └── top_tag.rs
│   │   ├── uuid/
│   │   │   ├── mod.rs
│   │   │   ├── uuid4.rs
│   │   │   └── uuid7.rs
│   │   ├── character_count.rs
│   │   ├── cycle.rs
│   │   ├── fallible.rs
│   │   ├── mod.rs
│   │   ├── page.rs
│   │   ├── rating.rs
│   │   ├── token.rs
│   │   ├── turnstile.rs
│   │   └── unixtime.rs
│   ├── endpoints/
│   │   ├── api_key/
│   │   │   ├── dsl.rs
│   │   │   ├── endpoint.rs
│   │   │   ├── interpreter.rs
│   │   │   └── mod.rs
│   │   ├── auth/
│   │   │   ├── creation/
│   │   │   │   ├── sign_up/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── verify_email/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── value.rs
│   │   │   ├── sign_in/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── sign_out/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   ├── handle/
│   │   │   ├── count/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── create/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── delete/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── list/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── rename/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   ├── profile/
│   │   │   ├── language/
│   │   │   │   ├── get/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── set/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── mod.rs
│   │   │   ├── region/
│   │   │   │   ├── set/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   ├── tag/
│   │   │   ├── list/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   ├── proposal/
│   │   │   │   ├── propose/
│   │   │   │   │   ├── dsl/
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── propose.rs
│   │   │   │   │   │   ├── relate_hierarchical_tags.rs
│   │   │   │   │   │   └── validate_topology.rs
│   │   │   │   │   ├── interpreter/
│   │   │   │   │   │   ├── mod.rs
│   │   │   │   │   │   ├── propose.rs
│   │   │   │   │   │   ├── update_tag_list.rs
│   │   │   │   │   │   └── validate_topology.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── withdraw/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── mod.rs
│   │   │   ├── rating/
│   │   │   │   ├── get/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── rate/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   ├── unrate/
│   │   │   │   │   ├── dsl.rs
│   │   │   │   │   ├── endpoint.rs
│   │   │   │   │   ├── interpreter.rs
│   │   │   │   │   └── mod.rs
│   │   │   │   └── mod.rs
│   │   │   ├── search/
│   │   │   │   ├── dsl.rs
│   │   │   │   ├── endpoint.rs
│   │   │   │   ├── interpreter.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── helper/
│   │   ├── redis/
│   │   │   ├── connection.rs
│   │   │   ├── mod.rs
│   │   │   ├── namespace.rs
│   │   │   └── namespaces.rs
│   │   ├── cache.rs
│   │   ├── error.rs
│   │   ├── middleware.rs
│   │   ├── mod.rs
│   │   ├── scylla.rs
│   │   └── test.rs
│   ├── middlewares/
│   │   ├── manage_session/
│   │   │   ├── dsl/
│   │   │   │   ├── authenticate.rs
│   │   │   │   ├── extract_session_info.rs
│   │   │   │   ├── manage_session.rs
│   │   │   │   ├── mitigate_session_theft.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── reauthenticate.rs
│   │   │   │   ├── refresh_session_series.rs
│   │   │   │   ├── update_refresh_token.rs
│   │   │   │   └── update_session.rs
│   │   │   ├── interpreter/
│   │   │   │   ├── authenticate.rs
│   │   │   │   ├── mitigate_session_theft.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── reauthenticate.rs
│   │   │   │   ├── refresh_session_series.rs
│   │   │   │   ├── update_refresh_token.rs
│   │   │   │   └── update_session.rs
│   │   │   ├── middleware.rs
│   │   │   └── mod.rs
│   │   ├── quota_limit/
│   │   │   ├── dsl.rs
│   │   │   ├── interpreter.rs
│   │   │   ├── middleware.rs
│   │   │   └── mod.rs
│   │   ├── rate_limit/
│   │   │   ├── dsl/
│   │   │   │   ├── increment_rate.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── rate_limit.rs
│   │   │   │   └── refresh_api_key.rs
│   │   │   ├── interpreter/
│   │   │   │   ├── increment_rate.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── rate_limit.rs
│   │   │   │   └── refresh_api_key.rs
│   │   │   ├── middleware.rs
│   │   │   └── mod.rs
│   │   ├── start_session/
│   │   │   ├── dsl/
│   │   │   │   ├── assign_refresh_pair.rs
│   │   │   │   ├── assign_session_id.rs
│   │   │   │   ├── mod.rs
│   │   │   │   └── start_session.rs
│   │   │   ├── interpreter/
│   │   │   │   ├── assign_refresh_pair.rs
│   │   │   │   ├── assign_session_id.rs
│   │   │   │   └── mod.rs
│   │   │   ├── middleware.rs
│   │   │   └── mod.rs
│   │   ├── limit.rs
│   │   ├── mod.rs
│   │   └── session.rs
│   ├── startup/
│   │   └── mod.rs
│   ├── translation/
│   │   ├── ja.rs
│   │   ├── mod.rs
│   │   └── us_en.rs
│   ├── lib.rs
│   └── main.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## セットアップ
このプロジェクトをセットアップするには、以下の手順に従ってください。

1. **依存関係のインストール**:
   ```bash
   cargo build
   ```

2. **環境変数の設定**:
   環境変数を使用して、必要な設定を行います。`.env`ファイルを作成し、以下のように設定します。
   ```
   PEPPER=your_secret_pepper
   REDIS_URL=redis://localhost:6379
   SCYLLA_URL=your_scylla_db_url
   TURNSTILE_SECRET=your_turnstile_secret
   ```

3. **データベースの準備**:
   RedisとScyllaDBの接続を設定し、必要なスキーマを作成します。

4. **アプリケーションの起動**:
   ```bash
   cargo run
   ```

## 使用技術・主要ライブラリ
- **axum**: Webフレームワーク
- **serde**: シリアライズ/デシリアライズライブラリ
- **redis**: Redisクライアント
- **scylla**: ScyllaDBクライアント
- **tokio**: 非同期処理のためのランタイム
- **thiserror**: エラーハンドリングを簡素化するためのライブラリ

## 実装のポイント
- **APIキーの発行**: `issue_api_key`関数を使用して、Turnstileトークンを基にAPIキーを発行します。
- **ユーザー登録**: `sign_up`関数でメールアドレスとパスワードを使用して新しいアカウントを作成します。
- **メール認証**: `verify_email`関数を使用して、ユーザーが受信したメールのリンクをクリックすることでメールアドレスを認証します。
- **セッション管理**: `ManageSession`トレイトを使用して、リクエストに対するセッションの認証、再認証、更新を行います。

## 今後の課題 / TODO
- **テストの充実**: 各エンドポイントに対するユニットテストと統合テストを追加する。
- **エラーハンドリングの改善**: より詳細なエラーメッセージを提供し、ユーザー体験を向上させる。
- **ドキュメントの整備**: 各モジュールや関数に対する詳細なドキュメントを追加する。
- **パフォーマンスの最適化**: レート制限やキャッシュの実装を見直し、パフォーマンスを向上させる。

このREADMEは、プロジェクトの概要と使用方法を理解するためのガイドです。詳細な実装や設定については、ソースコードを参照してください。