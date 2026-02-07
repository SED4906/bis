fn main() {
    let mut engine = tailmaw::base::Engine::new();
    engine.load_default_assets();
    engine.retitle_window("Order of the Stone");
    while engine.update() {
    }
}
