#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tile {
	pub block: Option<Block>,
	pub wall: Option<Wall>,
	pub liquid: Option<Liquid>,
	pub red_wire: bool,
	pub green_wire: bool,
	pub blue_wire: bool,
	pub yellow_wire: bool,
	pub actuator: bool
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Block {
	pub id: u16,
	pub color: Option<u8>,
	pub uv: Option<(u16, u16)>,
	pub inactive: bool,
	pub slope: Slope
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Slope {
	Full,
	Half,
	LowerLeft,
	LowerRight,
	UpperLeft,
	UpperRight
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Wall {
	pub id: u16,
	pub color: Option<u8>
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Liquid {
	pub kind: LiquidType,
	pub amount: u8
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum LiquidType {
	Water,
	Lava,
	Honey,
	Shimmer
}



#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Chest {
	pub name: String,
	pub x: u32,
	pub y: u32,
	pub items: Vec<Option<Item>>
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Item {
	pub id: u32,
	pub prefix: u8,
	pub count: u16
}



#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Sign {
	pub x: u32,
	pub y: u32,
	pub text: String
}



#[derive(PartialEq, Clone)]
pub struct NPC {
	pub id: u32,
	pub name: String,
	pub x: f32,
	pub y: f32,
	pub homeless: bool,
	pub home_x: u32,
	pub home_y: u32,
	pub variation_index: u32,
	pub shimmered: bool,
	pub is_pillar: bool
}



#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct TileEntity {
	pub id: u32,
	pub x: u16,
	pub y: u16,
	pub info: TileEntityInfo
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum TileEntityInfo {
	TargetDummy(u16),
	ItemFrame(Option<Item>),
	LogicSensor(u8, bool),
	Mannequin([Option<Item>; 16]),
	WeaponRack(Option<Item>),
	HatRack([Option<Item>; 4]),
	FoodPlatter(Option<Item>),
	Pylon
}



#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct NPCRoom {
	pub id: u32,
	pub x: u32,
	pub y: u32
}



