import os
import math
from pathlib import Path
from typing import Iterable, List, Tuple, Dict, Optional

import tiktoken
from openai import OpenAI

try:
    import tomllib  # Python 3.11+
except ModuleNotFoundError:
    tomllib = None  # toml解析はフォールバックで簡易判定

# ========= 設定 =========
INCLUDE_EXTS = {".rs"}
MODEL = os.environ.get("OPENAI_MODEL", "gpt-4o-mini")
SYSTEM_PROMPT = "You are a helpful assistant that writes clear documentation."
MAX_TOKENS_CONTEXT = 12000
# =======================


# ---------- ユーティリティ ----------
def get_encoder():
    return tiktoken.get_encoding("cl100k_base")

def count_tokens(text: str) -> int:
    enc = get_encoder()
    return len(enc.encode(text))

def read_text_file(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8", errors="replace")
    except Exception:
        return ""

def is_package_cargo(cargo_path: Path) -> bool:
    """
    Cargo.toml が [package] を持つなら True。
    tomllib が使えない場合は素朴に文字列検索。
    """
    txt = read_text_file(cargo_path)
    if not txt.strip():
        return False
    if tomllib:
        try:
            data = tomllib.loads(txt)
            return "package" in data
        except Exception:
            pass
    # フォールバック: 雑にセクション名を検索
    return "[package]" in txt

def find_crates(repo_root: Path) -> List[Path]:
    """
    リポジトリ配下のクレート（[package] を持つ Cargo.toml の親ディレクトリ）を列挙。
    target/ や .git/ は除外。
    """
    crates: List[Path] = []
    for cargo in repo_root.rglob("Cargo.toml"):
        # 除外ディレクトリ
        parts = set(cargo.parts)
        if "target" in parts or ".git" in parts:
            continue
        if is_package_cargo(cargo):
            crates.append(cargo.parent)
    return sorted(set(crates))

def iter_source_files(root: Path, exts: Iterable[str]) -> Iterable[Path]:
    for p in root.rglob("*"):
        if p.is_file() and p.suffix in exts:
            yield p

# ---------- コンテキスト構築 ----------
def collect_blocks(repo_root: Path) -> Tuple[List[Tuple[str, str]], Dict[str, str]]:
    """
    blocks: [(title, content)] を返す（チャンク化前）。title はファイル見出し。
    cargo_texts: crate_path -> Cargo.toml の中身
    """
    blocks: List[Tuple[str, str]] = []
    cargo_texts: Dict[str, str] = {}

    crates = find_crates(repo_root)
    print(f"Detected crates: {len(crates)}")
    if not crates:
        print("No crates found (no Cargo.toml with [package]).")

    for crate in crates:
        cargo_path = crate / "Cargo.toml"
        cargo_txt = read_text_file(cargo_path)
        if cargo_txt:
            cargo_texts[str(crate)] = cargo_txt

        src_dir = crate / "src"
        if not src_dir.exists():
            print(f"  - {crate} (no src/ found)")
            continue

        files = sorted(iter_source_files(src_dir, INCLUDE_EXTS))
        print(f"  - {crate} : {len(files)} source files")
        for f in files:
            content = read_text_file(f)
            if not content.strip():
                continue
            # リポジトリからの相対パスで見出しを作る
            rel = f.relative_to(repo_root)
            title = f"# {rel.as_posix()}"
            blocks.append((title, content))

    return blocks, cargo_texts

def chunk_blocks(blocks: List[Tuple[str, str]], max_tokens: int) -> List[str]:
    chunks: List[str] = []
    cur: List[str] = []
    cur_tokens = 0

    for title, body in blocks:
        unit = f"{title}\n\n{body}\n\n"
        unit_tokens = count_tokens(unit)

        if unit_tokens > max_tokens:
            # 大きすぎる単一ファイルは行で割る
            lines = unit.splitlines(keepends=True)
            approx_ratio = max(1, math.ceil(unit_tokens / max_tokens))
            stride = max(1, len(lines) // approx_ratio)
            buf, buf_tokens = [], 0
            for i in range(0, len(lines), stride):
                piece = "".join(lines[i:i+stride])
                piece_tokens = count_tokens(piece)
                if cur_tokens + piece_tokens > max_tokens and cur:
                    chunks.append("".join(cur))
                    cur, cur_tokens = [], 0
                cur.append(piece)
                cur_tokens += piece_tokens
            continue

        if cur_tokens + unit_tokens > max_tokens and cur:
            chunks.append("".join(cur))
            cur, cur_tokens = [], 0
        cur.append(unit)
        cur_tokens += unit_tokens

    if cur:
        chunks.append("".join(cur))
    return chunks

# ---------- OpenAI 呼び出し ----------
def summarize_chunk(client: OpenAI, chunk: str) -> str:
    prompt = f"""
次のコード断片から、README作成に役立つ要点を日本語で整理してください。
- 役割・エントリポイント・公開API・設定・重要関数/モジュール
- 実行フローや使用例
- 注意点/制約
- 依存クレートの示唆（あれば）
形式: 箇条書き中心、引用は20行以内。

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

def generate_readme(client: OpenAI, notes: str, cargo_map: Dict[str, str]) -> str:
    cargo_sections = []
    for crate, txt in cargo_map.items():
        cargo_sections.append(f"### {crate}\n\n```toml\n{txt}\n```")
    cargo_blob = "\n\n".join(cargo_sections)

    user_prompt = f"""
あなたは優れた技術ドキュメントの専門家です。
以下の「要点ノート」と Cargo.toml を基に、**日本語で分かりやすく詳細な README.md** を生成してください。

### 要件
- Markdown（見出し/リスト/コードブロック）
- 対象読者：Rust の基本知識を持つエンジニア
- 含める内容：
  - プロジェクト概要（1〜2段落）
  - ディレクトリ構成（ツリー形式で簡潔に）
  - セットアップ方法（`cargo build` / `cargo run` など）
  - 使用されている主なライブラリや技術（Cargo.toml から推測）
  - 実装のポイント・特徴的なモジュール（必要に応じ短い引用）
  - 今後の課題や TODO
- コード引用は20行以内にとどめる

=== 要点ノート ===
{notes}

=== Cargo.toml（クレート別）===
{cargo_blob if cargo_blob else '(該当なし)'}
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

# ---------- メイン ----------
def main():
    client = OpenAI(api_key=os.environ["OPENAI_API_KEY"])
    repo_root = Path(".").resolve()

    print(f"Repository root: {repo_root}")
    blocks, cargo_map = collect_blocks(repo_root)

    if not blocks:
        raise SystemExit(
            "No source files found. "
            "Tips: This script scans all crates detected by Cargo.toml with [package]. "
            "Ensure your Rust crate(s) live under <crate>/src/*.rs"
        )

    print(f"Total files considered: {len(blocks)}")

    chunks = chunk_blocks(blocks, MAX_TOKENS_CONTEXT)
    print(f"Chunks created: {len(chunks)}")

    notes_list: List[str] = []
    for i, ch in enumerate(chunks, 1):
        print(f"Summarizing chunk {i}/{len(chunks)}")
        summary = summarize_chunk(client, ch)
        notes_list.append(f"## Chunk {i}\n{summary}")

    all_notes = "\n\n".join(notes_list)

    print("Generating README.md …")
    readme_md = generate_readme(client, all_notes, cargo_map)
    Path("README.md").write_text(readme_md, encoding="utf-8")
    print("README.md written.")

if __name__ == "__main__":
    main()
