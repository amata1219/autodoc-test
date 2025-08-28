import os
import math
from pathlib import Path
from typing import Iterable, List, Tuple, Dict

import tiktoken
from openai import OpenAI

try:
    import tomllib  # Python 3.11+
except ModuleNotFoundError:
    tomllib = None  # toml解析はフォールバックで簡易判定

# ========= 設定 =========
INCLUDE_EXTS = {".rs"}
MODEL = os.environ.get("OPENAI_MODEL", "gpt-4o-mini")
SYSTEM_PROMPT = (
    "You are a senior software architecture analyst and documentation specialist. "
    "You infer house coding conventions, architectural patterns, and design tendencies from source code, "
    "and you write clear, actionable Japanese documentation for engineers."
)
MAX_TOKENS_CONTEXT = 12000

# ディレクトリツリー生成の設定
EXCLUDE_DIRS = {
    ".git", "target", ".idea", ".vscode", "node_modules", "dist", "build", ".venv", "__pycache__"
}
ALWAYS_INCLUDE_FILES = {
    "Cargo.toml", "Cargo.lock", "README.md", "LICENSE", "rust-toolchain.toml", ".gitignore"
}
MAX_TREE_DEPTH = 6  # 深すぎるツリーを抑制
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

# ---------- ディレクトリツリー生成 ----------
def _should_include_file(p: Path) -> bool:
    if p.name in ALWAYS_INCLUDE_FILES:
        return True
    if p.suffix in INCLUDE_EXTS:
        return True
    # src/配下の主要ファイルはなるべく含める
    if p.parent.name == "src" and p.suffix:
        return True
    return False

def _listdir_sorted(path: Path) -> List[Path]:
    try:
        entries = list(path.iterdir())
    except Exception:
        return []
    # ディレクトリ優先、アルファベット順
    entries.sort(key=lambda x: (not x.is_dir(), x.name.lower()))
    return entries

def _render_tree(root: Path, prefix: str = "", depth: int = 0) -> List[str]:
    """
    Unicode のツリー記号で表示を整える。
    深さ制限や除外ディレクトリを考慮しつつ、必要最小限のファイルのみ含める。
    """
    if depth > MAX_TREE_DEPTH:
        return [f"{prefix}└── … (truncated)\n"]

    children = []
    for entry in _listdir_sorted(root):
        if entry.is_dir():
            if entry.name in EXCLUDE_DIRS:
                continue
            # ディレクトリが有意（含めたいファイルを内包）かを先読みで判定
            if not _dir_has_includable(entry):
                continue
            children.append(entry)
        else:
            if _should_include_file(entry):
                children.append(entry)

    lines: List[str] = []
    for idx, child in enumerate(children):
        is_last = (idx == len(children) - 1)
        branch = "└── " if is_last else "├── "
        next_prefix = "    " if is_last else "│   "
        display_name = child.name + ("/" if child.is_dir() else "")
        lines.append(f"{prefix}{branch}{display_name}\n")
        if child.is_dir():
            lines.extend(_render_tree(child, prefix + next_prefix, depth + 1))
    return lines

def _dir_has_includable(d: Path, depth: int = 0) -> bool:
    """
    ディレクトリ配下に含めたい何かが存在するか（軽量に判定）。
    """
    if depth > 2:
        return True  # 深追いしすぎない（存在する前提でOK）
    try:
        for e in d.iterdir():
            if e.is_dir():
                if e.name in EXCLUDE_DIRS:
                    continue
                if _dir_has_includable(e, depth + 1):
                    return True
            else:
                if _should_include_file(e):
                    return True
    except Exception:
        return False
    return False

def build_dir_trees(repo_root: Path, crates: List[Path]) -> str:
    """
    各クレートごとにツリーを機械生成し、Markdown 用のコードブロックにまとめる。
    """
    if not crates:
        # 単一クレートでなくても、リポジトリ直下の意義あるものを表示
        header = f"### {repo_root.name}\n\n```text\n{repo_root.name}/\n"
        body = "".join(_render_tree(repo_root))
        return header + body + "```\n"

    sections: List[str] = []
    for crate in crates:
        rel = crate.relative_to(repo_root)
        header = f"### {rel.as_posix()}\n\n```text\n{rel.name}/\n"
        body = "".join(_render_tree(crate))
        sections.append(header + body + "```\n")
    return "\n".join(sections)

# ---------- コンテキスト構築 ----------
def collect_blocks(repo_root: Path) -> Tuple[List[Tuple[str, str]], Dict[str, str], List[Path]]:
    """
    blocks: [(title, content)] を返す（チャンク化前）。title はファイル見出し。
    cargo_texts: crate_path -> Cargo.toml の中身
    crates: 検出したクレートのルート一覧
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

        # src/ 配下（bin/も含む）
        src_dir = crate / "src"
        if src_dir.exists():
            files = sorted(iter_source_files(src_dir, INCLUDE_EXTS))
        else:
            files = []
            print(f"  - {crate} (no src/ found)")

        # build.rs
        build_rs = crate / "build.rs"
        if build_rs.exists():
            files.append(build_rs)

        # examples/, tests/, benches/ も解析対象に追加
        for extra in ("examples", "tests", "benches"):
            extra_dir = crate / extra
            if extra_dir.exists():
                files.extend(sorted(iter_source_files(extra_dir, INCLUDE_EXTS)))

        unique_files = []
        seen = set()
        for f in files:
            try:
                key = f.resolve()
            except Exception:
                key = f
            if key not in seen:
                unique_files.append(f)
                seen.add(key)

        print(f"  - {crate} : {len(unique_files)} source files (including examples/tests/benches/build.rs)")
        for f in unique_files:
            content = read_text_file(f)
            if not content.strip():
                continue
            # リポジトリからの相対パスで見出しを作る
            rel = f.relative_to(repo_root)
            title = f"# {rel.as_posix()}"
            blocks.append((title, content))

    return blocks, cargo_texts, crates

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
    # コーディング規約・設計傾向の推定を強化したプロンプト
    prompt = f"""
