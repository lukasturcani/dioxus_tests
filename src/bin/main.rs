fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(dioxus_tests::shared_state_callback::App);
}
