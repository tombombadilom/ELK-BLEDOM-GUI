#![allow(non_snake_case)]

mod app;
use app::App;
use dioxus_desktop::{Config, launch::launch};
use dioxus::prelude::*;

fn main() {
    launch(
        App,
        Vec::new(),
        Config::default()
    );
}
