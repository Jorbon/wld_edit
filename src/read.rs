use std::{collections::VecDeque, rc::Rc};
use hashbrown::hash_set::HashSet;

use crate::{wld::Wld, structs::{Block, Tile, Slope, Wall, Liquid, LiquidType, Chest, Item, Sign, NPC, TileEntity, TileEntityInfo, NPCRoom, CreativePower}};

struct Reader {
	deque: VecDeque<u8>,
	original_length: usize,
	pub positions: Vec<usize>
}

impl Reader {
	pub fn new(buffer: Vec<u8>) -> Self {
		Self {
			original_length: buffer.len(),
			deque: VecDeque::from(buffer),
			positions: vec![]
		}
	}
	
	pub fn _remaining(&self) -> usize {
		self.deque.len()
	}
	pub fn position(&self) -> usize {
		self.original_length - self.deque.len()
	}
	
	pub fn check_position(&self, i: usize) -> Result<(), String> {
		match self.position() == self.positions[i] {
			true => Ok(()),
			false => Err(format!("Position {} is wrong: {} from file, {} from data, off by {}", i, self.positions[i], self.position(), self.position() - self.positions[i]))
		}
	}
	
	pub fn u8(&mut self) -> u8 {
		self.deque.pop_front().unwrap()
	}
	pub fn u16(&mut self) -> u16 {
		self.deque.pop_front().unwrap() as u16 | ((self.deque.pop_front().unwrap() as u16) << 8)
	}
	pub fn u32(&mut self) -> u32 {
		(0..4).map(|i| (self.deque.pop_front().unwrap() as u32) << i*8).sum()
	}
	pub fn u64(&mut self) -> u64 {
		(0..8).map(|i| (self.deque.pop_front().unwrap() as u64) << i*8).sum()
	}
	pub fn u128(&mut self) -> u128 {
		(0..16).map(|i| (self.deque.pop_front().unwrap() as u128) << i*8).sum()
	}
	pub fn f32(&mut self) -> f32 {
		f32::from_bits(self.u32())
	}
	pub fn f64(&mut self) -> f64 {
		f64::from_bits(self.u64())
	}
	pub fn bool(&mut self) -> bool {
		self.deque.pop_front().unwrap() != 0
	}
	pub fn string(&mut self) -> String {
		String::from_utf8((0..self.u8()).map(|_| self.u8()).collect()).unwrap()
	}
}

