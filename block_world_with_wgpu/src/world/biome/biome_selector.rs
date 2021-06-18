use noise::{OpenSimplex, Seedable, NoiseFn};
use rand::{thread_rng, Rng};
use crate::block::location::BlockLocation;
use crate::world::biome::biome_type::BiomeType;
use std::sync::Arc;
use crate::world::biome::biome_registry::BiomeRegistry;

pub struct BiomeSelector {
    pub noise: OpenSimplex,
    pub biomes_registry: Arc<BiomeRegistry>
}

impl BiomeSelector {
    pub fn new(registry:Arc<BiomeRegistry>) -> Self {
        return Self { noise: OpenSimplex::new().set_seed(thread_rng().gen_range(0..u32::MAX)), biomes_registry: registry }
    }

    pub fn select(&self,loc:&BlockLocation) -> Arc<BiomeType>{
        let mut value = self.noise.get([loc.x as f64 * 0.01,loc.z as f64 * 0.01]);
        value = (value+1.0)/2.0;
        value *= 8.0;

        let biome = self.biomes_registry.get_biome_for_range(value);

        return biome.unwrap();
    }
}