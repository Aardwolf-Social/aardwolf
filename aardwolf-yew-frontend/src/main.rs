use aardwolf_yew_frontend::Aardwolf;

fn main() {
    yew::Renderer::<Aardwolf>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}
