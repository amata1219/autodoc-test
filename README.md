# netmate-api

## 概要
このプロジェクトは、APIキーの管理、ユーザー認証、セッション管理、タグ評価などの機能を提供するWeb APIです。Rustを使用しており、非同期処理を活用して高いパフォーマンスを実現しています。各機能はモジュール化されており、明確な責任分担がなされています。

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
1. Rustがインストールされていることを確認してください。
2. プロジェクトをクローンします。
   ```bash
   git clone <repository-url>
   cd netmate-api
   ```
3. 依存関係をインストールします。
   ```bash
   cargo build
   ```
4. 環境変数を設定します。`.env`ファイルを作成し、必要な設定を追加してください。
5. サーバーを起動します。
   ```bash
   cargo run
   ```

## 使用技術・主要ライブラリ
- **axum**: Webフレームワーク
- **tokio**: 非同期ランタイム
- **serde**: データシリアライズ/デシリアライズ
- **thiserror**: エラーハンドリング
- **scylla**: NoSQLデータベースクライアント
- **redis**: キャッシュ用のRedisクライアント
- **tracing**: ロギング

## 推定されるコーディング規約
- **構造体の使用**: データを表現するために構造体を使用し、関連するメソッドを実装する。 (`src/common/api_key/mod.rs`, 確信度: 5)
- **エラーハンドリング**: `thiserror`クレートを使用してエラーを定義し、エラーメッセージを日本語で提供する。 (`src/common/api_key/mod.rs`, 確信度: 5)
- **シリアライズ/デシリアライズ**: `serde`を使用して構造体のシリアライズとデシリアライズを行う。 (`src/common/api_key/mod.rs`, 確信度: 5)
- **非同期処理**: `async`/`await`を使用して非同期処理を行う。 (`src/endpoints/auth/creation/sign_up/endpoint.rs`, 確信度: 5)
- **型安全性の重視**: 新しい型を使用して、型安全性を高める。 (`src/common/api_key/mod.rs`, 確信度: 4)
- **モジュール分割**: 機能ごとにモジュールを分割し、明確な責任を持たせる。 (`src/endpoints/auth/creation/mod.rs`, 確信度: 5)

## アーキテクチャ設計の傾向
- **レイヤ/境界**: 機能ごとにモジュールが分かれており、`api_key`, `email`, `auth`, `tag`などの明確な境界が存在。 (確信度: 5)
- **依存方向/モジュール結合の強さ**: モジュール間の依存は明確で、特定の機能に対する依存が強い。 (確信度: 4)
- **状態管理/不変性の扱い**: 構造体は不変性を重視し、必要に応じてメソッドを通じて状態を変更する。 (確信度: 5)
- **エラーハンドリング戦略**: `thiserror`を使用したエラー処理が行われており、エラーの詳細が日本語で提供される。 (確信度: 5)
- **非同期/並行**: メール送信機能やAPIエンドポイントは非同期で実装されている。 (確信度: 4)
- **テスト戦略**: 各機能に対してユニットテストが実装されており、エッジケースを考慮したテストが行われている。 (確信度: 5)

## 実装のポイント
- 各エンドポイントは非同期で実装されており、リクエストの処理が効率的に行われます。
- エラーハンドリングは一貫して行われており、HTTPステータスコードを適切に返す設計がされています。
- モジュール間の依存関係は明確で、テストが容易な構造になっています。

## 今後の課題 / TODO
- ドキュメントの充実化: 各モジュールや関数に対する詳細なドキュメントを追加する。
- パフォーマンスの最適化: 特にデータベースアクセスやキャッシュの利用に関する最適化を検討する。
- セキュリティの強化: APIキーやユーザー情報の管理に関するセキュリティ対策を強化する。