pub fn read(buffer: Vec<u8>) -> Wld {
	let mut r = Reader::new(buffer);
	
	let version = r.u32();
	let magic_number = [r.u8(), r.u8(), r.u8(), r.u8(), r.u8(), r.u8(), r.u8()];
	let file_type = r.u8();
	let revision = r.u32();
	let is_favorite = r.u64();
	
	r.positions = (0..r.u16()).map(|_| r.u32() as usize).collect();
	
	let importance_len = r.u16();
	let mut importance = vec![];
	let mut i = 0;
	let mut n = 0;
	while importance.len() < importance_len as usize {
		if i == 0 {
			n = r.u8();
			i = 8;
		}
		importance.push(n & 1 == 1);
		n >>= 1;
		i -= 1;
	}
	
	r.check_position(0).unwrap();
	
	let name = r.string();
	println!("{name}: Reading header");
	
	let seed = r.string();
	let world_gen_version = r.u64();
	let guid = r.u128();
	let id = r.u32();
	let left = r.u32();
	let right = r.u32();
	let top = r.u32();
	let bottom = r.u32();
	let height = r.u32();
	let width = r.u32();
	let gamemode = r.u32();
	let drunk_world = r.bool();
	let good_world = r.bool();
	let tenth_anniversary_world = r.bool();
	let dont_starve_world = r.bool();
	let notthebees_world = r.bool();
	let remix_world = r.bool();
	let notraps_world = r.bool();
	let zenith_world = r.bool();
	let creation_time = r.u64();
	let moon_type = r.u8();
	let tree_type_xcoords = [r.u32(), r.u32(), r.u32()];
	let tree_types = [r.u32(), r.u32(), r.u32(), r.u32()];
	let cave_bg_xcoords = [r.u32(), r.u32(), r.u32()];
	let cave_bgs = [r.u32(), r.u32(), r.u32(), r.u32()];
	let ice_bg = r.u32();
	let jungle_bg = r.u32();
	let hell_bg = r.u32();
	let spawn_x = r.u32();
	let spawn_y = r.u32();
	let world_surface_y = r.f64();
	let rock_layer_y = r.f64();
	let game_time = r.f64();
	let is_day = r.bool();
	let moon_phase = r.u32();
	let blood_moon = r.bool();
	let eclipse = r.bool();
	let dungeon_x = r.u32();
	let dungeon_y = r.u32();
	let crimson_world = r.bool();
	let killed_eye_of_cthulu = r.bool();
	let killed_eater_of_worlds = r.bool();
	let killed_skeletron = r.bool();
	let killed_queen_bee = r.bool();
	let killed_the_destroyer = r.bool();
	let killed_the_twins = r.bool();
	let killed_skeletron_prime = r.bool();
	let killed_any_hardmode_boss = r.bool();
	let killed_plantera = r.bool();
	let killed_golem = r.bool();
	let killed_slime_king = r.bool();
	let saved_goblin_tinkerer = r.bool();
	let saved_wizard = r.bool();
	let saved_mechanic = r.bool();
	let defeated_goblin_invasion = r.bool();
	let killed_clown = r.bool();
	let defeated_frost_legion = r.bool();
	let defeated_pirates = r.bool();
	let broken_shadow_orb = r.bool();
	let meteor_spawned = r.bool();
	let shadow_orbs_broken_mod3 = r.u8();
	let altars_smashed = r.u32();
	let hard_mode = r.bool();
	let after_party_of_doom = r.bool();
	let goblin_invasion_delay = r.u32();
	let goblin_invasion_size = r.u32();
	let goblin_invasion_type = r.u32();
	let goblin_invasion_x = r.f64();
	let slime_rain_time = r.f64();
	let sundial_cooldown = r.u8();
	let is_raining = r.bool();
	let rain_time = r.u32();
	let max_rain = r.f32();
	let tier_1_ore_id = r.u32();
	let tier_2_ore_id = r.u32();
	let tier_3_ore_id = r.u32();
	let tree_style = r.u8();
	let corruption_style = r.u8();
	let jungle_style = r.u8();
	let snow_style = r.u8();
	let hallow_style = r.u8();
	let crimson_style = r.u8();
	let desert_style = r.u8();
	let ocean_style = r.u8();
	let cloud_bg = r.u32();
	let num_clouds = r.u16();
	let wind_speed = r.f32();
	
	let angler_finishers = (0..r.u32()).map(|_| r.string()).collect();
	
	let saved_angler = r.bool();
	let angler_quest = r.u32();
	let saved_stylist = r.bool();
	let saved_tax_collector = r.bool();
	let saved_golfer = r.bool();
	let invasion_size_start = r.u32();
	let temp_cultist_delay = r.u32();
	
	let kill_counts = (0..r.u16()).map(|_| r.u32()).collect();
	
	let fast_forward_time = r.bool();
	let downed_fishron = r.bool();
	let downed_martians = r.bool();
	let downed_ancient_cultist = r.bool();
	let downed_moonlord = r.bool();
	let downed_halloween_king = r.bool();
	let downed_halloween_tree = r.bool();
	let downed_christmas_ice_queen = r.bool();
	let downed_christmas_santank = r.bool();
	let downed_christmas_tree = r.bool();
	let downed_tower_solar = r.bool();
	let downed_tower_vortex = r.bool();
	let downed_tower_nebula = r.bool();
	let downed_tower_stardust = r.bool();
	let tower_active_solar = r.bool();
	let tower_active_vortex = r.bool();
	let tower_active_nebula = r.bool();
	let tower_active_stardust = r.bool();
	let lunar_apocalypse_is_up = r.bool();
	let party_manual = r.bool();
	let party_genuine = r.bool();
	let party_cooldown = r.u32();
	
	let party_celebrating_npcs = (0..r.u32()).map(|_| r.u32()).collect();
	
	let sandstorm_happening = r.bool();
	let sandstorm_time_left = r.u32();
	let sandstorm_severity = r.f32();
	let sandstorm_intended_severity = r.f32();
	let saved_bartender = r.bool();
	let downed_invasion_tier_1 = r.bool();
	let downed_invasion_tier_2 = r.bool();
	let downed_invasion_tier_3 = r.bool();
	let mushroom_bg = r.u8();
	let underworld_bg = r.u8();
	let tree2_bg = r.u8();
	let tree3_bg = r.u8();
	let tree4_bg = r.u8();
	let combat_book_was_used = r.bool();
	let lantern_night_stuff = r.u32();
	let lantern_night_more_stuff = [r.bool(), r.bool(), r.bool()];
	
	let tree_top_stuff = (0..u32::min(r.u32(), 13)).map(|_| r.u32()).collect();
	
	let force_halloween_for_today = r.bool();
	let force_xmas_for_today = r.bool();
	let copper_tier = r.u32();
	let iron_tier = r.u32();
	let silver_tier = r.u32();
	let gold_tier = r.u32();
	let bought_cat = r.bool();
	let bought_dog = r.bool();
	let bought_bunny = r.bool();
	let downed_empress_of_light = r.bool();
	let downed_queen_slime = r.bool();
	let downed_deerclops = r.bool();
	let unlocked_slime_blue_spawn = r.bool();
	let unlocked_merchant_spawn = r.bool();
	let unlocked_demolitionist_spawn = r.bool();
	let unlocked_party_girl_spawn = r.bool();
	let unlocked_dye_trader_spawn = r.bool();
	let unlocked_truffle_spawn = r.bool();
	let unlocked_arms_dealer_spawn = r.bool();
	let unlocked_nurse_spawn = r.bool();
	let unlocked_princess_spawn = r.bool();
	let combat_book_v2_was_used = r.bool();
	let peddlers_satched_was_used = r.bool();
	let unlocked_slime_green_spawn = r.bool();
	let unlocked_slime_old_spawn = r.bool();
	let unlocked_slime_purple_spawn = r.bool();
	let unlocked_slime_rainbow_spawn = r.bool();
	let unlocked_slime_red_spawn = r.bool();
	let unlocked_slime_yellow_spawn = r.bool();
	let unlocked_slime_copper_spawn = r.bool();
	let fast_forward_to_dusk = r.bool();
	let moondial_cooldown = r.u8();
	
	r.check_position(1).unwrap();
	println!("{name}: Reading tiles");
	
	
	let mut tile_set = HashSet::new();
	let mut tiles = vec![];
	
	let mut progress = 0;
	
	for x in 0..width {
		let mut y = 0;
		while y < height {
			let a = r.u8();
			let b = match a & 1 == 1 {
				true => r.u8(),
				false => 0
			};
			let c = match b & 1 == 1 {
				true => r.u8(),
				false => 0
			};
			
			if c & 1 == 1 || b & 128 == 128 { panic!() }
			
			let tile = Rc::new(Tile {
				block: (a & 2 == 2).then(|| {
					let id = match a & 32 == 32 {
						true => r.u16(),
						false => r.u8() as u16
					};
					Block {
						id,
						uv: importance[id as usize].then(|| (r.u16(), r.u16())),
						color: (c & 8 == 8).then(|| r.u8()),
						inactive: c & 4 == 4,
						slope: match (b >> 4) & 7 {
							0 => Slope::Full,
							1 => Slope::Half,
							2 => Slope::LowerLeft,
							3 => Slope::LowerRight,
							4 => Slope::UpperLeft,
							5 => Slope::UpperRight,
							_ => Slope::Full
						}
					}
				}),
				wall: 
					match a & 4 == 4 {
						true => Some(Wall {
							id: match c & 64 == 64 {
								true => r.u16(),
								false => r.u8() as u16
							},
							color: (c & 16 == 16).then(|| r.u8())
						}),
						false => None
					},
				liquid: ((a >> 3) & 3 > 0).then(|| Liquid {
					kind: match c & 128 == 128 {
						true => LiquidType::Shimmer,
						false => match (a >> 3) & 3 {
							2 => LiquidType::Lava,
							3 => LiquidType::Honey,
							_ => LiquidType::Water
						}
					},
					amount: r.u8()
				}),
				red_wire: b & 2 == 2,
				green_wire: b & 4 == 4,
				blue_wire: b & 8 == 8,
				yellow_wire: c & 32 == 32,
				actuator: c & 2 == 2
			});
			
			let p = tile_set.get_or_insert(tile);
			
			let k = match a & 128 == 128 {
				true => r.u16() as u32,
				false => match a & 64 == 64 {
					true => r.u8() as u32,
					false => 0
				}
			};
			
			for _ in 0..k+1 {
				tiles.push(Rc::clone(p));
			}
			y += k + 1;
			
		}
		
		if y != height { panic!() }
		
		if 10 * x / width > progress {
			progress = 10 * x / width;
			println!("{progress}0%")
		}
	}
	
	r.check_position(2).unwrap();
	println!("{name}: Reading chests");
	
	
	let num = r.u16();
	if r.u16() != 40 { panic!() }
	
	let chests = (0..num).map(|_| {
		Chest {
			x: r.u32(),
			y: r.u32(),
			name: r.string(),
			items:
				(0..40).map(|_| {
					let count = r.u16();
					(count > 0).then(|| Item { id: r.u32(), prefix: r.u8(), count })
				}).collect()
		}
	}).collect();
	
	r.check_position(3).unwrap();
	println!("{name}: Reading signs");
	
	
	let signs = (0..r.u16()).map(|_| Sign { text: r.string(), x: r.u32(), y: r.u32() }).collect();
	
	r.check_position(4).unwrap();
	println!("{name}: Reading npcs");
	
	
	let mut npcs = vec![];
	let shimmered: Vec<usize> = (0..r.u32()).map(|_| r.u32() as usize).collect();
	
	while r.bool() {
		npcs.push(NPC {
			id: r.u32(),
			name: r.string(),
			x: r.f32(),
			y: r.f32(),
			homeless: r.bool(),
			home_x: r.u32(),
			home_y: r.u32(),
			variation_index: match r.bool() {
				true => r.u32(),
				false => 0
			},
			shimmered: false,
			is_pillar: false
		});
	}
	
	for i in shimmered {
		npcs[i].shimmered = true;
	}
	
	while r.bool() {
		npcs.push(NPC {
			id: r.u32(),
			x: r.f32(),
			y: r.f32(),
			is_pillar: true,
			name: String::new(),
			homeless: false,
			home_x: 0,
			home_y: 0,
			variation_index: 0,
			shimmered: false
		});
	}
	
	
	r.check_position(5).unwrap();
	println!("{name}: Reading misc");
	
	
	let tile_entities = (0..r.u32()).map(|_| {
		let t = r.u8();
		TileEntity {
			id: r.u32(),
			x: r.u16(),
			y: r.u16(),
			info: match t {
				0 => TileEntityInfo::TargetDummy(r.u16()),
				1 => TileEntityInfo::ItemFrame({
					let id = r.u16() as u32;
					let prefix = r.u8();
					let count = r.u16();
					(count > 0).then(|| Item { id, prefix, count })
				}),
				2 => TileEntityInfo::LogicSensor(r.u8(), r.bool()),
				3 => TileEntityInfo::Mannequin({
					let mut buffer = [None; 16];
					let slots = r.u16();
					let mut i = 0;
					for item in (0..16).map(|i| ((slots >> i) & 1 == 1).then(
						|| Item { id: r.u16() as u32, prefix: r.u8(), count: r.u16() }
					)) {
						buffer[i] = item;
						i += 1;
					};
					buffer
				}),
				4 => TileEntityInfo::WeaponRack({
					let id = r.u16() as u32;
					let prefix = r.u8();
					let count = r.u16();
					(count > 0).then(|| Item { id, prefix, count })
				}),
				5 => TileEntityInfo::HatRack({
					let mut buffer = [None; 4];
					let slots = r.u8();
					let mut i = 0;
					for item in (0..4).map(|i| ((slots >> i) & 1 == 1).then(
						|| Item { id: r.u16() as u32, prefix: r.u8(), count: r.u16() }
					)) {
						buffer[i] = item;
						i += 1;
					};
					buffer
				}),
				6 => TileEntityInfo::FoodPlatter({
					let id = r.u16() as u32;
					let prefix = r.u8();
					let count = r.u16();
					(count > 0).then(|| Item { id, prefix, count })
				}),
				7 => TileEntityInfo::Pylon,
				_ => panic!()
			}
		}
	}).collect();
	
	r.check_position(6).unwrap();
	
	
	let weighted_pressure_plates = (0..r.u32()).map(|_| (r.u32(), r.u32())).collect();
	
	r.check_position(7).unwrap();
	
	
	let npc_rooms = (0..r.u32()).map(|_| NPCRoom { id: r.u32(), x: r.u32(), y: r.u32() }).collect();
	
	r.check_position(8).unwrap();
	
	
	let bestiary_kills = (0..r.u32()).map(|_| (r.string(), r.u32())).collect();
	let bestiary_sights = (0..r.u32()).map(|_| r.string()).collect();
	let bestiary_chats = (0..r.u32()).map(|_| r.string()).collect();
	
	r.check_position(9).unwrap();
	
	
	let mut creative_powers = vec![];
	while r.bool() {
		creative_powers.push(match r.u16() {
			0 => CreativePower::FreezeTime(r.bool()),
			8 => CreativePower::TimeRate(r.f32()),
			9 => CreativePower::FreezeWeather(r.bool()),
			10 => CreativePower::FreezeWind(r.bool()),
			12 => CreativePower::DifficultySlider(r.f32()),
			13 => CreativePower::FreezeSpread(r.bool()),
			_ => panic!()
		});
	}
	
	r.check_position(10).unwrap();
	
	if !r.bool() || r.string() != name || r.u32() != id { panic!() }
	println!("{name}: Done");
	
	
	Wld {
		version, magic_number, file_type, revision, is_favorite, importance, name, seed, world_gen_version, guid, id, left, right, top, bottom, height, width, gamemode, drunk_world, good_world, tenth_anniversary_world, dont_starve_world, notthebees_world, remix_world, notraps_world, zenith_world, creation_time, moon_type, tree_type_xcoords, tree_types, cave_bg_xcoords, cave_bgs, ice_bg, jungle_bg, hell_bg, spawn_x, spawn_y, world_surface_y, rock_layer_y, game_time, is_day, moon_phase, blood_moon, eclipse, dungeon_x, dungeon_y, crimson_world, killed_eye_of_cthulu, killed_eater_of_worlds, killed_skeletron, killed_queen_bee, killed_the_destroyer, killed_the_twins, killed_skeletron_prime, killed_any_hardmode_boss, killed_plantera, killed_golem, killed_slime_king, saved_goblin_tinkerer, saved_wizard, saved_mechanic, defeated_goblin_invasion, killed_clown, defeated_frost_legion, defeated_pirates, broken_shadow_orb, meteor_spawned, shadow_orbs_broken_mod3, altars_smashed, hard_mode, after_party_of_doom, goblin_invasion_delay, goblin_invasion_size, goblin_invasion_type, goblin_invasion_x, slime_rain_time, sundial_cooldown, is_raining, rain_time, max_rain, tier_1_ore_id, tier_2_ore_id, tier_3_ore_id, tree_style, corruption_style, jungle_style, snow_style, hallow_style, crimson_style, desert_style, ocean_style, cloud_bg, num_clouds, wind_speed, angler_finishers, saved_angler, angler_quest, saved_stylist, saved_tax_collector, saved_golfer, invasion_size_start, temp_cultist_delay, kill_counts, fast_forward_time, downed_fishron, downed_martians, downed_ancient_cultist, downed_moonlord, downed_halloween_king, downed_halloween_tree, downed_christmas_ice_queen, downed_christmas_santank, downed_christmas_tree, downed_tower_solar, downed_tower_vortex, downed_tower_nebula, downed_tower_stardust, tower_active_solar, tower_active_vortex, tower_active_nebula, tower_active_stardust, lunar_apocalypse_is_up, party_manual, party_genuine, party_cooldown, party_celebrating_npcs, sandstorm_happening, sandstorm_time_left, sandstorm_severity, sandstorm_intended_severity, saved_bartender, downed_invasion_tier_1, downed_invasion_tier_2, downed_invasion_tier_3, mushroom_bg, underworld_bg, tree2_bg, tree3_bg, tree4_bg, combat_book_was_used, lantern_night_stuff, lantern_night_more_stuff, tree_top_stuff, force_halloween_for_today, force_xmas_for_today, copper_tier, iron_tier, silver_tier, gold_tier, bought_cat, bought_dog, bought_bunny, downed_empress_of_light, downed_queen_slime, downed_deerclops, unlocked_slime_blue_spawn, unlocked_merchant_spawn, unlocked_demolitionist_spawn, unlocked_party_girl_spawn, unlocked_dye_trader_spawn, unlocked_truffle_spawn, unlocked_arms_dealer_spawn, unlocked_nurse_spawn, unlocked_princess_spawn, combat_book_v2_was_used, peddlers_satched_was_used, unlocked_slime_green_spawn, unlocked_slime_old_spawn, unlocked_slime_purple_spawn, unlocked_slime_rainbow_spawn, unlocked_slime_red_spawn, unlocked_slime_yellow_spawn, unlocked_slime_copper_spawn, fast_forward_to_dusk, moondial_cooldown,
		
		tile_set, tiles, chests, signs, npcs, tile_entities, weighted_pressure_plates, npc_rooms, bestiary_kills, bestiary_sights, bestiary_chats, creative_powers
	}
}