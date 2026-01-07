use std::fs;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebContext};
use muda::{Menu, Submenu, PredefinedMenuItem};
use anyhow::{Context, Result};
use url::Url;

fn main() -> Result<()> {
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
    let mut data_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("could not determine config dir"))?;
    data_dir.push("capra");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .with_context(|| format!("failed to create data dir {:?}", data_dir))?;
    }
    let mut web_context = WebContext::new(Some(data_dir));

    // 2. Window setup
    let window = WindowBuilder::new()
        .with_title("BGM Browser")
        .with_decorations(false)
        .with_maximized(true)
        .build(&event_loop)
        .context("failed to build window")?;

    // 3. Set YouTube homepage URL
    let url = "https://www.youtube.com/".to_string();

    // Safari (macOS) User Agent
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15";

    // 4. Initialization script
    let init_script = r#"
        (function() {
            window.isBgmMode = false;
            console.log("Started in Normal Mode. Press 'T' to toggle BGM Mode.");

            const css = `
                html, body, ytd-app { background: black !important; overflow: hidden !important; }
                ytd-masthead, #masthead-container, #secondary, #below, #comments, #chat, ytd-playlist-panel-renderer {
                    display: none !important;
                }
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

            function removeStyles() {
                const style = document.getElementById('force-fullscreen-style');
                if (style) style.remove();
                window.dispatchEvent(new Event('resize'));
            }

            // Keyboard listener
            document.addEventListener('keydown', (e) => {
                if (!(e.target instanceof HTMLElement)) return;
                if (e.target.isContentEditable) return;
                const tag = e.target.tagName;
                if (tag === 'INPUT' || tag === 'TEXTAREA') return;

                if (e.key === 't' || e.key === 'T') {
                    window.isBgmMode = !window.isBgmMode;
                    console.log("Mode Toggled:", window.isBgmMode ? "BGM Mode" : "Normal Mode");
                    if (window.isBgmMode) applyStyles();
                    else removeStyles();
                }
            });

            // MutationObserver to keep styles applied
            const observer = new MutationObserver(() => {
                if (window.isBgmMode) applyStyles();
            });
            
            observer.observe(document.documentElement, { 
                childList: true, 
                subtree: true, 
                attributes: true 
            });
        })();
    "#;

    // 5. WebView setup
    #[allow(unused_variables)]
    let webview = WebViewBuilder::with_web_context(&mut web_context)
        .with_url(&url)
        .with_user_agent(user_agent)
        .with_initialization_script(init_script)
        .with_autoplay(true)
        .with_devtools(cfg!(debug_assertions))
        .with_navigation_handler(|url_string| {
            // Parse the URL
            let Ok(url) = Url::parse(&url_string) else {
                println!("Blocked invalid URL: {}", url_string);
                return false;
            };

            // Allow specific schemes
            match url.scheme() {
                "data" | "blob" | "about" => return true,
                _ => {}
            }

            // Check host
            if let Some(host) = url.host_str() {
                let host = host.to_lowercase();
                let allowed_suffixes = [
                    "youtube.com", "youtu.be", "ytimg.com", 
                    "google.com", "gstatic.com", "googlevideo.com",
                    "accounts.google.com", "myaccount.google.com"
                ];
                
                if allowed_suffixes.iter().any(|s| host == *s || host.ends_with(&format!(".{}", s))) {
                    return true;
                }
            }
            
            println!("Blocked navigation to: {}", url_string);
            false
        })
        .build(&window)
        .context("failed to build webview")?;

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