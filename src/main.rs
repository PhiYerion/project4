#![feature(slice_split_once)]
#![feature(let_chains)]
mod app;
mod calculate;
mod op;
mod parse;
mod token;

fn main() {
    app::Calculator::default().run();
}
