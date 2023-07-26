mod app;
mod calculator;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
