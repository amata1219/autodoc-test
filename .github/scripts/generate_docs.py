#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
リポジトリの src/ 構造から structure (YAML 文字列) を生成し、
主要概念ごとに:
  1) structure を LLM に渡して関連ファイルパスを JSON 配列で抽出
  2) 該当ファイルを収集・整形して related_codes を作成
  3) related_codes + 概念 を LLM に渡して、誰でもわかる説明の Markdown を生成
  4) .docs/<slug>.md として保存し、mdBook 用の SUMMARY を自動生成

必要な環境変数:
- OPENAI_API_KEY: OpenAI の API キー

任意の環境変数:
- OAI_MODEL_FILES: ファイル抽出用モデル (default: gpt-4o-mini)
- OAI_MODEL_DOCS: ドキュメンテーション用モデル (default: gpt-4o-mini)
- SRC_DIR: 解析対象ディレクトリ (default: src)
- CONCEPTS_FILE: 概念リストファイル (default: .docs/concepts.yaml)
- OUTPUT_DIR: 出力ディレクトリ (default: .docs)
- MAX_CONTEXT_CHARS: related_codes の最大文字数 (default: 120000)
"""

from __future__ import annotations
import os
import re
import sys
import json
import glob
import yaml
import time
import pathlib
import textwrap
import typing as t
import hashlib
import requests

# ------------------ 設定 ------------------
OPENAI_API_KEY = os.environ.get("OPENAI_API_KEY", "")
if not OPENAI_API_KEY:
    print("[ERROR] OPENAI_API_KEY が設定されていません。Actions の Secrets に登録してください。", file=sys.stderr)
    sys.exit(1)

OAI_BASE_URL = os.environ.get("OPENAI_BASE_URL", "https://api.openai.com/v1")
MODEL_FILES = os.environ.get("OAI_MODEL_FILES", "gpt-4o-mini")
MODEL_DOCS = os.environ.get("OAI_MODEL_DOCS", "gpt-4o-mini")
SRC_DIR = os.environ.get("SRC_DIR", "src")
CONCEPTS_FILE = os.environ.get("CONCEPTS_FILE", ".docs/concepts.yaml")
OUTPUT_DIR = os.environ.get("OUTPUT_DIR", ".docs")
MAX_CONTEXT_CHARS = int(os.environ.get("MAX_CONTEXT_CHARS", "120000"))

EXCLUDE_PATTERNS = [
    r"(^|/)tests?(/|$)", r"(^|/)__tests__(/|$)", r"(^|/)spec(s)?(/|$)",
    r"(^|/)e2e(/|$)", r"(^|/)fixtures?(/|$)", r"(^|/)mocks?(/|$)",
    r"(^|/)stories?(/|$)", r"\.story\.", r"\.spec\.", r"\.test\.",
    r"(^|/)scripts?(/|$)", r"(^|/)bench(marks)?(/|$)", r"(^|/)examples?(/|$)",
    r"(^|/)docs?(/|$)", r"(^|/)\.docs(/|$)", r"(^|/)build(/|$)", r"(^|/)dist(/|$)",
]
EXCLUDE_RE = re.compile("|".join(EXCLUDE_PATTERNS), re.IGNORECASE)

ALLOWED_CODE_EXTS = {
    ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs",
    ".py", ".rb", ".go", ".rs", ".java", ".kt", ".kts", ".scala",
    ".php", ".cs", ".cpp", ".c", ".hpp", ".h", ".swift",
    ".sql", ".json", ".yml", ".yaml", ".toml", ".ini", ".env",
}

# -------------- ユーティリティ --------------

def slugify(text: str) -> str:
    s = re.sub(r"[^\w\-\sぁ-んァ-ヶ一-龠]", "", text, flags=re.UNICODE)
    s = s.strip().replace(" ", "-")
    s = re.sub(r"-+", "-", s)
    return s or "concept"


def read_concepts(path: str) -> tuple[str, list[str]]:
    if not os.path.exists(path):
        print(f"[WARN] 概念リスト {path} が見つかりません。スキップします。")
        return ("", [])
    with open(path, "r", encoding="utf-8") as f:
        data = yaml.safe_load(f) or {}
    domain = data.get("domain", "")
    concepts = data.get("concepts", []) or []
    if not concepts:
        print("[WARN] concepts が空です。")
    return (domain, list(concepts))


def build_structure_yaml(src_dir: str) -> str:
    """src ディレクトリ以下の完全なツリーを YAML 文字列として構築"""
    base = pathlib.Path(src_dir)
    if not base.exists():
        print(f"[ERROR] {src_dir} が存在しません。", file=sys.stderr)
        sys.exit(1)

    def tree(p: pathlib.Path):
        if p.is_dir():
            return {child.name: tree(child) for child in sorted(p.iterdir(), key=lambda x: x.name)}
        else:
            return None  # ファイルは None として表現

    structure = {base.name: tree(base)}
    return yaml.safe_dump(structure, sort_keys=False, allow_unicode=True)


def list_all_files(src_dir: str) -> list[str]:
    files = []
    for root, dirs, filenames in os.walk(src_dir):
        for fn in filenames:
            rel = os.path.join(root, fn)
            files.append(rel.replace("\\", "/"))
    return files


def is_code_file(path: str) -> bool:
    ext = os.path.splitext(path)[1].lower()
    return ext in ALLOWED_CODE_EXTS


def exclude_dev_asset(path: str) -> bool:
    return bool(EXCLUDE_RE.search(path))


def ext_to_lang(ext: str) -> str:
    return {
        ".ts": "ts", ".tsx": "tsx", ".js": "js", ".jsx": "jsx", ".mjs": "js", ".cjs": "js",
        ".py": "python", ".rb": "ruby", ".go": "go", ".rs": "rust", ".java": "java",
        ".kt": "kotlin", ".kts": "kotlin", ".scala": "scala", ".php": "php", ".cs": "csharp",
        ".cpp": "cpp", ".c": "c", ".hpp": "cpp", ".h": "c", ".swift": "swift", ".sql": "sql",
        ".json": "json", ".yml": "yaml", ".yaml": "yaml", ".toml": "toml", ".ini": "ini", ".env": "bash",
    }.get(ext.lower(), "")

# -------------- OpenAI 呼び出し --------------

def call_chat(model: str, system_prompt: str, user_prompt: str, temperature: float = 0.2, max_tokens: int | None = None) -> str:
    url = f"{OAI_BASE_URL}/chat/completions"
    headers = {
        "Authorization": f"Bearer {OPENAI_API_KEY}",
        "Content-Type": "application/json",
    }
    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt},
        ],
        "temperature": temperature,
    }
    if max_tokens:
        payload["max_tokens"] = max_tokens

    resp = requests.post(url, headers=headers, data=json.dumps(payload), timeout=120)
    if resp.status_code != 200:
        print("[ERROR] OpenAI API error:", resp.status_code, resp.text, file=sys.stderr)
        raise RuntimeError("OpenAI API error")
    data = resp.json()
    try:
        return data["choices"][0]["message"]["content"].strip()
    except Exception as e:
        print("[ERROR] Unexpected OpenAI response:", data, file=sys.stderr)
        raise


def extract_json_array(text: str) -> list[str]:
    """レスポンステキストから最初の JSON 配列を抽出してパース"""
    m = re.search(r"\[.*\]", text, flags=re.S)
    if not m:
        return []
    raw = m.group(0)
    try:
        arr = json.loads(raw)
        if isinstance(arr, list):
            return [str(x) for x in arr]
    except Exception:
        pass
    return []

# -------------- プロンプト --------------

FILE_PICKER_SYSTEM = (
    """
