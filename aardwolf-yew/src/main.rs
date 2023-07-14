use aardwolf_yew_app::Aardwolf;

fn main() {
    yew::Renderer::<Aardwolf>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}