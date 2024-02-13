use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ROMS: HashMap<String, Vec<u8>> = {
        let mut ret = HashMap::new();
        ret.insert("TANK".to_string(), include_bytes!("../../roms/TANK").to_vec());
        ret.insert("TETRIS".to_string(), include_bytes!("../../roms/TETRIS").to_vec());
        ret
    };
}
