use crate::app;
use app::App;
use egui::ThemePreference;

#[cfg(not(target_arch = "wasm32"))]
pub fn start(crate_name: String, theme: ThemePreference) -> eframe::Result {
    const WINDOW_WIDTH: f32 = 900.0;
    const WINDOW_HEIGHT: f32 = 550.0;

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(&crate_name)
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon-256.png")[..],
                )
                .unwrap_or_else(|err| {
                    log::error!(
                        "{}",
                        format!(
                            "Error occurred while loading app icon. Additional Info: {}",
                            err
                        )
                    );
                    std::process::exit(1);
                }),
            ),
        ..Default::default()
    };

    eframe::run_native(
        &crate_name,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, theme)))),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn start() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::new(cc, ThemePreference::Dark)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                },
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                },
            }
        }
    });
}

pub mod components {
    pub mod canvas;
    pub mod settings;
}
pub mod styles {
    pub mod colors;
    pub mod strokes;
}
pub mod windows;
