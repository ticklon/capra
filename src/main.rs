use std::fs;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebContext};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    // 1. データディレクトリ設定
    let mut data_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("./"));
    data_dir.push("bgm-browser");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap();
    }
    let mut web_context = WebContext::new(Some(data_dir));

    // 2. ウィンドウ設定
    let window = WindowBuilder::new()
        .with_title("BGM Browser")
        .with_decorations(false)
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();

    // 3. YouTubeのトップページを指定
    let url = "https://www.youtube.com/".to_string();

    // Safari (macOS) の User Agent
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15";

    // 4. スクリプト（ここが長いので注意）
    let init_script = r#"
        (function() {
            // 初期状態は「通常モード(false)」でスタート
            window.isBgmMode = false;

            console.log("Started in Normal Mode. Press 'T' to toggle BGM Mode.");

            // キーボードイベント: 't' キーでモード切替
            window.addEventListener('keydown', (e) => {
                // 文字入力中は反応させない
                if (e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA' && e.target.contentEditable !== 'true') {
                    if (e.key === 't' || e.key === 'T') {
                        window.isBgmMode = !window.isBgmMode;
                        console.log("Mode Toggled:", window.isBgmMode ? "BGM Mode" : "Normal Mode");
                        
                        if (!window.isBgmMode) {
                            // 通常モードへ戻す: CSS削除 & 表示復帰
                            const style = document.getElementById('force-fullscreen-style');
                            if (style) style.remove();
                            
                            const hidden = document.querySelectorAll('*[style*="display: none"]');
                            hidden.forEach(el => el.style.display = '');
                        } else {
                            // BGMモードへ: スタイル適用
                            applyStyles();
                        }
                    }
                }
            });

            // CSS定義
            const css = `
                /* 背景を黒に */
                html, body, ytd-app { background: black !important; overflow: hidden !important; }
                
                /* UIを非表示 */
                ytd-masthead, #masthead-container, #secondary, #below, #comments, #chat, ytd-playlist-panel-renderer {
                    display: none !important;
                }
                
                /* プレイヤーを全画面化・最前面・透明背景 */
                #movie_player, .html5-video-player, #player-container-outer, #player-container-inner, #player-container {
                    position: fixed !important;
                    top: 0 !important;
                    left: 0 !important;
                    width: 100vw !important;
                    height: 100vh !important;
                    z-index: 2147483647 !important;
                    background: transparent !important;
                    margin: 0 !important;
                    padding: 0 !important;
                }
                .html5-video-container { width: 100% !important; height: 100% !important; }
                video {
                    object-fit: cover !important;
                    width: 100% !important;
                    height: 100% !important;
                    display: block !important;
                }
                /* コントロールバー消去 */
                .ytp-chrome-top, .ytp-gradient-top, .ytp-chrome-bottom { display: none !important; }
            `;

            function applyStyles() {
                if (!window.isBgmMode) return;
                if (!document.head) return;

                if (!document.getElementById('force-fullscreen-style')) {
                    const style = document.createElement('style');
                    style.id = 'force-fullscreen-style';
                    style.textContent = css;
                    document.head.appendChild(style);
                }
            }

            // 監視ループ
            setInterval(() => {
                applyStyles();
            }, 100);
        })();
    "#; // ここが閉じタグです

    // 5. WebViewの構築
    let _webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_url(&url)
        .with_user_agent(user_agent)
        .with_initialization_script(init_script)
        .with_autoplay(true)
        .with_devtools(true)
        .build(&window)?;

    // 6. イベントループ
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
