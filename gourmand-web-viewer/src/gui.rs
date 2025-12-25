pub mod style;

use gourmand_web_viewer::recipe::Recipe;
use iced::{
    Element, Length, Size, Task, alignment,
    widget::{Column, Container, Row, Text, button, text_input},
};
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::gui::style::{button_filter, button_filter_inactive};

pub fn run_gui() {
    let app = iced::application(
        GourmandWebViewer::new,
        GourmandWebViewer::update,
        GourmandWebViewer::view,
    )
    .theme(GourmandWebViewer::theme)
    .settings(iced::Settings {
        ..Default::default()
    })
    .title(GourmandWebViewer::title)
    .window(iced::window::Settings {
        size: Size::new(525f32, 800f32),
        exit_on_close_request: true,
        ..Default::default()
    });
    app.run().unwrap();
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

#[derive(Debug, Clone)]
enum Message {
    ToggleFilterCategory(String),
    ToggleFilterCuisine(String),
    Input1Changed(String),
    Input2Changed(String),
    Input3Changed(String),
}

impl GourmandWebViewer {
    fn new() -> (GourmandWebViewer, Task<Message>) {
        let (categories, cuisines, recipes) = gourmand_web_viewer::load_json();

        let categories_buttons: BTreeMap<String, bool> =
            categories.into_iter().map(|c| (c, false)).collect();
        let cuisines_buttons: BTreeMap<String, bool> =
            cuisines.into_iter().map(|c| (c, false)).collect();

        (
            Self {
                categories_buttons,
                cuisines_buttons,
                category_filter: HashMap::new(),
                cuisine_filter: HashMap::new(),
                input1: String::new(),
                input2: String::new(),
                input3: String::new(),
                recipes,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Gourmand web viewer 0.2.0")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleFilterCategory(title) => {
                for (title_from_list, _state) in self.categories_buttons.iter_mut() {
                    if title.eq(title_from_list) {
                        self.category_filter.insert(
                            title.clone(),
                            !self.category_filter.get(&title).unwrap_or(&false),
                        );
                    } else {
                        self.category_filter.insert(title_from_list.clone(), false);
                    }
                }
            }
            Message::ToggleFilterCuisine(title) => {
                for (title_from_list, _state) in self.cuisines_buttons.iter_mut() {
                    if title.eq(title_from_list) {
                        self.cuisine_filter.insert(
                            title.clone(),
                            !self.cuisine_filter.get(&title).unwrap_or(&false),
                        );
                    } else {
                        self.cuisine_filter.insert(title_from_list.clone(), false);
                    }
                }
            }
            Message::Input1Changed(new_value) => {
                self.input1 = new_value.to_ascii_lowercase();
            }
            Message::Input2Changed(new_value) => {
                self.input2 = new_value.to_ascii_lowercase();
            }
            Message::Input3Changed(new_value) => {
                self.input3 = new_value.to_ascii_lowercase();
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mut categorie_filter = Row::new();
        let mut cuisine_filter = Row::new();

        for (title, _state) in self.categories_buttons.iter() {
            let is_selected = *self.category_filter.get(title).unwrap_or(&false);
            categorie_filter = categorie_filter
                .push(
                    button(
                        Text::new(title.as_str())
                            .align_x(alignment::Horizontal::Center)
                            .size(16),
                    )
                    .padding(8)
                    .on_press(Message::ToggleFilterCategory(title.to_string()))
                    .style(if is_selected {
                        button_filter
                    } else {
                        button_filter_inactive
                    }),
                )
                .padding(8);
        }

        for (title, _state) in self.cuisines_buttons.iter() {
            let is_selected = *self.cuisine_filter.get(title).unwrap_or(&false);
            cuisine_filter = cuisine_filter
                .push(
                    button(
                        Text::new(title.as_str())
                            .align_x(alignment::Horizontal::Center)
                            .size(16),
                    )
                    .padding(8)
                    .on_press(Message::ToggleFilterCuisine(title.to_string()))
                    .style(if is_selected {
                        button_filter
                    } else {
                        button_filter_inactive
                    }),
                )
                .padding(8);
        }

        let filter1 = self.input1.clone();
        let filter2 = self.input2.clone();
        let filter3 = self.input3.clone();

        let text_input1 = text_input("Ingredient 1", &filter1)
            .on_input(Message::Input1Changed)
            .size(20);
        let text_input2 = text_input("Ingredient 2", &filter2)
            .on_input(Message::Input2Changed)
            .size(20);
        let text_input3 = text_input("Ingredient 3", &filter3)
            .on_input(Message::Input3Changed)
            .size(20);

        let recipes1 = &self.recipes;

        let any_cuisine_selected = self.cuisine_filter.values().any(|&v| v);
        let any_category_selected = self.category_filter.values().any(|&v| v);

        let mut result1: Vec<_> = recipes1
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

        result1.sort_by(|r1, r2| r1.0.cmp(r2.0));

        let result = result1
            .iter()
            .fold(Column::new(), |column, recipe| -> Column<Message> {
                column.push(Text::new(recipe.0.as_str()).size(20))
            });

        let input1 = Column::new().push(text_input1).padding(4);
        let input2 = Column::new().push(text_input2).padding(4);
        let input3 = Column::new().push(text_input3).padding(4);

        let total = Column::new().push(
            Text::new("Total:")
                .align_x(alignment::Horizontal::Left)
                .size(20)
                .color([0.7, 0.7, 0.7]),
        );
        let row4 = Row::new().push(total).push(
            Text::new(result1.len().to_string())
                .align_x(alignment::Horizontal::Right)
                .size(20)
                .color([0.7, 0.7, 0.7]),
        );
        let content = iced::widget::scrollable(
            Column::new()
                .push(categorie_filter)
                .push(cuisine_filter)
                .push(input1)
                .push(input2)
                .push(input3)
                .push(row4)
                .push(result),
        );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .style(crate::gui::style::main_container)
            .into()
    }
}
