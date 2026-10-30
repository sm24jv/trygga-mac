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
                .body(Vec::new())
                .unwrap()
        })
        .setup(|app| {
            TrayIconBuilder::with_id(TRAY_ID)
                .icon(tauri::include_image!("icons/tray.png"))
                .icon_as_template(true)
                .build(app)?;

            // In code and not in tauri.conf.json: the title handler exists only on the builder.
            WebviewWindowBuilder::new(app, "panel", WebviewUrl::App("index.html".into()))
                .inner_size(380.0, 640.0)
                .visible(false)
                // A hidden page is suspended by default, and the bar lives on its ticks.
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