あなたは熟練のソフトウェアアーキテクトです。与えられたソースツリーの YAML 構造（structure）と対象の概念に基づき、
その概念を実装・表現する「主要コード」のファイルパスのみを **厳選** して抽出してください。

必ず守るルール:
- 出力は **JSON の配列**（文字列の相対パス）**のみ**。説明文は一切不要。
- `src/` 配下のみを対象。
- テスト・サンプル・開発補助のアセットは **除外**（例: `test`, `spec`, `mock`, `fixture`, `stories`, `storybook`, `e2e`, `docs`, `dist`, `build` など）。
- 型定義や設定ファイルは、概念の中核に直接関与する場合のみ含める。
- 関連度が低いユーティリティや再輸出だけの index は可能な限り除外。
- どうしても迷う場合は **最小限のセット** を選ぶ。
"""
).strip()

FILE_PICKER_USER_TMPL = (
    """
# 概念
{concept}

# structure (srcツリーの完全YAML)
{structure}

# 期待する出力形式
["src/feature/a.ts", "src/feature/b.tsx"]
"""
).strip()

DOC_WRITER_SYSTEM = (
    """
あなたは、非エンジニアも読者に含むプロダクト向けテクニカルライターです。与えられた `related_codes` と `概念` から、
プロダクトマネージャー・セールス・顧客にも理解しやすい **日本語の Markdown ドキュメント** を作成します。

