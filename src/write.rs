use crate::wld::Wld;

fn add<T: number>(v: &mut Vec<u8>, n: &T) {
	
}

pub fn write(w: &Wld) -> Vec<u8> {
	if w.width > 8401 {
		println!("{}: Too wide to open in game, {} / 8400", w.name, w.width);
	}
	if w.height > 2400 {
		println!("{}: Too tall to open in game, {} / 2400", w.name, w.height);
	}
	
	let mut positions = vec![];
	
	let mut prepos = vec![];
	prepos.append(&mut w.version.to_le_bytes().to_vec());
	prepos.append(&mut w.magic_number.to_vec());
	prepos.push(w.file_type.to_le_bytes());
	prepos.append(&mut w.revision.to_le_bytes().to_vec());
	
	
	
	
	
	data
}