#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WindowEvent, WebviewWindow};

#[tauri::command]
fn clean_data(window: tauri::Window) {
    #[cfg(target_os = "linux")] // Only run this on Linux
    window.with_webview(|webview| {
        use webkit2gtk::{WebViewExt, WebsiteDataManagerExt};
        if let Some(wv) = webview.downcast_ref::<webkit2gtk::WebView>() {
            let context = wv.context();
            context.clear_cache();
            context.clear_all_databases();
        }
    }).ok();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window: WebviewWindow = app.get_webview_window("main").unwrap();

            window
                .with_webview(|webview| {
                    use webkit2gtk::{
                        CookieAcceptPolicy,
                        SettingsExt,
                        WebViewExt,
                    };

                    if let Some(wv) = webview.downcast_ref::<webkit2gtk::WebView>() {

                        // Block third-party cookies
                        let context = wv.context();
                        context.set_cookie_accept_policy(
                            CookieAcceptPolicy::NoThirdParty,
                        );

                        // Browser engine settings
                        if let Some(settings) = wv.settings() {
                            settings.set_enable_webgl(true);
                            settings.set_enable_developer_extras(false);
                        }
                    }
                })
                .ok();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![clean_data])
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                clean_data(window.clone());
            }
        })
        .run(tauri::generate_context!())
        .expect("error running app");
}
