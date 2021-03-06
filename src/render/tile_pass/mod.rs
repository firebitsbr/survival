use amethyst::assets::Handle;
use amethyst::ecs::prelude::*;
use amethyst::ecs::{Component, Write};
use amethyst::renderer::Texture;
use hashbrown::HashMap;
use specs_derive::Component;

pub use pass::TileRenderPass;

use crate::specs_static::{Id, Storage};
use crate::systems::chunk::Chunk;
use crate::tiles::TileAsset;
use crate::utils::TILE_SIZE;

mod pass;
mod specs;

#[derive(Debug)]
pub struct TextureUsage {
    texture: Handle<Texture>,
    data: Vec<f32>,
    len: u32,
}
pub type WriteChunkRender<'a> = WriteExpect<'a, HashMap<(i32, i32), ChunkRender>>;

#[derive(Debug, Component)]
#[storage(HashMapStorage)]
pub struct ChunkRender {
    inner: Vec<TextureUsage>,
}

pub fn compile_chunk(chunk: &Chunk, tile_specs: &[TileAsset]) -> ChunkRender {
    let mut texture_map: HashMap<usize, TextureUsage> = HashMap::new();
    let (chunk_x, chunk_y) = chunk.pos;

    for x in 0..16 {
        for y in 0..16 {
            let texture_id = chunk.tiles[x][y].0 as usize;
            let asset = &tile_specs[texture_id];
            let slice = [
                (x as f32 + TILE_SIZE * chunk_x as f32) * TILE_SIZE,
                (y as f32 + TILE_SIZE * chunk_y as f32) * TILE_SIZE,
                asset.sprite.tex_coords.left,
                asset.sprite.tex_coords.right,
                asset.sprite.tex_coords.bottom,
                asset.sprite.tex_coords.top,
            ];

            match texture_map.get_mut(&texture_id) {
                Some(usage) => {
                    usage.data.extend(&slice);
                    usage.len += 1;
                }
                None => {
                    let usage = TextureUsage {
                        texture: asset.texture.clone(),
                        data: slice[..].into(),
                        len: 1,
                    };
                    texture_map.insert(texture_id, usage);
                }
            }
        }
    }
    let mut num = 0;
    let mut collected = Vec::with_capacity(texture_map.len());
    for (_, texture_usage) in texture_map {
        num += texture_usage.len;
        collected.push(texture_usage);
    }

    if num != 256 {
        log::error!(
            "Did not get correct number of total tiles rendered: {:?}",
            num
        );
    }
    ChunkRender { inner: collected }
}
