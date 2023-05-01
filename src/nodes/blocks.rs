use lazy_static::lazy_static;

use eframe::epaint::Color32;

#[derive(Clone)]
pub struct Block {
    pub id: &'static str,
    pub color: Color32
}
//TODO: should be a map???
lazy_static! {
    pub static ref BLOCK_LIST: Vec<Block> = {
        vec![Block{id:"minecraft:dirt", color: Color32::BROWN }]
    };
}
