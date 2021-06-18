use nalgebra_glm::TVec3;
use std::sync::Arc;
use crate::world::world::World;
use crate::block::location::BlockLocation;
use std::fmt::{Formatter, Display};
use std::fmt;

pub struct BlockRay {
    origin: TVec3<f64>,
    dir: TVec3<f64>,
    max: f64
}

impl BlockRay {
    pub fn start(&self, world: &World) -> BlockLocation {
        let mut current = 0.0;
        let target = &self.origin + &self.dir;
        let mut new_target = target * 1.0;

        while current < self.max {
            new_target = &self.origin + &self.dir * current;
            let location = BlockLocation::new_from_round(new_target.x,new_target.y,new_target.z);

            let block_type = world.get_type(&location);
            if block_type.is_some() && block_type.unwrap().is_air==false {
                return location;
            }
            current += 0.01;
        }

        return BlockLocation::new_from_round(new_target.x,new_target.y,new_target.z);
    }

    pub unsafe fn start_air(&self, world: &World) -> BlockLocation {
        let mut current = 0.0;
        let target = &self.origin + &self.dir;
        let mut new_target = target * 1.0;

        let mut last = BlockLocation::null();

        while current < self.max {
            new_target = &self.origin + &self.dir * current;
            let location = BlockLocation::new_from_round(new_target.x,new_target.y,new_target.z);

            let block_type = world.get_type(&location);
            if block_type.is_some() && block_type.unwrap().is_air==false {
                return last;
            }
            last = location.clone();
            current += 0.01;
        }

        return BlockLocation::new_from_round(new_target.x,new_target.y,new_target.z);
    }

    pub fn new(origin: TVec3<f64>,dir: TVec3<f64>,max: f64) -> BlockRay {
        return BlockRay {
            origin,
            dir,
            max
        };
    }
}

impl Display for BlockLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{},{},{}",self.x,self.y,self.z)
    }
}