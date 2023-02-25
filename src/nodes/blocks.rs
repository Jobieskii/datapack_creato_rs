use std::sync::LazyLock;

use eframe::epaint::Color32;

#[derive(Clone)]
pub struct Block {
    pub id: &'static str,
    pub color: Color32
}

pub fn default_block_list() -> Vec<&'static Block> {
    vec![
        &Block{id:"minecraft:dirt", color: Color32::BROWN }
    ]
}
pub static BLOCK_LIST: LazyLock<Vec<&Block>> = LazyLock::new(default_block_list);