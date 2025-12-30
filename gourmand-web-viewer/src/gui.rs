use eframe::egui;
use gourmand_web_viewer::recipe::Recipe;
use std::collections::{BTreeMap, HashMap};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
pub fn run_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([525.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gourmand web viewer 0.2.0",
        options,
        Box::new(|cc| Ok(Box::new(GourmandWebViewer::new(cc)))),
    )

}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
pub fn run_gui_wasm() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

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
                Box::new(|cc| Ok(Box::new(GourmandWebViewer::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

#[derive(Debug)]
struct GourmandWebViewer {
    categories_buttons: BTreeMap<String, bool>,
    cuisines_buttons: BTreeMap<String, bool>,
    category_filter: HashMap<String, bool>,
    cuisine_filter: HashMap<String, bool>,
    input1: String,
    input2: String,
    input3: String,
    recipes: HashMap<String, Recipe>,
}

impl GourmandWebViewer {
        /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (categories, cuisines, recipes) = gourmand_web_viewer::load_json();

        let categories_buttons: BTreeMap<String, bool> =
            categories.into_iter().map(|c| (c, false)).collect();
        let cuisines_buttons: BTreeMap<String, bool> =
            cuisines.into_iter().map(|c| (c, false)).collect();

        Self {
            categories_buttons,
            cuisines_buttons,
            category_filter: HashMap::new(),
            cuisine_filter: HashMap::new(),
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            recipes,
        }
    }
}

impl eframe::App for GourmandWebViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Custom background color from style.rs: 0xFF, 0xFE, 0xF0
        let my_frame = egui::containers::Frame {
            fill: egui::Color32::from_rgb(0xFF, 0xFE, 0xF0),
            ..egui::containers::Frame::default()
        };

        egui::CentralPanel::default()
            .frame(my_frame)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 8.0);

                    // Categories
                    ui.horizontal_wrapped(|ui| {
                        for (title, _) in self.categories_buttons.iter() {
                            let is_selected = *self.category_filter.get(title).unwrap_or(&false);
                            let button = if is_selected {
                                egui::Button::new(
                                    egui::RichText::new(title).color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(0x11, 0x42, 0x87))
                            } else {
                                egui::Button::new(
                                    egui::RichText::new(title).color(egui::Color32::BLACK),
                                )
                                .fill(egui::Color32::TRANSPARENT)
                            };

                            // Custom styling to match "rounded" from iced
                            // egui buttons are rounded by default, but we can customize if needed.
                            // For now default rounded is fine.

                            if ui.add(button).clicked() {
                                let new_state = !is_selected;
                                self.category_filter.clear(); // Single selection behavior
                                if new_state {
                                    self.category_filter.insert(title.clone(), true);
                                }
                            }
                        }
                    });

                    // Cuisines
                    ui.horizontal_wrapped(|ui| {
                        for (title, _) in self.cuisines_buttons.iter() {
                            let is_selected = *self.cuisine_filter.get(title).unwrap_or(&false);
                            let button = if is_selected {
                                egui::Button::new(
                                    egui::RichText::new(title).color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(0x11, 0x42, 0x87))
                            } else {
                                egui::Button::new(
                                    egui::RichText::new(title).color(egui::Color32::BLACK),
                                )
                                .fill(egui::Color32::TRANSPARENT)
                            };

                            if ui.add(button).clicked() {
                                let new_state = !is_selected;
                                self.cuisine_filter.clear();
                                if new_state {
                                    self.cuisine_filter.insert(title.clone(), true);
                                }
                            }
                        }
                    });

                    // Inputs
                    ui.add(egui::TextEdit::singleline(&mut self.input1).hint_text("Ingredient 1"));
                    ui.add(egui::TextEdit::singleline(&mut self.input2).hint_text("Ingredient 2"));
                    ui.add(egui::TextEdit::singleline(&mut self.input3).hint_text("Ingredient 3"));

                    // Filtering Logic
                    let filter1 = self.input1.to_ascii_lowercase();
                    let filter2 = self.input2.to_ascii_lowercase();
                    let filter3 = self.input3.to_ascii_lowercase();

                    let any_cuisine_selected = self.cuisine_filter.values().any(|&v| v);
                    let any_category_selected = self.category_filter.values().any(|&v| v);

                    let mut result: Vec<_> = self
                        .recipes
                        .iter()
                        .filter(|&(_, v)| {
                            if !any_cuisine_selected {
                                return true;
                            }
                            if let Some(cuisine) = &v.cuisine {
                                *self.cuisine_filter.get(cuisine).unwrap_or(&false)
                            } else {
                                false
                            }
                        })
                        .filter(|&(_, v)| {
                            if !any_category_selected {
                                return true;
                            }
                            if let Some(category) = &v.category {
                                *self.category_filter.get(category).unwrap_or(&false)
                            } else {
                                false
                            }
                        })
                        .filter(|&(_, v)| {
                            if filter1.is_empty() {
                                return true;
                            }
                            if let Some(list) = &v.ingredient_list {
                                list.ingredients.iter().any(|e| {
                                    if let Some(key) = &e.key {
                                        key.to_ascii_lowercase().contains(&filter1)
                                    } else {
                                        false
                                    }
                                })
                            } else {
                                false
                            }
                        })
                        .filter(|&(_, v)| {
                            if filter2.is_empty() {
                                return true;
                            }
                            if let Some(list) = &v.ingredient_list {
                                list.ingredients.iter().any(|e| {
                                    if let Some(key) = &e.key {
                                        key.to_ascii_lowercase().contains(&filter2)
                                    } else {
                                        false
                                    }
                                })
                            } else {
                                false
                            }
                        })
                        .filter(|&(_, v)| {
                            if filter3.is_empty() {
                                return true;
                            }
                            if let Some(list) = &v.ingredient_list {
                                list.ingredients.iter().any(|e| {
                                    if let Some(key) = &e.key {
                                        key.to_ascii_lowercase().contains(&filter3)
                                    } else {
                                        false
                                    }
                                })
                            } else {
                                false
                            }
                        })
                        .collect();

                    result.sort_by(|r1, r2| r1.0.cmp(r2.0));

                    // Total count
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Total:")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(178, 178, 178)),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(result.len().to_string())
                                    .size(20.0)
                                    .color(egui::Color32::from_rgb(178, 178, 178)),
                            );
                        });
                    });

                    // Results
                    for (name, _) in result {
                        ui.label(
                            egui::RichText::new(name)
                                .size(20.0)
                                .color(egui::Color32::BLACK),
                        );
                    }
                });
            });
    }
}
