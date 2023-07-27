mod app;
mod calculator;
mod keyboard_event_helper;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
