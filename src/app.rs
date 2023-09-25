use petgraph::graphmap::*;

// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)]
pub struct App<'a> {
    graph: UnGraphMap<&'a str, f32>,
    start: String,
    end: String,
    method: Method,
    path: Option<Path>,
}

pub enum Method {
    AStar,
    BruteForce,
    BreadthFirst,
    DepthFirst,
    IDDFS,
    BestFirst,
    None,
}

pub struct Path {
    distance: f32,
    path: Vec<String>,
    mem_used: u32,
    run_time: u8, // Convert to time
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            graph: UnGraphMap::new(),
            start: "".to_string(),
            end: "".to_string(),
            method: Method::None,
            path: Option::<Path>::None,
        }
    }
}

impl App<'_> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
            // return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App<'_> {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { graph, start, end, method, path } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            egui::ComboBox::from_label("Start Point")
                .selected_text(format!("{:?}", self.start))            
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.start, "test".to_string(), "test");
                }
            );
            ui.heading("eframe template");

        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
