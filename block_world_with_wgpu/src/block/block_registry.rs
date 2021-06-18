use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::block::block_type::BlockType;
use std::rc::Rc;
use crate::block::texture_coords::TextureCoords;
use rust_game_library_wgpu::objects::texture_map::TextureMap;

pub struct BlockRegistry{
    pub blocks: HashMap<String,Arc<BlockType>>
}

impl BlockRegistry {
    pub fn register_type(&mut self,block_type: BlockType) {
        self.blocks.insert(block_type.id.to_string(),Arc::new(block_type));
    }

    pub fn register_all(&mut self,block_types: Vec<BlockType>) {
        for b in block_types {
            self.blocks.insert(b.id.to_string(),Arc::new(b));
        }
    }

    pub fn new() -> BlockRegistry {
        return BlockRegistry {
            blocks: HashMap::new()
        }
    }

    pub fn add_all_types(reg:Arc<Mutex<BlockRegistry>>,texture_map:Rc<TextureMap>) {
        let air = BlockType {
            id: "air".to_string(),
            tex_coords: TextureCoords::new_default(),
            is_air: true,
            is_transparent: true,
            is_default_model: true
        };

        let dirt = BlockType {
            id: "dirt".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(0,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let grass = BlockType {
            id: "grass".to_string(),
            tex_coords: TextureCoords::new_sides_top_bottom(texture_map.get_tex_coord(1,0),texture_map.get_tex_coord(2,0),texture_map.get_tex_coord(0,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let stone = BlockType {
            id: "stone".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(3,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let log = BlockType {
            id: "log".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(4,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let cobble = BlockType {
            id: "cobble".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(5,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let leaves = BlockType {
            id: "leaves".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(6,0)),
            is_air: false,
            is_transparent: true,
            is_default_model: true
        };

        let glass = BlockType {
            id: "glass".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(7,0)),
            is_air: false,
            is_transparent: true,
            is_default_model: true
        };

        let plank = BlockType {
            id: "plank".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(8,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let sand = BlockType {
            id: "sand".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(9,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        let snow = BlockType {
            id: "snow".to_string(),
            tex_coords: TextureCoords::new_from_one(texture_map.get_tex_coord(10,0)),
            is_air: false,
            is_transparent: false,
            is_default_model: true
        };

        reg.lock().unwrap().register_all(vec![air,dirt,grass,stone,log,cobble,leaves,glass,plank,sand,snow]);
    }
}