extern crate hashbrown;

use structs::{Block, Tile, Slope, Liquid};
use wld::Wld;

mod wld;
mod structs;
mod read;
mod write;

fn main() {
	let path = String::from("C:\\Users\\benap\\Documents\\My Games\\Terraria\\Worlds\\");
	
	let mut w = Wld::read(&(path.clone() + "1.wld")).unwrap();
	
	let tile = Tile { block: Some(Block { id: 1, color: None, uv: None, inactive: false, slope: Slope::LowerLeft }), wall: None, liquid: Some(Liquid { kind: structs::LiquidType::Honey, amount: 127 }), red_wire: false, green_wire: true, blue_wire: false, yellow_wire: true, actuator: false };
	for x in 0..200 {
		for y in 0..200 {
			w.set_block(x, y, tile);
		}
	}
	
	w.name = String::from("1 block");
	
	w.write(&(path + "1_block.wld")).unwrap();
}