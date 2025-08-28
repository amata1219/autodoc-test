import os
import math
from pathlib import Path
from typing import Iterable, List, Tuple

import tiktoken
from openai import OpenAI

# ========= 設定 =========
INCLUDE_EXTS = {".rs"}  # 必要なら {".rs", ".toml", ".md", ".proto", ...} 等に拡張
SRC_DIR = Path("src")
OPTIONAL_FILES = [Path("Cargo.toml")]  # 読める場合のみ取り込み
MODEL = os.environ.get("OPENAI_MODEL", "gpt-4o-mini")
SYSTEM_PROMPT = "You are a helpful assistant that writes clear documentation."
# ざっくり安全側のトークン上限（出力余裕込み）
MAX_TOKENS_CONTEXT = 12000
MAX_TOKENS_OUTPUT = 2000
# =======================

def get_encoder():
    # gpt-4o 系は cl100k_base で概算可
    return tiktoken.get_encoding("cl100k_base")

def count_tokens(text: str) -> int:
    enc = get_encoder()
    return len(enc.encode(text))

def read_text_file(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8", errors="replace")
    except Exception:
        # バイナリ/権限などで読めない場合はスキップ
        return ""

def iter_source_files(root: Path, exts: Iterable[str]) -> Iterable[Path]:
    if not root.exists():
        return []
    for p in root.rglob("*"):
        if p.is_file() and p.suffix in exts:
            yield p

def file_blocks() -> List[Tuple[str, str]]:
    """(title, content) のリストを返す。title は見出し用、content は本文。"""
    blocks: List[Tuple[str, str]] = []

    # src 以下
    for p in sorted(iter_source_files(SRC_DIR, INCLUDE_EXTS)):
        content = read_text_file(p)
        if not content.strip():
            continue
        title = f"# {p.as_posix()}"
        blocks.append((title, content))

    # 任意ファイル（Cargo.toml 等）
    for p in OPTIONAL_FILES:
        if p.exists():
            content = read_text_file(p)
            if content.strip():
                title = f"# {p.as_posix()}"
                blocks.append((title, content))

    return blocks

def chunk_blocks(blocks: List[Tuple[str, str]], max_tokens: int) -> List[str]:
    """ファイル境界を尊重しながらチャンクにまとめる。"""
    chunks: List[str] = []
    cur: List[str] = []
    cur_tokens = 0

    for title, body in blocks:
        unit = f"{title}\n\n{body}\n\n"
        unit_tokens = count_tokens(unit)

        # 単一ファイルが大きすぎる場合は分割
        if unit_tokens > max_tokens:
            # 粗い分割：行で割る
            lines = unit.splitlines(keepends=True)
            approx_ratio = max(1, math.ceil(unit_tokens / max_tokens))
            stride = max(1, len(lines) // approx_ratio)
            for i in range(0, len(lines), stride):
                piece = "".join(lines[i:i+stride])
                piece_tokens = count_tokens(piece)
                if cur_tokens + piece_tokens > max_tokens and cur:
                    chunks.append("".join(cur))
                    cur, cur_tokens = [], 0
                cur.append(piece)
                cur_tokens += piece_tokens
            continue

        # ふつうの詰め込み
        if cur_tokens + unit_tokens > max_tokens and cur:
            chunks.append("".join(cur))
            cur, cur_tokens = [], 0
        cur.append(unit)
        cur_tokens += unit_tokens

    if cur:
        chunks.append("".join(cur))
    return chunks

def summarize_chunk(client: OpenAI, chunk: str) -> str:
    """各チャンクから README 生成に必要な『要点ノート』を抽出（map段階）。"""
    prompt = f"""
次のコード断片（src配下の一部）から、README作成に役立つ要点を日本語で整理してください。
- 役割・エントリポイント・公開API・設定値・重要な関数やモジュール構成
- 使い方の例や実行フロー
- 注意点や制約
- 依存クレートに関する示唆（あれば）

フォーマット：
- 箇条書き中心（セクション見出しOK）
- 必要なコード引用は短く（20行以内）

=== 断片開始 ===
{chunk}
=== 断片終わり ===
""".strip()

    resp = client.chat.completions.create(
        model=MODEL,
        messages=[
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": prompt},
        ],
        temperature=0.2,
    )
    return resp.choices[0].message.content.strip()

def generate_readme(client: OpenAI, notes: str, cargo_toml: str | None) -> str:
    cargo_section = f"\n\n=== Cargo.toml ===\n{cargo_toml}\n" if cargo_toml else ""
    user_prompt = f"""
あなたは優れた技術ドキュメントの専門家です。
以下の「要点ノート」{('と Cargo.toml') if cargo_toml else ''}に基づいて、**日本語で分かりやすく詳細な README.md** を生成してください。

### 要件
- 出力は Markdown（見出し・リスト・コードブロックを適切に使用）
- 対象読者：Rust の基本知識を持つエンジニア
- 以下のセクション（適宜調整可）を含める：
  - プロジェクト概要（1〜2段落）
  - ディレクトリ構成（簡潔なツリー）
  - セットアップ方法（`cargo build` / `cargo run` など）
  - 使用されている主なライブラリや技術（Cargo.toml から推測）
  - 実装のポイント・特徴的なモジュール（必要に応じて短い引用）
  - 今後の課題や TODO（推測でも可）
- コード引用は**過度に長くしない**（20行以内）

=== 要点ノート ===
{notes}
{cargo_section}
""".strip()

    resp = client.chat.completions.create(
        model=MODEL,
        messages=[
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user_prompt},
        ],
        temperature=0.2,
    )
    return resp.choices[0].message.content

def main():
    client = OpenAI(api_key=os.environ["OPENAI_API_KEY"])

    print("Collecting source files...")
    blocks = file_blocks()
    if not blocks:
        raise SystemExit("No source files found under src/")

    # Cargo.toml をあとで最終プロンプトに入れる用（任意）
    cargo_txt = ""
    for title, body in blocks:
        if title.startswith("# Cargo.toml"):
            cargo_txt = body
            break

    print(f"Total files considered: {len(blocks)}")

    # チャンク化（ゆとりを持って 1 チャンク ~ MAX_TOKENS_CONTEXT）
    chunks = chunk_blocks(blocks, MAX_TOKENS_CONTEXT)
    print(f"Chunks: {len(chunks)}")

    # map: 各チャンクを要約ノートに
    notes_list: List[str] = []
    for i, ch in enumerate(chunks, 1):
        print(f"Summarizing chunk {i}/{len(chunks)} ...")
        summary = summarize_chunk(client, ch)
        notes_list.append(f"## Chunk {i}\n{summary}")

    all_notes = "\n\n".join(notes_list)

    # reduce: ノートを元に最終 README を生成
    print("Generating README.md ...")
    readme_md = generate_readme(client, all_notes, cargo_txt)

    Path("README.md").write_text(readme_md, encoding="utf-8")
    print("README.md written.")

if __name__ == "__main__":
    main()
