use iced::{Border, Color, Theme, border::Radius, widget::button, widget::container};

pub fn main_container(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Color::from_rgb8(0xFF, 0xFE, 0xF0).into()),
        ..Default::default()
    }
}

pub fn rounded(theme: &Theme, status: button::Status) -> button::Style {
    let radius = Radius {
        top_left: 12.0,
        top_right: 12.0,
        bottom_left: 12.0,
        bottom_right: 12.0,
    };
    button::Style {
        text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
        border: Border {
            radius,
            width: 1.0,
            color: Color::from_rgb(0.53, 0.0, 0.85),
        },
        ..button::primary(theme, status)
    }
}

pub fn button_filter(theme: &Theme, status: button::Status) -> button::Style {
    match status {
        iced::widget::button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
            text_color: Color::WHITE,
            ..rounded(theme, status)
        },
        _ => button::Style {
            background: Some(Color::from_rgb(0.11, 0.42, 0.87).into()),
            ..rounded(theme, status)
        },
    }
}

pub fn button_filter_inactive(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::BLACK,
        ..rounded(theme, status)
    }
}
