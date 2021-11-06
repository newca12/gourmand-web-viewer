use gourmand_web_viewer::recipe::Recipe;
use iced::{
    text_input, Column, Container, Element, HorizontalAlignment, Length, Row, Sandbox, Settings,
    Text, TextInput,
};

use std::collections::HashMap;

pub fn run_gui() {
    let mut settings = Settings::default();
    settings.window.size = (525u32, 533u32);
    VersatiList::run(settings).unwrap();
}

#[derive(Debug)]
struct VersatiList {
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
    Input1Changed(String),
    Input2Changed(String),
    Input3Changed(String),
}

impl Sandbox for VersatiList {
    type Message = Message;

    fn new() -> Self {
        Self {
            input1: String::new(),
            input2: String::new(),
            input3: String::new(),
            state1: text_input::State::new(),
            state2: text_input::State::new(),
            state3: text_input::State::new(),
            recipes: gourmand_web_viewer::load(false),
            found: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Gourmand web viewer 0.1")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
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
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| e.key.to_ascii_lowercase().contains(&filter1))
            })
            .filter(|&(_, v)| {
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| e.key.to_ascii_lowercase().contains(&filter2))
            })
            .filter(|&(_, v)| {
                v.clone()
                    .ingredient_list
                    .unwrap()
                    .ingredients
                    .iter()
                    .any(|e| e.key.to_ascii_lowercase().contains(&filter3))
            })
            .collect();

        let mut result = result1.clone();
        /*
                for val in &result1 {
                    if !result.contains(val) {
                        result.push((val.0, val.1))
                    };
                }
        */
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
