# AIエージェントシステム

## 概要
このプロジェクトは、AIエージェントの管理と操作を行うためのフレームワークです。エージェントは、タスクを実行し、メッセージを送受信し、学習セッションを管理することができます。エージェント管理、タスク処理、学習セッション管理を行うAPIを提供します。

## ディレクトリ構成
### .

```text
/
├── src/
│   ├── agent/
│   │   ├── agent.rs
│   │   └── mod.rs
│   ├── domain/
│   │   ├── entities.rs
│   │   ├── mod.rs
│   │   ├── repositories.rs
│   │   └── services.rs
│   ├── interface/
│   │   ├── repositories/
│   │   │   └── sqlx_repository.rs
│   │   └── mod.rs
│   ├── presentation/
│   │   ├── web/
│   │   │   └── api.rs
│   │   └── mod.rs
│   ├── shared/
│   │   ├── config.rs
│   │   ├── error.rs
│   │   └── mod.rs
│   ├── usecase/
│   │   ├── agent_management.rs
│   │   ├── learning_management.rs
│   │   ├── mod.rs
│   │   └── task_management.rs
│   └── main.rs
├── tests/
│   ├── integration/
│   │   └── api_test.rs
│   └── unit/
│       └── agent_management_test.rs
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## セットアップ
このプロジェクトをセットアップするには、以下の手順を実行してください。

1. **リポジトリのクローン**:
   ```bash
   git clone https://github.com/yourusername/ai-agent-system.git
   cd ai-agent-system
   ```

2. **依存関係のインストール**:
   ```bash
   cargo build
   ```

3. **環境変数の設定**:
   環境変数や設定ファイルを使用してアプリケーションの設定を行います。`.env`ファイルを作成し、必要な設定を追加してください。

4. **データベースの設定**:
   PostgreSQLまたはSQLiteを使用する場合は、適切なデータベースをセットアップし、接続情報を環境変数に設定してください。

5. **アプリケーションの起動**:
   ```bash
   cargo run --bin ai-agent
   ```

## 使用技術・主要ライブラリ
- **非同期処理**: `tokio`
- **Webフレームワーク**: `axum`
- **データベース操作**: `sqlx`
- **メッセージング**: `redis`
- **シリアライズ**: `serde`
- **エラー処理**: `thiserror`, `anyhow`
- **ロギング**: `tracing`

## 実装のポイント
- **エントリポイント**: `src/main.rs`にメイン関数があり、アプリケーションの初期化が行われます。
- **公開API**:
  - エージェント管理: エージェントの作成、更新、削除を行うためのAPIを提供します。
  - タスク管理: タスクの作成、更新、削除を行うためのAPIを提供します。
  - 学習管理: 学習セッションの開始、進捗更新、完了を行うためのAPIを提供します。

### 使用例
以下は、エージェントを作成するためのAPI呼び出しの例です。
```rust
let agent = create_agent(agent_config).await?;
```

## 今後の課題 / TODO
- エラーハンドリングの強化: より詳細なエラーメッセージを提供する。
- テストの充実: 各機能に対するユニットテストを追加する。
- ドキュメントの整備: APIの詳細な使用方法をドキュメント化する。
- モック実装の改善: 実際のサービス実装に向けたモックの見直し。