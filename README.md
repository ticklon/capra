# CAPRA (カプラ)

**全ての背景透過ターミナル愛好家（ジャンキー）たちへ**

**To all transparent terminal junkies.**

![License](https://img.shields.io/badge/license-MIT-blue.svg)

> **"The Background Grazer"**
>
> 「あなたのターミナルの裏庭で、静かに音を食む。」

**CAPRA** is a lightweight, distraction-free web browser built with Rust, 
specifically designed for playing YouTube videos in the background while you work.

![sample](https://github.com/user-attachments/assets/1a803f98-22d7-40ee-8eb5-7a4ba98b4291)

**Use Case:** Coding with WezTerm (transparent background) + Capra.

**使用例:** 背景透過したWezTermの裏でCapraを動作させている様子。

<small>Video Source: [【Playlist】クセになる51曲 3時間](https://youtu.be/rCStA4cNrVM) by zukisuzuki BGM</small>

## 🐐 Concept & Story

**The Background Grazer.**

Goats (*Capra*) are known to climb steep mountains (complex development environments) with ease and graze on weeds (distractions/ads) for nutrition.

**CAPRA** resides in the "backyard" (background) of your development environment, chewing away the intrusive UI elements and delivering only the pure music and visuals you need.

---

**背景の放牧者。**

山羊（*Capra*）は、険しい山（複雑な開発環境）でも平気で登り、雑草を食べて栄養にします。

**CAPRA** は、あなたの開発環境の「裏側（Background）」に常駐し、UIを噛み砕いて、純粋な音楽と映像だけを届けます。

## ✨ Features / 特徴

*   **Dedicated to YouTube**: Opens YouTube automatically on launch.
*   **YouTube専用設計**: 起動と同時にYouTubeを開きます。

*   **BGM Mode (Distraction-Free)**: Press **`T`** to toggle "BGM Mode". This hides comments, sidebars, and headers, leaving only the video player in full screen.
*   **BGMモード**: **`T`** キーを押すと、動画以外の要素（ヘッダー、サイドバー、コメント欄など）を全て非表示にします。

*   **MacOS Native Shortcuts**: Supports `Cmd+Q` (Quit), `Cmd+W` (Close), `Cmd+C/V` (Copy/Paste), etc.
*   **Mac標準ショートカット対応**: 終了やコピペなど、OS標準のキー操作に対応しています。

*   **Persistent Session**: Log in once, and your session is saved for next time.
    *   **ログイン状態の保持**: ログイン情報は保存されるため、毎回ログインし直す必要はありません。
*   **Security (Navigation Lock)**: Blocks access to external sites. Only YouTube and Google Login are allowed.
    *   **セキュリティ（ナビゲーションロック）**: 外部サイトへのアクセスをブロックします。YouTubeとGoogleログインのみ許可されます。
## 🚀 Installation & Usage / インストールと使い方

### Prerequisites / 事前準備
*   **macOS** (Optimized for macOS)
*   **Rust**: If not installed, get it from [rustup.rs](https://rustup.rs/).
*   Rustが未インストールの場合は [rustup.rs](https://rustup.rs/) からインストールしてください。

### Option 1: Install with Cargo (Recommended for CLI users) / Cargoでインストール

If you have Rust installed, you can install CAPRA directly.

Rust環境がある方は、以下のコマンドで直接インストールできます。

```bash
cargo install --git https://github.com/ticklon/capra.git
```

Run from anywhere:
インストール後はターミナルから `capra` で起動できます:
```bash
capra
```

### Option 2: Build as macOS App / macOSアプリとしてビルド

If you want to create a standalone `Capra.app`:

スタンドアロンの `.app` 形式でビルドする場合：

```bash
# Install cargo-bundle if you haven't
cargo install cargo-bundle

# Build the bundle
cargo bundle --release
```
The output will be in `target/release/bundle/osx/Capra.app`. You can move it to your `/Applications` folder.

ビルドされたアプリは `target/release/bundle/osx/Capra.app` に出力されます。`/Applications` フォルダに移動して使用してください。

### Option 3: Build from Source / ソースから直接実行

```bash
git clone https://github.com/ticklon/capra.git
cd capra
cargo run --release
```

### Keyboard Shortcuts / ショートカット

| Key / キー | Action / 動作 |
|:---|:---|
| **`T`** | Toggle BGM Mode (Show/Hide UI) / BGMモード切替 |
| `Cmd + Q` | Quit Application / アプリ終了 |
| `Cmd + W` | Close Window / ウィンドウを閉じる |
| `Cmd + R` | Reload / 再読み込み (Standard Browser Shortcut) |

## 🛠 Tech Stack / 技術スタック

*   [Rust](https://www.rust-lang.org/)
*   [Tao](https://github.com/tauri-apps/tao) - Cross-platform window creation library.
*   [Wry](https://github.com/tauri-apps/wry) - Cross-platform WebView rendering library.
*   [Muda](https://github.com/tauri-apps/muda) - Menu Bar library.

## ⚠️ Disclaimer / 免責事項

This application uses DOM manipulation to hide YouTube UI elements. Since it depends on YouTube's site structure, "BGM Mode" may break if YouTube updates their layout.

本アプリはYouTubeのHTML構造（DOM）を直接操作してUIを非表示にしています。YouTube側の仕様変更により、BGMモードが正常に動作しなくなる可能性があります。

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
