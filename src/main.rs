extern crate hashbrown;
extern crate rand;
use rand::Rng;

use wld::Wld;

mod wld;
mod structs;
mod read;
mod write;

fn main() {
	let path = String::from("C:\\Users\\benap\\Documents\\My Games\\Terraria\\Worlds\\");
	
	let mut w = Wld::read(&(path.clone() + "1.wld")).unwrap();
	
	let mut good_ids = vec![];
	for i in 0..w.importance.len() {
		if !w.importance[i] {
			good_ids.push(i as u16);
		}
	}
	
	let mut r = rand::thread_rng();
	
	for x in 0..w.width {
		for y in 0..w.height {
			let tile = &w.tiles[(x*w.height + y) as usize];
			let mut new_tile = *tile.clone();
			if let Some(mut block) = tile.block {
				block.id = good_ids[r.gen_range(0..good_ids.len())];
				block.uv = None;
				new_tile.block = Some(block);
			}
			if let Some(mut wall) = tile.wall {
				wall.id = r.gen_range(0..255);
				new_tile.wall = Some(wall);
			}
			w.set_block(x, y, new_tile);
		}
	}
	
	w.name = String::from("weird");
	
	w.write(&(path.clone() + "weird.wld")).unwrap();
	
	//let _w2 = Wld::read(&(path.clone() + "1_block.wld")).unwrap();
}