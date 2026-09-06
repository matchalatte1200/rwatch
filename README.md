# rwatch

**rwatch** は、Rustで開発している軽量なシステム監視ツールです。

CPU、メモリ、ディスクなどのシステムリソースを定期的に取得し、ログとして出力します。

## 機能

* CPU使用率の監視
* メモリ使用率の監視
* ルートファイルシステム（`/`）のディスク使用率の監視
* NTPによる時刻同期
* 5秒間隔での定期監視
* `env_logger` を利用したログ出力

## 動作環境

* Linux
* Rust
* Cargo

## インストール

リポジトリをクローンして、Cargoでビルドします。

```bash
git clone https://github.com/matchalatte1200/rwatch.git
cd rwatch
cargo build --release
```

ビルドされた実行ファイルは以下に生成されます。

```text
target/release/rwatch
```

## 使い方

Cargoから実行できます。

```bash
cargo run
```

リリースビルドを直接実行する場合：

```bash
./target/release/rwatch
```

デフォルトでは、**5秒間隔**でシステムの状態を取得します。

出力例：

```text
[2026-09-06 12:00:00.000] CPU: 12.34%, Memory available: 58.21%, Disk: 72.45%
```

## 監視項目

### CPU

システム全体のCPU使用率を取得します。

CPUの情報取得には [`sysinfo`](https://crates.io/crates/sysinfo) crate を使用しています。

### メモリ

現在利用可能なメモリの割合を取得します。

以下の計算によって算出します。

```text
利用可能メモリ / 総メモリ × 100
```

### ディスク

ルートファイルシステム（`/`）の使用率を監視します。

使用率は以下の計算によって算出します。

```text
(総容量 - 利用可能容量) / 総容量 × 100
```

現在は `/` のファイルシステムのみを監視対象としています。

## 時刻同期

rwatchでは、監視結果のタイムスタンプに使用する時刻をNTPによって取得します。

現在設定されているNTPサーバーは以下の通りです。

```text
169.254.169.254:123
ntp.nict.jp:123
```

設定されたNTPサーバーとの時刻同期に失敗した場合、rwatchは監視処理を開始せず終了します。

## ログ

ログにはRustの `log` ecosystemと `env_logger` を使用しています。

デフォルトのログレベルは `info` です。

`RUST_LOG` 環境変数によってログレベルを変更できます。

```bash
RUST_LOG=debug cargo run
```

例えば、警告以上のみを表示する場合：

```bash
RUST_LOG=warn ./target/release/rwatch
```

## プロジェクト構成

現在のソースコードは、監視対象ごとにモジュールを分離しています。

```text
src/
├── main.rs
├── log.rs
└── metrics/
    ├── mod.rs
    ├── cpu.rs
    ├── memory.rs
    ├── disk.rs
    └── time.rs
```

### `main.rs`

主に以下の処理を担当します。

* ロガーの初期化
* システム情報の初期化
* NTPの初期化
* 監視ループ
* メトリクスの取得
* ログ出力

### `metrics/cpu.rs`

`sysinfo` を利用してCPU使用率を取得します。

### `metrics/memory.rs`

利用可能メモリの割合を取得します。

### `metrics/disk.rs`

ルートファイルシステム（`/`）のディスク使用率を取得します。

### `metrics/time.rs`

NTPによる時刻同期を担当します。

## 使用しているcrate

| crate        | 用途                |
| ------------ | ----------------- |
| `sysinfo`    | CPU・メモリ・ディスク情報の取得 |
| `chrono`     | 日時の処理             |
| `rsntp`      | NTPによる時刻同期        |
| `log`        | ログAPI             |
| `env_logger` | ロガーの実装            |

## 開発

通常ビルド：

```bash
cargo build
```

リリースビルド：

```bash
cargo build --release
```

実行：

```bash
cargo run
```

デバッグログを有効にして実行：

```bash
RUST_LOG=debug cargo run
```

コードフォーマット：

```bash
cargo fmt
```

テスト：

```bash
cargo test
```

コンパイルチェック：

```bash
cargo check
```

## 開発状況

rwatchは現在開発中です。

現在は、CPU・メモリ・ディスクなどの基本的なシステムメトリクスを取得する機能と、監視処理の基本構造を実装しています。

今後、監視対象やログ機能などを拡張していく予定です。

## 今後の予定

* [ ] JSONL形式でのログ出力
* [ ] ネットワーク通信量・速度の監視
* [ ] 監視間隔の設定
* [ ] NTPサーバーの設定
* [ ] 閾値に基づくWarning / Critical判定
* [ ] Graceful Shutdown
* [ ] コマンドラインオプションの追加
* [ ] 監視項目の追加

## ライセンス

現在、ライセンスは設定していません。
