# inpedia

> 引用の電子辞書 + CMS。引いたらパッと出る。

ローカル完結のセマンティック検索付き引用管理ツール。  
CLI と Tauri GUI の両方で動作します。

---

## 概要

- **引用を登録**して、著者・出典・タグ・メモを紐付け
- **自然言語で検索**（fastembed + multilingual-e5-small によるローカル埋め込み）
- **メモの版管理**（追記のみ、削除禁止）
- **画像・動画をメモ内にインライン展開**（`{{img:hash}}` / `{{vid:hash}}` 記法）
- すべてローカル完結。サーバー不要、オフライン動作

---

## データベース構造

`~/.inpedia/inpedia.db`（SQLite）に以下のテーブルを自動作成します。

```sql
quotes          -- 引用（不変）
memo_versions   -- メモ版管理（追記のみ）
assets          -- メディアファイル管理
```

メディアファイルは `~/.inpedia/assets/` に保存されます。

---

## インストール

### 前提条件

- Rust 1.75+
- Node.js 18+（GUI のみ）
- [Tauri の依存関係](https://tauri.app/start/prerequisites/)（GUI のみ）

### CLI ビルド

```sh
git clone https://github.com/haruaki-nayuta/inpedia
cd inpedia
cargo build --release -p inpedia
```

バイナリは `target/release/inpedia` に生成されます。  
パスを通すか、`cargo install --path inpedia` でインストールしてください。

### GUI ビルド

```sh
cd inpedia-app/frontend
npm install
cd ..
cargo tauri build
```

---

## CLI 使い方

### 引用を登録

```sh
inpedia add
```

対話形式で以下を入力します：
- 引用テキスト（必須）
- 著者名
- 出典タイトル
- URL
- タグ（カンマ区切り）
- メモ（エディタが開きます）

embedding は自動生成されます（初回は multilingual-e5-small モデルをダウンロード）。

### セマンティック検索

```sh
inpedia search "認知の歪みについて"
inpedia search "自由意志" --top 10
```

自然言語クエリで意味的に近い引用を返します。

### メモを更新

```sh
inpedia update <id>
```

エディタが開き、編集後に新バージョンとして保存されます（旧版は削除されません）。

### 一覧表示

```sh
inpedia list
```

### タグで絞り込み

```sh
inpedia tag "哲学"
```

### 版の変遷を確認

```sh
inpedia history <id>
```

メモの各版を差分（追加: 緑、削除: 赤）付きで表示します。

---

## GUI 使い方

```sh
cargo tauri dev
```

- **検索タブ**: 検索窓にキーワードを入力するとリアルタイムでカード表示
- **一覧タブ**: 全引用を表示。タグで絞り込み可能
- **+ 追加ボタン**: フォームから引用を登録
- **カードをクリック**: メモの版管理（差分表示）を開く

---

## プロジェクト構成

```
inpedia/
├── Cargo.toml              # workspace
├── inpedia-core/           # コアライブラリ（DB・embedding・検索）
│   └── src/
│       ├── db.rs           # SQLite CRUD
│       ├── embedding.rs    # fastembed ラッパー
│       ├── models.rs       # データモデル
│       └── search.rs       # コサイン類似度検索
├── inpedia/                # CLI バイナリ
│   └── src/commands/
│       ├── add.rs
│       ├── search.rs
│       ├── update.rs
│       ├── list.rs
│       ├── tag.rs
│       └── history.rs
└── inpedia-app/            # Tauri GUI
    ├── frontend/           # React + TypeScript
    │   └── src/
    │       ├── App.tsx
    │       └── components/ # QuoteCard, DiffView, AddForm
    └── src-tauri/          # Rust バックエンド
```

---

## 技術スタック

| レイヤー | 技術 |
|---|---|
| DB | SQLite（`rusqlite` bundled） |
| Embedding | `fastembed` 5.x — `intfloat/multilingual-e5-small` |
| 検索 | インメモリ コサイン類似度 |
| CLI | `clap` 4 + `dialoguer` + `colored` |
| GUI フレーム | Tauri 2 |
| フロントエンド | React + TypeScript（Vite） |
| 差分表示 | `similar`（Rust）/ `diff`（JS） |

---

## ライセンス

MIT
