use common::chunk::Chunk;

use noise::{BasicMulti, Perlin};
use vek::Vec2;

pub struct WorldGenerator {
    gen: BasicMulti<Perlin>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            gen: BasicMulti::new(88),
        }
    }

    pub fn generate_chunk(&self, offset: Vec2<i32>) -> Chunk {
        Chunk::generate(&self.gen, offset)
        // Chunk::flat(common::block::BlockId::Dirt)
    }
}
