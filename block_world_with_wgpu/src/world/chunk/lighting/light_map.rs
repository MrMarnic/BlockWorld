use std::collections::HashMap;
use crate::block::location::BlockLocation;
use crate::world::chunk::lighting::light_state::LightState;
use crate::world::chunk::lighting::light_source::LightSource;
use crate::world::chunk::chunk::Chunk;
use crate::world::chunk::chunk_handler::ChunkHandler;

pub struct LightMap {
    states:HashMap<BlockLocation,LightState>,
    sources:HashMap<BlockLocation,LightSource>
}

impl LightMap {
    /*
    pub fn generate(chunk:&Chunk,chunk_handler:&ChunkHandler) -> LightMap {

        let mut sources : HashMap<BlockLocation,LightSource> = HashMap::new();
        let mut states : HashMap<BlockLocation,LightState> = HashMap::new();

        for rc in chunk.render_chunks.iter() {
            for (x,bbb) in rc.blocks.iter().enumerate() {
                for (y,bb) in bbb.iter().enumerate() {
                    for (z,b) in bb.iter().enumerate() {
                        if b.is_air {
                            let loc = BlockLocation::new_from(x as i32,y as i32,z as i32);
                            if chunk.is_no_block_above(&loc) {
                                //sources.insert(loc.clone(),LightSource::new(15))
                            }
                        }
                    }
                }
            }
        }

        fn set_light_level(chunk:&Chunk,loc:&BlockLocation,level:i32,states:&mut HashMap<BlockLocation,LightState>) {
            if level != 0 {
                if states.contains_key(&loc) {
                    let already = &mut states[&loc];

                    if level > already.level {
                        already.level = level;
                    } else {
                        return;
                    }
                } else {
                    states.insert(loc.clone(),LightState::new(level));
                }
                for l in loc.get_neighbours() {
                    if chunk.is_air(loc) {
                        set_light_level(chunk,&l,level-1,states);
                    }
                }
            }
        }

        for (loc,s) in &sources {
            for l in loc.get_neighbours() {
                set_light_level(chunk,&l,s.level,&mut states);
            }
        }

        return LightMap { states, sources }
    }
     */
}