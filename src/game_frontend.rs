use iced::widget::{Row, Column, Button, button, Text, text, Space, themer};
use iced::{Settings, Color, Program, Alignment, alignment, Background, Theme, Border};
use iced::border::Radius;
use crate::game_backend::{self, ShiftDirection, Backend};

mod cell_colours {
    use super::*;

    fn from_rgb(r:f32,g:f32,b:f32) -> Color {
        return Color {
            r: r/255.0,
            g: g/255.0,
            b: b/255.0,
            a: 255.0
        }
    }

    pub fn get_cell_colour(i: usize) -> Color {
        match i {
            0 => C0(),
            2 => C2(),
            4 => C4(),
            8 => C8(),
            16 => C16(),
            32 => C32(),
            64 => C64(),
            128 => C128(),
            256 => C256(),
            512 => C512(),
            1024 => C1024(),
            2048 => C2048(),
            _ => CM()
        }
    }

    fn C0() -> Color { from_rgb(202.0, 192.0, 179.0) }
    fn C2() -> Color { from_rgb(236.0, 228.0, 218.0) }
    fn C4() -> Color { from_rgb(235.0, 224.0, 200.0) }
    fn C8() -> Color { from_rgb(234.0, 178.0, 121.0) }
    fn C16() -> Color { from_rgb(234.0, 150.0, 99.0) }
    fn C32() -> Color { from_rgb(234.0, 125.0, 97.0) }
    fn C64() -> Color { from_rgb(233.0, 95.0, 61.0) }
    fn C128() -> Color { from_rgb(231.0, 208.0, 113.0) }
    fn C256() -> Color { from_rgb(231.0, 205.0, 96.0) }
    fn C512() -> Color { from_rgb(230.0, 201.0, 77.0) }
    fn C1024() -> Color { from_rgb(230.0, 198.0, 60.0) }
    fn C2048() -> Color { from_rgb(230.0, 195.0, 40.0) }
    fn CM() -> Color { from_rgb(191.0, 94.0, 206.0) }
}


pub struct RS2048 {
    backend: Backend
}
impl Default for RS2048 {
    fn default() -> Self {
        Self {
            backend: Backend::new()
        }
    }
}

impl RS2048 {
    fn make_rows(&self) -> [Row<Message>;4] {
        let mut out = [
            Row::<Message>::new(),
            Row::<Message>::new(),
            Row::<Message>::new(),
            Row::<Message>::new()
        ];

        for row in 0..4 {
            let mut r = Row::<Message>::new();
            for col in 0..4 {
                r = r.push(
                    self.make_button(self.backend.grid_ref()[row][col])
                )
            }
            out[row] = r;
        }
        return out;
    }
    // fn button_coloured_style(&self, i: usize) {
    //     let des_colour = cell_colours::get_cell_colour(i);
    //     button::Style::default().with_background(des_colour)
    // }
    fn coloured_button_style(i:usize) -> impl Fn(&Theme, button::Status) -> button::Style {
        const ROUNDING: u16 = 5;
        move |_theme: &Theme, _status: button::Status| button::Style {
            background: Some(
                Background::Color(
                    cell_colours::get_cell_colour(i)
                )
            ),
            text_color: Default::default(),
            border: Border {
                color: Default::default(),
                width: 0.0,
                radius: Radius::from(ROUNDING),
            },
            shadow: Default::default(),
        }
    }
    fn make_button(&self, i: usize) -> Button<Message> {
        use iced::alignment::{Vertical, Horizontal};

        const SQUARE_BUTTON_SIZE: u16 = 60;
        const TEXT_SIZE: u16 = 30;

        let text = text(i)
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center)
            .size(TEXT_SIZE);

        let des_colour = cell_colours::get_cell_colour(i);

        let button = button(text)
            .width(SQUARE_BUTTON_SIZE)
            .height(SQUARE_BUTTON_SIZE);

        return button.style(
            Self::coloured_button_style(i)
        );

        // themer(des_colour, button);


        // return button.style(
        //     Box<|_theme,_status| button::Style::default().with_background(des_colour)> |_theme,_status| button::Style::default().with_background(des_colour)
        // );

    }

    pub fn view_(&self) -> Column<Message> {
        let mut col: Column<Message> = Column::new();

        let rows: [Row<Message>;4] = self.make_rows();
        for r in rows {
            col = col.push(r)
        };

        col
    }
    pub fn view(&self) -> Column<Message> {
        let mut col: Column<Message> = Column::new();

        col.push(
            Button::new("T")
                .width(60)
                .height(60)
                .style(Self::coloured_button_style(512))
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Shift(d) => {
                self.backend.shift(d)
            }
        };
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Shift(ShiftDirection)
}