次のコード断片を精読し、README作成に直結する分析を**日本語**で出力してください。
目的: ソースから推定される「独自のコーディング規約」および「アーキテクチャ設計の傾向」を抽出します。

[1] 要点サマリ（簡潔）
- 役割/責務、公開API・エントリポイント、設定/重要関数・モジュール
- 実行フロー/使用例（あれば）
- 注意点/制約
- 依存クレートの示唆（あれば）

[2] 推定される独自のコーディング規約（可能な限り）
各規約を次の形式で列挙:
- 規約: <短い見出し>
  説明: <何を/なぜ>
  根拠: <ファイルパス> からの短い引用(<=80字) を最大3件
  例外/補足: <該当すれば>
  確信度: 1-5

（例: 命名・モジュール分割・結果型/エラー処理・所有権/ライフタイム方針・unsafeの可否・非同期/並行・テスト記述・ドキュメンテーションコメント・フォーマッタ/Clippyに従う癖 など）

[3] アーキテクチャ設計の傾向（可能な限り）
- レイヤ/境界（例: domain/application/infrastructure, hexagonal 等）
- 依存方向/モジュール結合の強さ
- 状態管理/不変性の扱い
- エラーハンドリング戦略（Result/thiserror/anyhow 等）
- 非同期/並行（tokio/async-std, send+syncの扱い）
- 型設計/所有権の戦略（newtype/phantom/traitでの抽象化 等）
- テスト戦略（単体/統合/fixtures/プロパティテスト等）
- パフォーマンス/安全性の方針（unsafe/アロケーション削減/キャッシュ 等）
- 採用パターン（DDD/イベント駆動/DI/プラグイン/ECS/状態遷移 など）
必要に応じて各項目に根拠（ファイルパス + 短い引用）と確信度(1-5)を併記。

出力ルール:
- 箇条書き中心。引用は20行以内、1項目で最大3件まで。
- 断片以外の推測は書かない。言い切れない場合は仮説として明記。
- 断片先頭に含まれる「# <ファイルパス>」見出しのパスを根拠に用いて良い。

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
    """
    LLM に README 本文を生成させる。
    ディレクトリ構成はここでは生成させず、プレースホルダ <!-- DIR_TREE --> を入れておく。
    """
    cargo_sections = []
    for crate, txt in cargo_map.items():
        cargo_sections.append(f"### {crate}\n\n```toml\n{txt}\n```")
    cargo_blob = "\n\n".join(cargo_sections)

    user_prompt = f"""
あなたは優れた**ソフトウェアアーキテクト兼テクニカルライター**です。
以下の「要点ノート」と Cargo.toml を統合し、**日本語で分かりやすく詳細な README.md** を生成してください。
特に「ソースコードから推定される独自のコーディング規約・アーキテクチャ設計の傾向」を整理して明示します。

### 厳守事項
- **ディレクトリ構成は LLM で生成しない。** README 内の「## ディレクトリ構成」のプレースホルダ `<!-- DIR_TREE -->` をそのまま残すこと（削除・改変禁止）。
- その他のセクション（概要/セットアップ/ライブラリ/規約/設計/実装のポイント/TODO 等）を作成する。
- コード引用は20行以内に収める。
- 断片ごとの仮説は**本文では統合**し、重複は解消。確信度が低い推測は「仮説」と注記。

### 対象読者
- Rust の基本知識を持つエンジニア

### README の骨子（この順序で出力）
# プロジェクト名（仮で問題ありません）
## 概要
## ディレクトリ構成
<!-- DIR_TREE -->
## セットアップ
## 使用技術・主要ライブラリ
## 推定されるコーディング規約
- 命名/整形/モジュール分割/エラー処理/非同期/所有権/テスト/ドキュメント/CIに関する方針などを箇条書き
- 各項目に簡潔な根拠（ファイルパス）と確信度(1-5)を可能な範囲で付与
## アーキテクチャ設計の傾向
- レイヤ/依存方向/境界/抽象化/状態管理/パターン等を要約
- 必要に応じて図示風の箇条書き（テキストのみ）と根拠・確信度
## 実装のポイント
## 今後の課題 / TODO

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
    blocks, cargo_map, crates = collect_blocks(repo_root)

    if not blocks:
        raise SystemExit(
            "No source files found. "
            "Tips: This script scans all crates detected by Cargo.toml with [package]. "
            "Ensure your Rust crate(s) live under <crate>/(src|examples|tests|benches)/*.rs or build.rs"
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

    # ディレクトリツリーを機械生成
    print("Building directory trees …")
    dir_trees_md = build_dir_trees(repo_root, crates)

    print("Generating README.md …")
    readme_md = generate_readme(client, all_notes, cargo_map)

    # プレースホルダ置換。なければ末尾に追加。
    placeholder = "<!-- DIR_TREE -->"
    if placeholder in readme_md:
        readme_md = readme_md.replace(placeholder, dir_trees_md.strip())
    else:
        # セクションが無い場合に備えて追記
        readme_md = (
            readme_md.rstrip()
            + "\n\n## ディレクトリ構成\n\n"
            + dir_trees_md.strip()
            + "\n"
        )

    Path("README.md").write_text(readme_md, encoding="utf-8")
    print("README.md written.")

if __name__ == "__main__":
    main()
