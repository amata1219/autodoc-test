import os
from openai import OpenAI

client = OpenAI(api_key=os.environ["OPENAI_API_KEY"])

def read_code_files():
    code = ""
    for root, dirs, files in os.walk("src"):
        for f in files:
            if f.endswith(".rs"):
                with open(os.path.join(root, f)) as file:
                    code += f"# {f}\n" + file.read() + "\n\n"
    return code

prompt = f"""
あなたは優れた技術ドキュメントの専門家です。
以下のRustコードベースに基づいて、**日本語で分かりやすく詳細なREADME.mdを生成してください**。

### 要件:
- 出力はMarkdown形式にしてください
- 対象読者はRustの基本知識を持つエンジニアを想定
- 見出しやリストを使って構造的で読みやすい文書にしてください
- 内容は以下のセクションを含んでください（必要に応じてカスタマイズ可）:

---

# プロジェクト概要

プロジェクトの目的や機能、特徴を1〜2段落で記述

## ディレクトリ構成（必要に応じて）

簡単なファイル・フォルダ構造を示す

## セットアップ方法

Rust環境のセットアップとプロジェクトのビルド／実行手順

```bash
# インストール例
cargo build
cargo run
````

## 使用されている主なライブラリや技術

Cargo.toml から推測できる依存ライブラリやツールを説明

## 実装のポイント・特徴的なモジュール

このコードベースの中で重要なポイント・関数・構成などがあれば記述

## 今後の課題やTODO（もしあれば）

---

以下がコードベース全体です：

{read_code_files()}
"""

response = client.chat.completions.create(
    model="gpt-4",
    messages=[
        {"role": "system", "content": "You are a helpful assistant that writes clear documentation."},
        {"role": "user", "content": prompt}
    ]
)

content = response.choices[0].message.content

os.makedirs("docs", exist_ok=True)
with open("docs/generated.md", "w") as f:
    f.write(content)
