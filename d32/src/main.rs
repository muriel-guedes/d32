use engine::Terrain;

fn main() {
    let engine = engine::Engine::new();
    Terrain::new(&engine.context);
    engine.start()
}