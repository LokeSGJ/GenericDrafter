use crate::{Player, run_drafter};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    path: String,

    #[serde(skip)]     player_list: Vec<Player>, // This how you opt-out of serialization of a field
    num_players: usize,
    num_picks: usize,
    unique_picks: bool,
    error_message: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            path: "games/".to_owned(),
            num_players: 3,
            num_picks: 3,
            player_list: Vec::new(),
            unique_picks: true,
            error_message: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Generic Drafter");

            ui.horizontal(|ui| {
                ui.label("File path for options: ");
                ui.text_edit_singleline(&mut self.path);
            });

            ui.label("Number of players:");
            ui.add(egui::Slider::new(&mut self.num_players, 1..=10));

            ui.label("Picks per player:");
            ui.add(egui::Slider::new(&mut self.num_picks, 1..=10));

            ui.add(egui::Checkbox::new(&mut self.unique_picks, "Disallow duplicates"));

            if ui.button("Draft!").clicked() {
                self.error_message = String::new();
                match run_drafter(&mut self.path, self.num_players, self.num_picks, self.unique_picks) {
                    Ok(pl) => self.player_list = pl,
                    Err(e) => self.error_message = e.to_string(),
                }
            }

            ui.separator();

            ui.label(&self.error_message);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui|{

                for player in &self.player_list {
                    ui.vertical(|ui|{
                        ui.label("Player ".to_owned() + player.number.to_string().as_str());
                        for pick in &player.picks{
                            ui.label(pick);
                        }
                    });
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.add(egui::github_link_file!(
                    "https://github.com/LokeSGJ/GenericDrafter/tree/master/",
                    "Source code."
                ));
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}