執筆ポリシー:
- 専門用語は **平易に言い換え**、必要なら用語集で補足。
- コードの逐次解説ではなく、 **何を実現するものか** → **なぜ重要か** → **どう動くか** の順で説明。
- テストや開発用資産には触れない。
- 読者が最初に知りたいのは価値とフロー。内部詳細は控えめに、要点だけ。
- 可能なら Mermaid のシーケンス図/フローチャートを 1 つ含める。
- 最後に "関連ファイル" として使用したパスを列挙。

出力は **純粋な Markdown** のみ。
"""
).strip()

DOC_WRITER_USER_TMPL = (
    """
# 概念
{concept}

# related_codes（ファイルごとにラベル済み）
{related_codes}

# 期待する Markdown セクション例
# {title}
## 概要
## なぜ重要か
## 基本の流れ
## 主要コンポーネント
## 外部/内部インターフェース
## 例（ユーザー視点のシナリオ）
## 制約と既知の注意点
## 用語集
## 関連ファイル
"""
).strip()

COMPRESSOR_SYSTEM = (
    """
あなたはソフトウェアアーキテクトです。多数のコード断片を、**重要な振る舞い/ルール/データ構造** の箇条書きに圧縮してください。
出力は日本語の Markdown 箇条書きのみ。
"""
).strip()

# -------------- 主要処理 --------------

def ensure_output_dirs():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    os.makedirs(os.path.join(OUTPUT_DIR, "src"), exist_ok=True)


def write_file(path: str, content: str):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def build_related_codes(paths: list[str]) -> tuple[str, list[str]]:
    """ファイルを読み込み、LLM に渡しやすい形へ整形。
    returns: (related_codes_text, used_paths)
    """
    blocks = []
    used = []
    total_len = 0

    for p in paths:
        if not os.path.exists(p):
            continue
        if exclude_dev_asset(p):
            continue
        if not is_code_file(p):
            continue
        ext = os.path.splitext(p)[1]
        lang = ext_to_lang(ext)
        try:
            with open(p, "r", encoding="utf-8", errors="replace") as f:
                code = f.read()
        except Exception:
            continue
        header = f"===== file: {p} ====="
        chunk = f"{header}\n```{lang}\n{code}\n```\n"
        blocks.append(chunk)
        used.append(p)
        total_len += len(chunk)
        if total_len > MAX_CONTEXT_CHARS:
            break

    related = "\n\n".join(blocks)
    return related, used


def compress_if_needed(related: str, concept: str) -> str:
    if len(related) <= MAX_CONTEXT_CHARS:
        return related
    # 文字数超過時は分割し、要点に圧縮
    parts = []
    idx = 0
    step = MAX_CONTEXT_CHARS // 2
    while idx < len(related) and len(parts) < 6:  # 安全のため分割上限
        chunk = related[idx: idx + step]
        idx += step
        uprompt = f"# 概念\n{concept}\n\n# コード断片\n{chunk}"
        summary = call_chat(MODEL_DOCS, COMPRESSOR_SYSTEM, uprompt, temperature=0.1)
        parts.append(summary)
        time.sleep(0.6)
    merged = "\n".join(parts)
    return f"<!-- compressed from large related_codes -->\n{merged}"


def generate_markdown(concept: str, related_codes_text: str, used_paths: list[str]) -> str:
    title = f"{concept}"
    uprompt = DOC_WRITER_USER_TMPL.format(
        concept=concept,
        related_codes=related_codes_text,
        title=title,
    )
    md = call_chat(MODEL_DOCS, DOC_WRITER_SYSTEM, uprompt, temperature=0.25)
    # 関連ファイル追記（念のため）
    tail = "\n\n---\n## 関連ファイル\n" + "\n".join([f"- `{p}`" for p in used_paths]) + "\n"
    if "## 関連ファイル" not in md:
        md = md.rstrip() + tail
    return md


def ensure_mdbook_scaffold(domain: str, concept_files: list[tuple[str, str]]):
    # index
    index_md = textwrap.dedent(f"""
    # {domain or 'プロダクトドキュメント'}

    このサイトはソースコード（Single Source of Truth）から自動生成された、主要概念の読み物です。

    - 生成日時: {time.strftime('%Y-%m-%d %H:%M:%S %Z')}
    - 対象ディレクトリ: `{SRC_DIR}`

    各章は、該当概念に関する関連コードを解析し、非エンジニアにも読みやすい形でまとめています。
    """)
    write_file(os.path.join(OUTPUT_DIR, "src", "index.md"), index_md)

    # book.toml（無ければ作成）
    book_toml_path = os.path.join(OUTPUT_DIR, "book.toml")
    if not os.path.exists(book_toml_path):
        book_toml = textwrap.dedent(
            f"""
            [book]
            title = "{domain or 'Concept Docs'}"
            authors = ["Generated by GitHub Actions"]
            language = "ja"

            [build]
            create-missing = true
            """
        )
        write_file(book_toml_path, book_toml)

    # SUMMARY.md
    lines = ["# Summary", "", "- [トップ](index.md)"]
    for title, filename in concept_files:
        lines.append(f"- [{title}]({filename})")
    write_file(os.path.join(OUTPUT_DIR, "src", "SUMMARY.md"), "\n".join(lines) + "\n")


def main():
    ensure_output_dirs()

    domain, concepts = read_concepts(CONCEPTS_FILE)
    if not concepts:
        print("[INFO] 概念が無いため処理を終了します。")
        # mdBook scaffold だけ作る
        ensure_mdbook_scaffold(domain, [])
        return

    # structure を生成
    structure_yaml = build_structure_yaml(SRC_DIR)
    write_file(os.path.join(OUTPUT_DIR, "_structure.yaml"), structure_yaml)

    concept_files: list[tuple[str, str]] = []

    for concept in concepts:
        print(f"[INFO] Processing concept: {concept}")
        uprompt = FILE_PICKER_USER_TMPL.format(concept=concept, structure=structure_yaml)
        resp = call_chat(MODEL_FILES, FILE_PICKER_SYSTEM, uprompt, temperature=0.1)
        candidates = extract_json_array(resp)

        # フォールバック: structure から単純検索（念のため）
        if not candidates:
            print(f"[WARN] LLM がファイルを返しませんでした。フォールバック検索を試みます。")
            all_files = [p for p in list_all_files(SRC_DIR) if p.startswith(f"{SRC_DIR}/")]
            key = slugify(concept).replace("-", "")
            candidates = [p for p in all_files if key and key.lower() in p.lower()]

        # フィルタリング
        files = []
        for p in candidates:
            p = p.strip().lstrip("./")
            if not p.startswith(f"{SRC_DIR}/"):
                continue
            if exclude_dev_asset(p):
                continue
            if not is_code_file(p):
                continue
            if not os.path.exists(p):
                continue
            files.append(p)
        files = list(dict.fromkeys(files))  # de-dup

        # 関連コード整形
        related, used_paths = build_related_codes(files)
        if not related:
            related = "<!-- no related codes found -->"

        related = compress_if_needed(related, concept)

        # ドキュメント生成
        md = generate_markdown(concept, related, used_paths)

        # 保存
        slug = slugify(concept)
        out_path = os.path.join(OUTPUT_DIR, "src", f"{slug}.md")
        write_file(out_path, md)
        concept_files.append((concept, f"{slug}.md"))
        time.sleep(0.6)

    # mdBook セットアップ
    ensure_mdbook_scaffold(domain, concept_files)


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print("[ERROR] 処理に失敗しました:", e, file=sys.stderr)
        sys.exit(1)
