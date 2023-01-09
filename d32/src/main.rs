fn main() {
    let engine = engine::Engine::new();
    {
        engine.context.add_object(1);
    }
    engine.start()
}