#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bar_text;
mod measure;

use tauri::tray::TrayIconBuilder;
use tauri::utils::config::BackgroundThrottlingPolicy;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

const TRAY_ID: &str = "main";

fn main() {
    tauri::Builder::default()
        .register_uri_scheme_protocol("probe", |_ctx, request| {
            measure::append("page.log", request.uri().query().unwrap_or(""));
            tauri::http::Response::builder()
                .status(204)
                .header("Access-Control-Allow-Origin", "*")
                .body(Vec::new())
                .unwrap()
        })
        .setup(|app| {
            TrayIconBuilder::with_id(TRAY_ID)
                .icon(tauri::include_image!("icons/tray.png"))
                .icon_as_template(true)
                .build(app)?;

            WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("index.html".into()))
                .inner_size(380.0, 640.0)
                .visible(false)
                .background_throttling(BackgroundThrottlingPolicy::Disabled)
                .on_document_title_changed(|window, title| {
                    measure::append("title.log", &title);
                    if let Some(tray) = window.app_handle().tray_by_id(TRAY_ID) {
                        let _ = tray.set_title(bar_text::from_title(&title));
                    }
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("the app failed to start");
}
