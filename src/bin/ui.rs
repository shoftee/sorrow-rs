fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    sorrow_ui::mount();
}
