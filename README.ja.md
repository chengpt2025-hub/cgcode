<div align="center">

<img src="https://raw.githubusercontent.com/chengpt2025-hub/cgcode/v2/desktop/overlay/apps/desktop/resources/icon.png" width="120" alt="cgcode logo" />

# cgcode

### 自分のモデルで動く、中国語ユーザーのための AI コーディングエージェント & オールインワン・デスクトップアシスタント

[简体中文](README.md) | [繁體中文](README.zh-TW.md) | [English](README.en.md) | **日本語** | [한국어](README.ko.md)

[ダウンロード（ターミナル版）](https://github.com/chengpt2025-hub/cgcode/releases) · [主な特長](#主な特長) · [cgcode を選ぶ理由](#cgcode-を選ぶ理由) · [ロードマップ](#ロードマップ) · [よくある質問](#よくある質問)

</div>

---

## ひとことで

cgcode は、**好きな大規模言語モデルで使える AI アシスタント**です。世界トップクラスのオープンソース・エージェント基盤（OpenAI Codex と DeepSeek Harness）を、**完全中国語化・設定不要・アカウント不要・テレメトリなし**の製品へと作り直しました。開発者向けの**ターミナル版**と、すべての人向けの**デスクトップクライアント**を提供します。

| | ターミナル版（v1） | デスクトップクライアント（v2） |
|---|---|---|
| 対象 | 開発者 | すべての人：学生、会社員、クリエイター、チーム |
| できること | プロジェクト理解、コード作成、ファイル編集、コマンド実行、コードレビュー、Git 管理 | チャット、Web 検索、文書・表計算・スライド作成、画像生成、コーディング、マルチエージェント協調、定期タスク |
| 使い方 | ターミナルで実行 | ダブルクリックでインストール、ウィンドウ UI |
| ブランチ | `main` | `v2` |
| 状態 | 検収版リリース済み | 検収版ダウンロード可能 |

---

## 主な特長

### 1. モデルを自由に選べる
- 既定では SenseTime の **SenseNova（日日新）** に接続済みで、すぐに使えます。
- DeepSeek、Qwen、Kimi など、**OpenAI 互換**のあらゆるエンドポイントや中継サービスに対応。
- ターミナル版は **Chat Completions ブリッジを内蔵**。上流の Codex は Chat 対応を削除し Responses API のみになりましたが、cgcode なら Chat のみ対応のモデルでもエージェントを完全に動かせます。
- デスクトップ版は UI から**いつでもモデルを切り替え**られます。

### 2. 本格的な中国語対応
- UI、スラッシュコマンドの説明、ウェルカム画面、ステータス画面、キーヒント、既定の返答まですべて簡体字中国語。
- プリインストールのスキルも中国語で書かれ、自然な中国語の言い回しで起動します。

### 3. 導入のハードルなし
- 初回起動時の**中国語セットアップウィザード**でエンドポイント・キー・モデル名を入力すると、**接続を自動テスト**。設定ファイルの編集は不要です。
- デスクトップ版はウェルカム画面で API キーを 1 つ入れるだけ。
- ChatGPT や Claude など海外アカウントは不要です。

### 4. プライバシー重視
- API キーは自分の PC にだけ保存。
- 会話は自分で設定したモデルサービスにのみ送信。
- 利用データ送信、フィードバック送信、セッションテレメトリ、公式アップデート確認は**既定ですべてオフ**。

### 5. 安定性
- レート制限（429）時は**自動で素早く再試行**。
- **予備エンドポイント**を設定すれば、メインが失敗したとき自動で切り替え。

### 6. すぐ使えるスキル
- ターミナル版は中国語の開発スキル **6 個**、デスクトップ版は **23 個**（自社開発の中国語スキル 11 個 + 人気オープンソーススキル 12 個）を同梱。
- デスクトップ版には**中国語スキルマーケット**を内蔵。ひとことで skills.sh、ClawHub、SkillHub を検索し、結果を中国語で表示、**安全チェック後にワンクリックでインストール**。
- 同梱スキルはすべてライセンス確認とセキュリティ審査済み。**脱獄（ジェイルブレイク）系スキルは採用せず**、キーやアカウントの停止リスクから守ります。

### 7. 美しいデザイン
- **iOS 風デザイン**：角丸、ゆとりある余白、iOS システムカラー。ターミナル版は 7 種類のテーマを `/theme` でライブプレビュー切り替え。
- 公式の **10 秒起動アニメーション**：地球 → ワームホール → 機械トンネル → 閃光 → CG CODE / SYSTEM ONLINE、躍動感ある音楽付き。

### 8. オールインワン + マルチエージェント（デスクトップ版）
- チャット、検索、Word / Excel / PowerPoint、画像、コーディングを 1 つのアプリで。
- Grok に倣った**マルチエージェント協調**：複雑なタスクを複数のエージェントに分担して並行処理。
- **定期タスク**と**プラグイン**で拡張可能。

---

## cgcode を選ぶ理由

| | cgcode | 公式 Codex CLI | ChatGPT / Claude アプリ |
|---|---|---|---|
| 使えるモデル | OpenAI 互換の任意のモデル、既定は SenseNova | 主に OpenAI モデル、Responses API のみ | 自社モデルのみ |
| Chat のみ対応のモデル（中国系に多い） | 対応（ブリッジ内蔵） | 非対応 | 非対応 |
| UI 言語 | 完全中国語 | 英語 | 多言語 |
| アカウント | 不要、自分のキーだけ | OpenAI アカウントまたはキー | 海外アカウント |
| 初回設定 | 中国語ウィザード + 接続テスト | 手動設定 | アカウントでログイン |
| 利用データ送信 | 既定ですべてオフ | 提供元の設定による | 提供元のポリシーによる |
| 中国語スキル同梱 | 6 個（ターミナル）/ 23 個 + スキルマーケット（デスクトップ） | なし | なし |
| 非開発者向けデスクトップアプリ | あり | なし | あり |

---

## クイックスタート

### ターミナル版

1. [Releases](https://github.com/chengpt2025-hub/cgcode/releases) から最新の `cgcode-windows-x64.zip`（約 186 MB）をダウンロードし、`D:\cgcode` などに展開します。
2. ターミナルまたは PowerShell でプロジェクトフォルダに移動し、`D:\cgcode\cgcode.exe` を実行します。
   - まだコード署名されていないため、「PC を保護しました」と表示されたら **詳細情報 → 実行** をクリックしてください。
3. 起動アニメーション（任意のキーでスキップ）の後、ウィザードに入力します：
   - ① エンドポイント（例：`https://token.sensenova.cn/v1`）
   - ② API キー
   - ③ モデル名（例：`sensenova-6.8-flash`）
4. 接続テストに通ればすぐに会話を始められます。

### デスクトップクライアント（検収段階）

1. GitHub にログインし、[Actions](https://github.com/chengpt2025-hub/cgcode/actions) で `v2` ブランチの最新の成功したビルドを開きます。
2. **Artifacts** から `cgcode-windows-x64` をダウンロードして展開します。
3. `cgcode-<バージョン>-win-x64-unsigned.exe` を実行。SmartScreen が出たら **詳細情報 → 実行** をクリック。
4. 初回起動時に起動動画が流れ、ウェルカム画面で SenseNova の API キーを入力すれば使えます。

> 正式版では署名済みインストーラーと自動更新を提供します。

---

## ターミナル版の詳細

### テーマ（7 種類、`/theme` で切り替え）

| テーマ | 説明 |
|---|---|
| iOS ダーク | 既定 |
| iOS ライト | 明るいターミナル背景向け |
| ミッドナイトブルー | 深い青 |
| ミントグリーン | さわやかな緑 |
| サンセットオレンジ | 暖かいオレンジ |
| グラファイトグレー | 落ち着いたグレー |
| ハイコントラスト | 視認性重視、画面共有向け |

各テーマは UI の配色とシンタックスハイライトを同時に統一します。

### 同梱スキル（6 個）

| スキル | 内容 | 起動のきっかけ |
|---|---|---|
| cg-review | 中国語でコードレビュー、重要度別に問題と修正案を提示 | 「レビューして」「コードを見て」 |
| cg-ship | リリース前チェック：整形・ビルド・テスト・機密情報検出、中国語のコミットメッセージ作成 | 「リリース」「コミット前チェック」 |
| cg-focus | `.cgcode/FOCUS.md` のフォーカスボードで長いタスクを管理 | 「どこまで進んだ？」 |
| cg-memory | `.cgcode/MEMORY.md` に規約や教訓を記録 | 「覚えておいて」 |
| cg-git | Git ヘルパー：状態説明、ブランチ・競合・取り消し、危険な操作は事前確認 | git、ブランチ、競合 |
| cg-context | コンテキスト管理：進捗を要約し `/compact` や `/new` を提案 | 「長すぎる」「まとめて」 |

### 設定ファイル

場所：`%USERPROFILE%\.cgcode\cgcode.toml`（ウィザードが自動作成。通常は編集不要）。

```toml
[api]
base_url = "https://token.sensenova.cn/v1"
api_key  = "あなたのキー"
model    = "sensenova-6.8-flash"

# 任意：メインが失敗したときの予備エンドポイント
# [[fallback]]
# base_url = "https://api.deepseek.com/v1"
# api_key  = "..."
# model    = "deepseek-chat"

[intro]
enabled = true   # false で起動アニメーションをオフ
sound   = true   # false で起動音をオフ
```

---

## デスクトップクライアントの詳細

### 内蔵機能
- **モデル切り替え**：既定は SenseNova `sensenova-6.8-flash`。Qwen（qwen-plus、qwen-max）と Kimi（moonshot-v1-128k）を設定済み。OpenAI 互換モデルを自由に追加可能。
- **オールインワン**：チャット、Web 検索、Word / Excel / PowerPoint、画像生成、コーディング。
- **マルチエージェント協調**、**定期タスク**、**プラグイン管理**。
- 原画質の起動動画（1248×704）、クリックまたは任意のキーでスキップ。

### 同梱スキル（23 個）

**自社開発の中国語スキル（11 個）**

| スキル | 内容 |
|---|---|
| cg-writing | 中国語の執筆と推敲 |
| cg-meeting-notes | 議事録作成 |
| cg-data-analysis | データ分析 |
| cg-ppt | スライド作成 |
| cg-research | 調査 |
| cg-code-review | コードレビュー |
| cg-team | マルチエージェントの分担 |
| cg-daily-plan | 1 日の計画 |
| cg-skill-market | 中国語スキルマーケット：skills.sh・ClawHub・SkillHub を検索、安全チェック後にインストール |
| cg-web-scraper | Web データ収集、Excel に整理 |
| cg-multi-search | 複数エンジン検索（Baidu、Bing、Sogou など）を集約・重複排除 |

**厳選オープンソーススキル（12 個）**

| スキル | 内容 | 出典 / ライセンス |
|---|---|---|
| find-skills | オープンソーススキルの検索とインストール | vercel-labs/skills · MIT |
| agent-browser | ブラウザ自動操作：ページを開く、クリック、入力、スクリーンショット | vercel-labs/agent-browser · Apache-2.0 |
| frontend-design | デザイン性の高い Web ページと UI | anthropics/skills · Apache-2.0 |
| skill-creator | 独自スキルの作成・テスト・改善 | anthropics/skills · Apache-2.0 |
| mcp-builder | MCP ツールサーバーの開発 | anthropics/skills · Apache-2.0 |
| webapp-testing | Playwright による Web アプリのテスト | anthropics/skills · Apache-2.0 |
| canvas-design | ポスターとビジュアルデザイン | anthropics/skills · Apache-2.0 |
| theme-factory | 文書・スライド・Web の配色とフォントのテーマ | anthropics/skills · Apache-2.0 |
| web-artifacts-builder | 複数コンポーネントの複雑な Web ページ | anthropics/skills · Apache-2.0 |
| algorithmic-art | p5.js によるジェネラティブアート | anthropics/skills · Apache-2.0 |
| pdf | PDF の抽出・結合・分割・入力・作成 | anthropics/skills · Apache-2.0 |
| internal-comms | 週報やお知らせなど社内文書 | anthropics/skills · Apache-2.0 |

### スキルの安全方針
- 同梱スキルはすべてライセンスを確認し、付属スクリプトを審査済み（キー窃取・不審な通信・システム改変なし）。
- オープンソーススキルはビルド時に**固定コミット**で取得し、上流の改ざんを防止。
- モデルの安全制限を回避する脱獄系スキルは**同梱せず、スキルマーケットでも除外**：利用規約違反でキーやアカウントが停止される恐れがあり、公開配布には法的リスクもあります。

---

## アーキテクチャ

### ターミナル版

```
ターミナル
  └─ cgcode.exe（Rust）
       ├─ 中国語ウィザード / 起動アニメーション / iOS テーマ / 中国語 UI
       ├─ Codex コア（ファイル、コマンド、承認）── Responses API
       └─ cgcode-chat-bridge（ローカルブリッジ、127.0.0.1）
              └─ Responses ⇄ Chat Completions 変換、レート制限再試行、予備エンドポイント
                    └─ あなたのモデル API（SenseNova / DeepSeek / Qwen …）
```

| ファイル | 役割 |
|---|---|
| `codex-rs/cgcode-chat-bridge/` | Chat Completions ブリッジ（単体テスト付き） |
| `codex-rs/cli/src/cgcode.rs` | 設定、初回ウィザード、同梱スキル、ローカルブリッジ起動 |
| `codex-rs/cli/src/cgcode_intro.rs` | ターミナル起動アニメーションと音 |
| `codex-rs/tui/src/cgcode_theme.rs` | 7 種類の iOS 風テーマ |
| `.github/workflows/cgcode-build.yml` | Windows 自動ビルドとリリース |

### デスクトップクライアント

上流コードを直接書き換えず、ビルド時にカスタマイズを適用するため、上流の更新に追従しやすい構成です：

```
desktop/
├─ DSH_COMMIT              固定した上流バージョン
├─ apply.sh                ブランド、中国語既定、モデル設定を適用
├─ patch-bundle.mjs        既定モデル、複数モデル、テレメトリ無効化
├─ patch-desktop.mjs       ウェルカム画面のキー、起動動画、スキル導入
├─ fetch-skills.mjs        固定版のオープンソーススキル取得と中国語説明付与
├─ upstream-skills.json    スキル一覧（出典、バージョン、中国語説明）
└─ overlay/                アイコン、起動動画、自社開発の中国語スキル
ci/build-windows.sh        Windows ビルドスクリプト（Node 24 + pnpm）
```

### ビルドとリリース
- プッシュのたびに GitHub Actions が Windows 上で自動ビルド。
- ターミナル版：約 1.5〜2 時間、成功すると Releases に公開。
- デスクトップ版：約 15〜25 分、インストーラーを Artifact としてアップロード。
- ビルド完了ごとに「ビルド成功 / ビルド失敗」の Issue で結果を通知。

---

## ロードマップ

### ターミナル版
- [x] Chat のみ対応モデル向け Chat Completions ブリッジ
- [x] SenseNova 対応、レート制限時の再試行、予備エンドポイント
- [x] 中国語の初回セットアップウィザード
- [x] 独自ブランド（cgcode、`~/.cgcode`）、ChatGPT ログインとテレメトリを削除
- [x] 完全中国語 UI
- [x] 7 種類の iOS 風テーマ + `/theme`
- [x] 公式起動動画（ターミナル文字アート版）+ 起動音楽
- [x] 中国語スキル 6 個を同梱
- [x] Windows 自動ビルドとリリース
- [ ] 実機での検収
- [ ] 検収フィードバックを反映した安定版 v1.0
- [ ] 実行ログ機能

### デスクトップクライアント
- [x] cgcode ブランド、既定で中国語 UI
- [x] 複数モデル：SenseNova（既定）、Qwen、Kimi
- [x] ウェルカム画面でキー入力、導入のハードルなし
- [x] セッションテレメトリと製品分析をオフ
- [x] 原画質の起動動画
- [x] マルチエージェント協調、Office、定期タスク
- [x] スキル 23 個同梱 + 中国語スキルマーケット
- [ ] 実機での検収
- [ ] 上流のプラグインマーケットを cgcode のおすすめに置き換え
- [ ] Claude 風 UI の磨き込み（すりガラスなど iOS 効果の完全再現）
- [ ] スキルマーケット専用 UI
- [ ] コード署名と自動更新
- [ ] 正式版インストーラーの公開

---

## 既知の制限

- まだ署名されていないため、初回実行時に Windows SmartScreen の警告が出ます。「実行」を選んでください。
- どちらの版も実際の Windows PC での検収は未完了です。
- デスクトップ版は現在 GitHub にログインして Actions からダウンロードする必要があります。正式版は Releases で公開予定です。
- 一部のモデルサービス（SenseNova など）にはレート制限があり、混雑時は自動再試行に少し時間がかかることがあります。

---

## よくある質問

**SenseNova を使わなければいけませんか？**
いいえ。OpenAI 互換のエンドポイントなら何でも使えます。ウィザードや設定でエンドポイント、キー、モデル名を変更してください。

**起動アニメーションが長い / 音を消したい。**
ターミナル版は `cgcode.toml` の `[intro]` で `enabled` または `sound` を `false` に。デスクトップ版はクリックまたは任意のキーでスキップできます。

**コードや会話がアップロードされることはありますか？**
ありません。会話は自分で設定したモデルサービスにのみ送られ、テレメトリは既定ですべてオフです。

**スキルを追加するには？**
デスクトップ版で「〇〇ができるスキルを探して」と話しかけるだけ。スキルマーケットを検索して中国語で表示し、安全チェック後にインストールします。

---

## ライセンスと謝辞

- ターミナル版は [OpenAI Codex](https://github.com/openai/codex)（Apache License 2.0）をベースにしており、元の `LICENSE` と `NOTICE` を保持しています。
- デスクトップクライアントは DeepSeek Harness（MIT License）をベースに構築しています。cgcode は独立した製品であり、DeepSeek や OpenAI とは提携関係にありません。
- 同梱のオープンソーススキルは [vercel-labs/skills](https://github.com/vercel-labs/skills)（MIT）、[vercel-labs/agent-browser](https://github.com/vercel-labs/agent-browser)（Apache-2.0）、[anthropics/skills](https://github.com/anthropics/skills)（Apache-2.0）に由来し、各スキルのフォルダに元のライセンスを保持しています。
