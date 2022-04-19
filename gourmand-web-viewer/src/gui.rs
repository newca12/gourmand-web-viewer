use gourmand_web_viewer::recipe::Recipe;
use iced::HorizontalAlignment;
use iced::{
    button, text_input, Button, Column, Container, Element, Length, Row, Sandbox, Settings, Text,
    TextInput,
};

use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (525u32, 533u32);
    GourmandWebViewer::run(settings).unwrap();
}

#[derive(Debug)]
struct GourmandWebViewer {
    categories_buttons: BTreeMap<String, button::State>,
    cuisines_buttons: BTreeMap<String, button::State>,
    category_filter: HashMap<String, bool>,
    cuisine_filter: HashMap<String, bool>,
    input1: String,
    input2: String,
    input3: String,
    state1: text_input::State,
    state2: text_input::State,
    state3: text_input::State,
    recipes: HashMap<String, Recipe>,
    found: usize,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleFilterCategory(String),
    ToggleFilterCuisine(String),
    Input1Changed(String),
    Input2Changed(String),
    Input3Changed(String),
}

impl Sandbox for GourmandWebViewer {
    type Message = Message;

    fn new() -> Self {
        let (categories, cuisines, recipes) = gourmand_web_viewer::load_json();
        let mut cuisines_buttons = BTreeMap::new();
        for c in cuisines {
            cuisines_buttons.insert(c, button::State::new());
        }
        let mut categories_buttons = BTreeMap::new();
        for c in categories {
            categories_buttons.insert(c, button::State::new());
        }

        Self {
            categories_buttons,
            cuisines_buttons,
            category_filter: HashMap::new(),
            cuisine_filter: HashMap::new(),
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            state1: text_input::State::new(),
            state2: text_input::State::new(),
            state3: text_input::State::new(),
            recipes,
            found: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Gourmand web viewer 0.1")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ToggleFilterCategory(title) => {
                self.category_filter.insert(
                    title.clone(),
                    !self.category_filter.get(&title).unwrap_or(&false),
                );
            }
            Message::ToggleFilterCuisine(title) => {
                self.cuisine_filter.insert(
                    title.clone(),
                    !self.cuisine_filter.get(&title).unwrap_or(&false),
                );
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
    }

    fn view(&mut self) -> Element<Message> {
        let mut categorie_filter = Row::new();
        let mut cuisine_filter = Row::new();

        for (title, state) in self.categories_buttons.iter_mut() {
            if *self.category_filter.get(title).unwrap_or(&false) {
                categorie_filter = categorie_filter
                    .push(
                        Button::new(
                            state,
                            Text::new(title)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(16),
                        )
                        .padding(8)
                        .on_press(Message::ToggleFilterCategory(title.to_string()))
                        .style(style::Button::Selected),
                    )
                    .padding(8);
            } else {
                categorie_filter = categorie_filter
                    .push(
                        Button::new(
                            state,
                            Text::new(title)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(16),
                        )
                        .padding(8)
                        .on_press(Message::ToggleFilterCategory(title.to_string()))
                        .style(style::Button::UnSelected),
                    )
                    .padding(8);
            }
        }

        for (title, state) in self.cuisines_buttons.iter_mut() {
            if *self.cuisine_filter.get(title).unwrap_or(&false) {
                cuisine_filter = cuisine_filter
                    .push(
                        Button::new(
                            state,
                            Text::new(title)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(16),
                        )
                        .padding(8)
                        .on_press(Message::ToggleFilterCuisine(title.to_string()))
                        .style(style::Button::Selected),
                    )
                    .padding(8);
            } else {
                cuisine_filter = cuisine_filter
                    .push(
                        Button::new(
                            state,
                            Text::new(title)
                                .horizontal_alignment(HorizontalAlignment::Center)
                                .size(16),
                        )
                        .padding(8)
                        .on_press(Message::ToggleFilterCuisine(title.to_string()))
                        .style(style::Button::UnSelected),
                    )
                    .padding(8);
            }
        }

        let filter1 = self.input1.clone();
        let text_input1 = TextInput::new(
            &mut self.state1,
            "Ingredient 1",
            &self.input1,
            Message::Input1Changed,
        );
        let filter2 = self.input2.clone();
        let text_input2 = TextInput::new(
            &mut self.state2,
            "Ingredient 2",
            &self.input2,
            Message::Input2Changed,
        );
        let filter3 = self.input3.clone();
        let text_input3 = TextInput::new(
            &mut self.state3,
            "Ingredient 3",
            &self.input3,
            Message::Input3Changed,
        );
        let recipes1 = self.recipes.clone();

        let result1: Vec<_> = recipes1
            .iter()
            .filter(|&(_, v)| {
                let mut selected = false;
                for (_, select) in self.cuisine_filter.clone() {
                    if select {
                        selected = true
                    }
                }
                !selected
                    || (v.clone().cuisine.is_some()
                        && *self
                            .cuisine_filter
                            .get(&v.clone().cuisine.unwrap())
                            .unwrap_or(&false))
            })
            .filter(|&(_, v)| {
                let mut selected = false;
                for (_, select) in self.category_filter.clone() {
                    if select {
                        selected = true
                    }
                }
                !selected
                    || (v.clone().category.is_some()
                        && *self
                            .category_filter
                            .get(&v.clone().category.unwrap())
                            .unwrap_or(&false))
                // &&
            })
            .filter(|&(_, v)| {
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| {
                        e.key
                            .as_ref()
                            .unwrap()
                            .to_ascii_lowercase()
                            .contains(&filter1)
                    })
            })
            .filter(|&(_, v)| {
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| {
                        e.key
                            .as_ref()
                            .unwrap()
                            .to_ascii_lowercase()
                            .contains(&filter2)
                    })
            })
            .filter(|&(_, v)| {
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| {
                        e.key
                            .as_ref()
                            .unwrap()
                            .to_ascii_lowercase()
                            .contains(&filter3)
                    })
            })
            .collect();

        let mut result = result1.clone();

        self.found = result.len();
        let result = result
            .iter_mut()
            .fold(Column::new(), |column, recipe| -> Column<Message> {
                column.push(Text::new(recipe.0))
            });
        let input1 = Column::new().push(text_input1).padding(4);
        let input2 = Column::new().push(text_input2).padding(4);
        let input3 = Column::new().push(text_input3).padding(4);
        let total = Column::new().push(
            Text::new("Total:")
                .horizontal_alignment(HorizontalAlignment::Center)
                .color([0.7, 0.7, 0.7]),
        );
        let row4 = Row::new().push(total).push(
            Text::new(&self.found.to_string())
                .horizontal_alignment(HorizontalAlignment::Center)
                .color([0.7, 0.7, 0.7]),
        );
        let content = Column::new()
            .push(categorie_filter)
            .push(cuisine_filter)
            .push(input1)
            .push(input2)
            .push(input3)
            .push(row4)
            .push(result);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Selected,
        UnSelected,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Selected => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::UnSelected => Color::from_rgb(0.87, 0.87, 0.87),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: match self {
                    Button::Selected => Color::WHITE,
                    Button::UnSelected => Color::BLACK,
                },
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: match self {
                    Button::Selected => Color::from_rgb(0.87, 0.87, 0.87),
                    Button::UnSelected => Color::from_rgb(0.11, 0.42, 0.87),
                },
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
