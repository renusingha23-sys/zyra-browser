#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

#[tauri::command]
fn clean_data(window: tauri::Window) {
    window.with_webview(|webview| {
        #[cfg(target_os = "linux")]
        {
            use webkit2gtk::{WebViewExt, WebsiteDataManagerExt};

            if let Some(wv) = webview.downcast_ref::<webkit2gtk::WebView>() {
                let context = wv.context();
                context.clear_cache();
                context.clear_all_databases();
            }
        }
    }).ok();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            window.with_webview(|webview| {
                #[cfg(target_os = "linux")]
                {
                    use webkit2gtk::{
                        WebViewExt,
                        SettingsExt,
                        CookieAcceptPolicy,
                    };

                    if let Some(wv) = webview.downcast_ref::<webkit2gtk::WebView>() {

                        // 🔒 Block third-party cookies
                        let context = wv.context();
                        context.set_cookie_accept_policy(
                            CookieAcceptPolicy::NoThirdParty
                        );

                        // 🎨 Dark mode for GUI (not websites)
                        if let Some(settings) = wv.settings() {
                            settings.set_enable_webgl(true); // keep normal rendering
                            settings.set_enable_developer_extras(false);
                        }
                    }
                }
            }).ok();

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