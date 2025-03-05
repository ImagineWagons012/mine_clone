use std::collections::HashMap;

use crate::{Cardinal, texture::TextureManager};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Block {
    #[default]
    Air,
    Grass,
    CobbleStone,
}

pub fn get_block_texture_ids(block: Block, texture_manager: &TextureManager) -> HashMap<Cardinal, f32> {
    let mut map = HashMap::new();
    match block {
        Block::Grass => {
            map.insert(Cardinal::North, texture_manager.get_id("grass_side".into()));
            map.insert(Cardinal::South, texture_manager.get_id("grass_side".into()));
            map.insert(Cardinal::East, texture_manager.get_id("grass_side".into()));
            map.insert(Cardinal::West, texture_manager.get_id("grass_side".into()));
            map.insert(Cardinal::Up, texture_manager.get_id("grass_top".into()));
            map.insert(Cardinal::Down, texture_manager.get_id("dirt".into()));
        },
        Block::CobbleStone => {
            map.insert(Cardinal::North, texture_manager.get_id("cobblestone".into()));
            map.insert(Cardinal::South, texture_manager.get_id("cobblestone".into()));
            map.insert(Cardinal::East, texture_manager.get_id("cobblestone".into()));
            map.insert(Cardinal::West, texture_manager.get_id("cobblestone".into()));
            map.insert(Cardinal::Up, texture_manager.get_id("cobblestone".into()));
            map.insert(Cardinal::Down, texture_manager.get_id("cobblestone".into()));
        }
        _ => {}
    }
    map
}
