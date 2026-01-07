use std::fs;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebContext};
use muda::{Menu, Submenu, PredefinedMenuItem};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();

    // 0. Menu bar setup (using muda)
    let menu = Menu::new();
    
    let file_menu = Submenu::new("File", true);
    let _ = file_menu.append(&PredefinedMenuItem::quit(None));
    
    let edit_menu = Submenu::new("Edit", true);
    let _ = edit_menu.append(&PredefinedMenuItem::cut(None));
    let _ = edit_menu.append(&PredefinedMenuItem::copy(None));
    let _ = edit_menu.append(&PredefinedMenuItem::paste(None));
    let _ = edit_menu.append(&PredefinedMenuItem::select_all(None));

    let _ = menu.append(&file_menu);
    let _ = menu.append(&edit_menu);

    #[cfg(target_os = "macos")]
    {
        menu.init_for_nsapp();
    }

    // 1. Data directory setup
    let mut data_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("./"));
    data_dir.push("bgm-browser");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap();
    }
    let mut web_context = WebContext::new(Some(data_dir));

    // 2. Window setup
    let window = WindowBuilder::new()
        .with_title("BGM Browser")
        .with_decorations(false)
        .with_maximized(true)
        .build(&event_loop)
        .unwrap();

    // 3. Set YouTube homepage URL
    let url = "https://www.youtube.com/".to_string();

    // Safari (macOS) User Agent
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15";

    // 4. Initialization script (This is large)
    let init_script = r#"
        (function() {
            // Start in "Normal Mode" (false)
            window.isBgmMode = false;

            console.log("Started in Normal Mode. Press 'T' to toggle BGM Mode.");

            // Keyboard event: Toggle mode with 't' key
            window.addEventListener('keydown', (e) => {
                // Do not react during text input
                if (e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA' && e.target.contentEditable !== 'true') {
                    if (e.key === 't' || e.key === 'T') {
                        window.isBgmMode = !window.isBgmMode;
                        console.log("Mode Toggled:", window.isBgmMode ? "BGM Mode" : "Normal Mode");
                        
                        if (!window.isBgmMode) {
                            // Return to Normal Mode: Remove CSS & Restore visibility
                            const style = document.getElementById('force-fullscreen-style');
                            if (style) style.remove();
                            
                            const hidden = document.querySelectorAll('*[style*="display: none"]');
                            hidden.forEach(el => el.style.display = '');
                        } else {
                            // Switch to BGM Mode: Apply styles
                            applyStyles();
                        }
                    }
                }
            });

            // CSS definitions
            const css = `
                /* Set background to black */
                html, body, ytd-app { background: black !important; overflow: hidden !important; }
                
                /* Hide UI elements */
                ytd-masthead, #masthead-container, #secondary, #below, #comments, #chat, ytd-playlist-panel-renderer {
                    display: none !important;
                }
                
                /* Force player to full screen, top layer, transparent background */
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
                /* Hide control bar */
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

            // Watch loop
            setInterval(() => {
                applyStyles();
            }, 100);
        })();
    "#; // End of initialization script

    // 5. Build WebView
    let _webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_url(&url)
        .with_user_agent(user_agent)
        .with_initialization_script(init_script)
        .with_autoplay(true)
        .with_devtools(cfg!(debug_assertions))
        .build(&window)?;

    // 6. Event loop
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
