mod bar_text;
mod placement;

use placement::{Area, Screen};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;
use tauri::menu::{Menu, PredefinedMenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::utils::config::BackgroundThrottlingPolicy;
use tauri::webview::NewWindowResponse;
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Rect, RunEvent, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder, WindowEvent,
};

const TRAY_ID: &str = "main";
const WINDOW_LABEL: &str = "timer";
const WIDTH: f64 = 380.0;
const HEIGHT: f64 = 640.0;

const HOST: &str = "app.trygga.io";
const ADDRESS: &str = "https://app.trygga.io/timer";

/// The user agent a `WKWebView` has by itself, with trygga's marker. Setting one replaces the
/// whole string. The marker's name is `MENU_BAR_APP_MARKER` in trygga, and installed apps hang on it.
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 \
                          (KHTML, like Gecko) TryggaMac";

/// What a press on the icon found, kept for its release, which acts: the press itself can hide the
/// panel by taking the focus, so the release no longer sees what the press was aimed at.
#[derive(Default)]
struct IconClicks {
    hidden_at: Mutex<Option<Instant>>,
    found_open: AtomicBool,
}

/// The one window is a panel without a title bar, or an ordinary window with one.
fn is_panel(window: &WebviewWindow) -> bool {
    window.is_decorated().is_ok_and(|decorated| !decorated)
}

fn is_open_panel(window: &WebviewWindow) -> bool {
    is_panel(window) && window.is_visible().unwrap_or(false)
}

fn set_shape(window: &WebviewWindow, panel: bool) {
    let _ = window.set_decorations(!panel);
    let _ = window.set_always_on_top(panel);
    let _ = window.set_visible_on_all_workspaces(panel);
    // A title bar takes its height out of the page.
    let _ = window.set_size(LogicalSize::new(WIDTH, HEIGHT));
}

/// On the next turn of the main thread, because a change of shape is queued there: showing at once
/// would draw the old shape in the old place.
fn show_with_focus(window: &WebviewWindow) {
    let shown = window.clone();
    let _ = window.run_on_main_thread(move || {
        let _ = shown.unminimize();
        let _ = shown.show();
        let _ = shown.set_focus();
    });
}

fn open_panel(window: &WebviewWindow, icon: Rect) {
    set_shape(window, true);
    if let Some((left, top)) = place_under(window, icon) {
        let _ = window.set_position(LogicalPosition::new(left, top));
    }
    show_with_focus(window);
}

/// The icon's rectangle comes in the scale of its own screen, which it does not name. The cursor
/// stands on the icon and comes in the main screen's scale, so that one finds the screen.
fn place_under(window: &WebviewWindow, icon: Rect) -> Option<(f64, f64)> {
    let main_scale = window.primary_monitor().ok()??.scale_factor();
    let cursor = window.cursor_position().ok()?;
    let monitor = window
        .monitor_from_point(cursor.x / main_scale, cursor.y / main_scale)
        .ok()??;
    let position = icon.position.to_physical::<f64>(1.0);
    let size = icon.size.to_physical::<f64>(1.0);
    let icon = Area {
        left: position.x,
        top: position.y,
        width: size.width,
        height: size.height,
    };
    let screen = Screen {
        area: Area {
            left: f64::from(monitor.position().x),
            top: f64::from(monitor.position().y),
            width: f64::from(monitor.size().width),
            height: f64::from(monitor.size().height),
        },
        scale: monitor.scale_factor(),
    };
    Some(placement::under_icon(icon, WIDTH, &screen))
}

fn open_ordinary_window(window: &WebviewWindow) {
    if is_panel(window) {
        set_shape(window, false);
        let _ = window.center();
    }
    show_with_focus(window);
}

fn on_icon_event(app: &AppHandle, event: TrayIconEvent) {
    let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state,
        rect,
        ..
    } = event
    else {
        return;
    };
    let Some(window) = app.get_webview_window(WINDOW_LABEL) else {
        return;
    };
    let clicks = app.state::<IconClicks>();
    match button_state {
        MouseButtonState::Down => {
            let hidden_at = *clicks.hidden_at.lock().unwrap();
            let open = placement::found_open(
                is_open_panel(&window),
                hidden_at.map(|moment| moment.elapsed()),
            );
            clicks.found_open.store(open, Ordering::Relaxed);
        }
        MouseButtonState::Up if clicks.found_open.load(Ordering::Relaxed) => {
            let _ = window.hide();
        }
        MouseButtonState::Up => open_panel(&window, rect),
    }
}

fn on_window_event(window: &WebviewWindow, event: &WindowEvent) {
    match event {
        // Hidden and not closed: the page has to keep running for the bar.
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            let _ = window.hide();
        }
        // A change of shape hands the keyboard to the window, and the page wants it.
        WindowEvent::Focused(true) => {
            let webview: &tauri::Webview = window.as_ref();
            let _ = webview.set_focus();
        }
        WindowEvent::Focused(false) if is_panel(window) => {
            let _ = window.hide();
            *window.state::<IconClicks>().hidden_at.lock().unwrap() = Some(Instant::now());
        }
        _ => {}
    }
}

fn main() {
    // No capability file exists and none may: without one the page reaches nothing native. The
    // Edit menu that copying and pasting need is in Tauri's default menu.
    tauri::Builder::default()
        .manage(IconClicks::default())
        .setup(|app| {
            let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;
            TrayIconBuilder::with_id(TRAY_ID)
                .icon(tauri::include_image!("icons/tray.png"))
                .icon_as_template(true)
                .menu(&Menu::with_items(app, &[&quit])?)
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| on_icon_event(tray.app_handle(), event))
                .build(app)?;

            // In code and not in tauri.conf.json: the handlers exist only on the builder.
            let address = WebviewUrl::External(ADDRESS.parse()?);
            let window = WebviewWindowBuilder::new(app, WINDOW_LABEL, address)
                .title("trygga")
                .inner_size(WIDTH, HEIGHT)
                .resizable(false)
                .decorations(false)
                .visible(false)
                .user_agent(USER_AGENT)
                // A hidden page is suspended by default, and the bar lives on its ticks.
                .background_throttling(BackgroundThrottlingPolicy::Disabled)
                .on_navigation(|url| url.scheme() == "https" && url.host_str() == Some(HOST))
                .on_new_window(|_, _| NewWindowResponse::Deny)
                .on_document_title_changed(|window, title| {
                    if let Some(tray) = window.app_handle().tray_by_id(TRAY_ID) {
                        let _ = tray.set_title(bar_text::from_title(&title));
                    }
                })
                .build()?;

            let events_window = window.clone();
            window.on_window_event(move |event| on_window_event(&events_window, event));
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("the app failed to start")
        .run(|app, event| {
            if let RunEvent::Reopen { .. } = event {
                if let Some(window) = app.get_webview_window(WINDOW_LABEL) {
                    open_ordinary_window(&window);
                }
            }
        });
}
