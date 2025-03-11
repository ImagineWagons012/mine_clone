use crate::{Cardinal, texture::TextureManager};


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Block {
    #[default]
    Air = 0,
    Grass,
    Dirt,
    Stone,
    Cobblestone,
    Bedrock,
}


pub fn get_block_texture_ids(block: Block, texture_manager: &TextureManager) -> [f32; 6] {
    let mut indexes = [0.; 6];
    match block {
        Block::Grass => {
            indexes[Cardinal::Up as usize] = texture_manager.get_id("grass_top".into());
            indexes[Cardinal::Down as usize] = texture_manager.get_id("dirt".into());

            indexes[Cardinal::North as usize..=Cardinal::West as usize]
            .iter_mut()
            .for_each(
                |x| 
                *x = texture_manager.get_id("grass_side".into())
            );
        },
        Block::Dirt => {
            indexes = [texture_manager.get_id("dirt".into()); 6];
        },
        Block::Cobblestone => {
            indexes = [texture_manager.get_id("cobblestone".into()); 6];
        },
        Block::Stone => {
            indexes = [texture_manager.get_id("stone".into()); 6];
        },
        Block::Bedrock => {
            indexes = [texture_manager.get_id("bedrock".into()); 6];
        }
        _ => {}
    }
    indexes
}
