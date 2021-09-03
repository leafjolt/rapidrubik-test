#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::Arc;
use librapidrubik::{Shuffle, Timer};
use druid::widget::prelude::*;
use druid::{AppLauncher, Data, Lens, UnitPoint, WidgetExt, WindowDesc};
use druid::widget::{Button, Align, Flex, Label, TextBox};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Data, Lens)]
struct AppState {
    moves: String,
    shuf: String,        
}

fn main() {
    let win = WindowDesc::new(ui)
        .title("RapidRubik")
        .window_size((1000.0, 700.0));
    let state = AppState {
        moves: 20.to_string(),   
        shuf: String::new(),
    };
    AppLauncher::with_window(win)
        .launch(state)
        .expect("Failed to launch application");
}

fn ui() -> impl Widget<AppState> {
    let textbox = TextBox::new()
        .with_placeholder("Amount of moves in shuffle")
        .fix_width(200.0)
        .lens(AppState::moves);
    let shuffle_label = Label::new(|data: &AppState, _env: &Env| format!("{}", data.shuf)).with_text_size(32.0);
    let button = Button::new("Generate Shuffle").on_click(|_ctx, data: &mut AppState, _env| {
        data.shuf = "".to_string();
        let shuffle = Shuffle::new(data.moves.parse::<u8>().unwrap());
        for i in shuffle.shuffle {
            data.shuf.push_str(&i);
            data.shuf.push_str(" ");
        }
    });
    Align::centered(Flex::column()
        .with_child(Flex::row().with_child(Label::new("Amount of moves:")).with_spacer(10.0).with_child(textbox))
        .with_spacer(20.0)
        .with_child(shuffle_label)
        .with_spacer(20.0)
        .with_child(button))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_main() {
    main()
}
