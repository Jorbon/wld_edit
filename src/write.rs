use std::rc::Rc;

use crate::{wld::Wld, structs::{Block, Wall, Liquid, Slope, LiquidType, TileEntityInfo, CreativePower}};

struct Writer {
	pub data: Vec<u8>
}

impl Writer {
	pub fn mark(&mut self, i: usize) {
		let n = self.data.len() as u32;
		self.data[26 + 4*i] = n as u8;
		self.data[27 + 4*i] = (n >> 8) as u8;
		self.data[28 + 4*i] = (n >> 16) as u8;
		self.data[29 + 4*i] = (n >> 24) as u8;
	}
	
	pub fn u8(&mut self, n: u8) { self.data.push(n) }
	pub fn u16(&mut self, n: u16) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn u32(&mut self, n: u32) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn u64(&mut self, n: u64) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn u128(&mut self, n: u128) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn f32(&mut self, n: f32) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn f64(&mut self, n: f64) { self.data.append(&mut n.to_le_bytes().to_vec()) }
	pub fn bool(&mut self, n: bool) { self.data.push(match n { true => 1, false => 0 }) }
	pub fn string(&mut self, n: &String) {
		self.data.push(n.len() as u8);
		self.data.append(&mut n.as_bytes().to_vec());
	}
}

pub fn write(wld: &Wld) -> Vec<u8> {
	if wld.width > 8401 {
		println!("{}: Too wide to open in game, {} / 8400", wld.name, wld.width);
	}
	if wld.height > 2400 {
		println!("{}: Too tall to open in game, {} / 2400", wld.name, wld.height);
	}
	
	let mut w = Writer { data: vec![] };
	
	w.u32(wld.version);
	for n in wld.magic_number { w.u8(n) }
	w.u8(wld.file_type);
	w.u32(wld.revision);
	w.u64(wld.is_favorite);
	w.u16(11);
	for _ in 0..11 { w.u32(0) }
	w.u16(wld.importance.len() as u16);
	
	let mut i = 0;
	let mut n = 0;
	for b in &wld.importance {
		if *b {
			n += 1 << i;
		}
		i += 1;
		if i >= 8 {
			w.u8(n);
			n = 0;
			i = 0;
		}
	}
	if i != 0 {
		w.u8(n);
	}
	
	w.mark(0);
	
	
	println!("{}: Writing header", wld.name);
	
	w.string(&wld.name);
	w.string(&wld.seed);
	w.u64(wld.world_gen_version);
	w.u128(wld.guid);
	w.u32(wld.id);
	w.u32(wld.left);
	w.u32(wld.right);
	w.u32(wld.top);
	w.u32(wld.bottom);
	w.u32(wld.height);
	w.u32(wld.width);
	w.u32(wld.gamemode);
	w.bool(wld.drunk_world);
	w.bool(wld.good_world);
	w.bool(wld.tenth_anniversary_world);
	w.bool(wld.dont_starve_world);
	w.bool(wld.notthebees_world);
	w.bool(wld.remix_world);
	w.bool(wld.notraps_world);
	w.bool(wld.zenith_world);
	w.u64(wld.creation_time);
	w.u8(wld.moon_type);
	for n in wld.tree_type_xcoords { w.u32(n) }
	for n in wld.tree_types { w.u32(n) }
	for n in wld.cave_bg_xcoords { w.u32(n) }
	for n in wld.cave_bgs { w.u32(n) }
	w.u32(wld.jungle_bg);
	w.u32(wld.ice_bg);
	w.u32(wld.hell_bg);
	w.u32(wld.spawn_x);
	w.u32(wld.spawn_y);
	w.f64(wld.world_surface_y);
	w.f64(wld.rock_layer_y);
	w.f64(wld.game_time);
	w.bool(wld.is_day);
	w.u32(wld.moon_phase);
	w.bool(wld.blood_moon);
	w.bool(wld.eclipse);
	w.u32(wld.dungeon_x);
	w.u32(wld.dungeon_y);
	w.bool(wld.crimson_world);
	w.bool(wld.killed_eye_of_cthulu);
	w.bool(wld.killed_eater_of_worlds);
	w.bool(wld.killed_skeletron);
	w.bool(wld.killed_queen_bee);
	w.bool(wld.killed_the_destroyer);
	w.bool(wld.killed_the_twins);
	w.bool(wld.killed_skeletron_prime);
	w.bool(wld.killed_any_hardmode_boss);
	w.bool(wld.killed_plantera);
	w.bool(wld.killed_golem);
	w.bool(wld.killed_slime_king);
	w.bool(wld.saved_goblin_tinkerer);
	w.bool(wld.saved_wizard);
	w.bool(wld.saved_mechanic);
	w.bool(wld.defeated_goblin_invasion);
	w.bool(wld.killed_clown);
	w.bool(wld.defeated_frost_legion);
	w.bool(wld.defeated_pirates);
	w.bool(wld.broken_shadow_orb);
	w.bool(wld.meteor_spawned);
	w.u8(wld.shadow_orbs_broken_mod3);
	w.u32(wld.altars_smashed);
	w.bool(wld.hard_mode);
	w.bool(wld.after_party_of_doom);
	w.u32(wld.goblin_invasion_delay);
	w.u32(wld.goblin_invasion_size);
	w.u32(wld.goblin_invasion_type);
	w.f64(wld.goblin_invasion_x);
	w.f64(wld.slime_rain_time);
	w.u8(wld.sundial_cooldown);
	w.bool(wld.is_raining);
	w.u32(wld.rain_time);
	w.f32(wld.max_rain);
	w.u32(wld.tier_1_ore_id);
	w.u32(wld.tier_2_ore_id);
	w.u32(wld.tier_3_ore_id);
	w.u8(wld.tree_style);
	w.u8(wld.corruption_style);
	w.u8(wld.jungle_style);
	w.u8(wld.snow_style);
	w.u8(wld.hallow_style);
	w.u8(wld.crimson_style);
	w.u8(wld.desert_style);
	w.u8(wld.ocean_style);
	w.u32(wld.cloud_bg);
	w.u16(wld.num_clouds);
	w.f32(wld.wind_speed);
	w.u32(wld.angler_finishers.len() as u32);
	for s in &wld.angler_finishers { w.string(&s) }
	w.bool(wld.saved_angler);
	w.u32(wld.angler_quest);
	w.bool(wld.saved_stylist);
	w.bool(wld.saved_tax_collector);
	w.bool(wld.saved_golfer);
	w.u32(wld.invasion_size_start);
	w.u32(wld.temp_cultist_delay);
	w.u16(wld.kill_counts.len() as u16);
	for n in &wld.kill_counts { w.u32(*n) }
	w.bool(wld.fast_forward_time);
	w.bool(wld.downed_fishron);
	w.bool(wld.downed_martians);
	w.bool(wld.downed_ancient_cultist);
	w.bool(wld.downed_moonlord);
	w.bool(wld.downed_halloween_king);
	w.bool(wld.downed_halloween_tree);
	w.bool(wld.downed_christmas_ice_queen);
	w.bool(wld.downed_christmas_santank);
	w.bool(wld.downed_christmas_tree);
	w.bool(wld.downed_tower_solar);
	w.bool(wld.downed_tower_vortex);
	w.bool(wld.downed_tower_nebula);
	w.bool(wld.downed_tower_stardust);
	w.bool(wld.tower_active_solar);
	w.bool(wld.tower_active_vortex);
	w.bool(wld.tower_active_nebula);
	w.bool(wld.tower_active_stardust);
	w.bool(wld.lunar_apocalypse_is_up);
	w.bool(wld.party_manual);
	w.bool(wld.party_genuine);
	w.u32(wld.party_cooldown);
	w.u32(wld.party_celebrating_npcs.len() as u32);
	for n in &wld.party_celebrating_npcs { w.u32(*n) }
	w.bool(wld.sandstorm_happening);
	w.u32(wld.sandstorm_time_left);
	w.f32(wld.sandstorm_severity);
	w.f32(wld.sandstorm_intended_severity);
	w.bool(wld.saved_bartender);
	w.bool(wld.downed_invasion_tier_1);
	w.bool(wld.downed_invasion_tier_2);
	w.bool(wld.downed_invasion_tier_3);
	w.u8(wld.mushroom_bg);
	w.u8(wld.underworld_bg);
	w.u8(wld.tree2_bg);
	w.u8(wld.tree3_bg);
	w.u8(wld.tree4_bg);
	w.bool(wld.combat_book_was_used);
	w.u32(wld.lantern_night_stuff);
	for b in wld.lantern_night_more_stuff { w.bool(b) }
	w.u32(wld.tree_top_stuff.len() as u32);
	for n in &wld.tree_top_stuff { w.u32(*n) }
	w.bool(wld.force_halloween_for_today);
	w.bool(wld.force_xmas_for_today);
	w.u32(wld.copper_tier);
	w.u32(wld.iron_tier);
	w.u32(wld.silver_tier);
	w.u32(wld.gold_tier);
	w.bool(wld.bought_cat);
	w.bool(wld.bought_dog);
	w.bool(wld.bought_bunny);
	w.bool(wld.downed_empress_of_light);
	w.bool(wld.downed_queen_slime);
	w.bool(wld.downed_deerclops);
	w.bool(wld.unlocked_slime_blue_spawn);
	w.bool(wld.unlocked_merchant_spawn);
	w.bool(wld.unlocked_demolitionist_spawn);
	w.bool(wld.unlocked_party_girl_spawn);
	w.bool(wld.unlocked_dye_trader_spawn);
	w.bool(wld.unlocked_truffle_spawn);
	w.bool(wld.unlocked_arms_dealer_spawn);
	w.bool(wld.unlocked_nurse_spawn);
	w.bool(wld.unlocked_princess_spawn);
	w.bool(wld.combat_book_v2_was_used);
	w.bool(wld.peddlers_satched_was_used);
	w.bool(wld.unlocked_slime_green_spawn);
	w.bool(wld.unlocked_slime_old_spawn);
	w.bool(wld.unlocked_slime_purple_spawn);
	w.bool(wld.unlocked_slime_rainbow_spawn);
	w.bool(wld.unlocked_slime_red_spawn);
	w.bool(wld.unlocked_slime_yellow_spawn);
	w.bool(wld.unlocked_slime_copper_spawn);
	w.bool(wld.fast_forward_to_dusk);
	w.u8(wld.moondial_cooldown);
	
	w.mark(1);
	
	
	println!("{}: Writing tiles", wld.name);
	
	let mut i = 0;
	while (i as u32) < wld.width * wld.height {
		let tile = Rc::clone(&wld.tiles[i]);
		let mut k = 0;
		loop {
			i += 1;
			if i as u32 % wld.height == 0 { break }
			if wld.tiles[i] != tile { break }
			k += 1;
		}
		
		let mut a = 0;
		let mut b = 0;
		let mut c = 0;
		
		
		
		let mut temp = Vec::new();
		
		if let Some(Block { id, color, uv, inactive, slope }) = tile.block {
			a += 2;
			temp.push(id as u8);
			if id >= 256 {
				a += 32;
				temp.push((id >> 8) as u8);
			}
			if wld.importance[id as usize] {
				let uv = uv.unwrap_or((0, 0));
				temp.append(&mut vec![uv.0 as u8, (uv.0 >> 8) as u8, uv.1 as u8, (uv.1 >> 8) as u8]);
			}
			if let Some(n) = color {
				c += 8;
				temp.push(n);
			}
			if inactive {
				c += 4;
			}
			b += match slope {
				Slope::Full => 0,
				Slope::Half => 1,
				Slope::LowerLeft => 2,
				Slope::LowerRight => 3,
				Slope::UpperLeft => 4,
				Slope::UpperRight => 5
			} << 4;
		}
		
		if let Some(Wall { id, color }) = tile.wall {
			a += 4;
			temp.push(id as u8);
			if id >= 256 {
				c += 64;
				temp.push((id >> 8) as u8);
			}
			if let Some(n) = color {
				c += 16;
				temp.push(n);
			}
		}
		
		if let Some(Liquid { kind, amount }) = tile.liquid {
			a += match kind {
				LiquidType::Water => 1,
				LiquidType::Lava => 2,
				LiquidType::Honey => 3,
				LiquidType::Shimmer => { c += 128; 1 }
			} << 3;
			temp.push(amount);
		}
		
		if tile.red_wire { b += 2 }
		if tile.green_wire { b += 4 }
		if tile.blue_wire { b += 8 }
		if tile.yellow_wire { c += 32 }
		if tile.actuator { c += 2 }
		
		if k > 0 {
			a += 64;
			temp.push(k as u8);
			if k >= 256 {
				a += 64;
				temp.push((k >> 8) as u8);
			}
		}
		
		if c > 0 {
			temp.insert(0, c);
			b += 1;
		}
		if b > 0 {
			temp.insert(0, b);
			a += 1;
		}
		
		w.u8(a);
		w.data.append(&mut temp);
	}
	
	w.mark(2);
	
	
	println!("{}: Writing chests", wld.name);
	
	w.u16(wld.chests.len() as u16);
	w.u16(40);
	for chest in &wld.chests {
		w.u32(chest.x);
		w.u32(chest.y);
		w.string(&chest.name);
		for item in &chest.items {
			match item {
				Some(n) => { w.u16(n.count); w.u8(n.prefix); w.u32(n.id) },
				None => w.u16(0)
			}
		}
	}
	
	w.mark(3);
	
	
	println!("{}: Writing signs", wld.name);
	
	w.u16(wld.signs.len() as u16);
	for sign in &wld.signs {
		w.string(&sign.text);
		w.u32(sign.x);
		w.u32(sign.y);
	}
	
	w.mark(4);
	
	
	println!("{}: Writing npcs", wld.name);
	
	let mut temp = vec![];
	let mut n = 0;
	for npc in &wld.npcs {
		if npc.shimmered {
			temp.push(npc.id);
			n += 1;
		}
	}
	w.u32(n);
	temp.iter().map(|n| w.u32(*n)).count();
	
	for npc in &wld.npcs {
		if npc.is_pillar { continue }
		w.bool(true);
		w.u32(npc.id);
		w.string(&npc.name);
		w.f32(npc.x);
		w.f32(npc.y);
		w.bool(npc.homeless);
		w.u32(npc.home_x);
		w.u32(npc.home_y);
		match npc.variation_index {
			0 => w.bool(false),
			_ => { w.bool(true); w.u32(npc.variation_index) }
		};
	}
	w.bool(false);
	for npc in &wld.npcs {
		if !npc.is_pillar { continue }
		w.bool(true);
		w.u32(npc.id);
		w.f32(npc.x);
		w.f32(npc.y);
	}
	w.bool(false);
	
	w.mark(5);
	
	
	println!("{}: Writing misc", wld.name);
	
	w.u32(wld.tile_entities.len() as u32);
	for t in &wld.tile_entities {
		w.u8(match t.info {
			TileEntityInfo::TargetDummy(..) => 0,
			TileEntityInfo::ItemFrame(..) => 1,
			TileEntityInfo::LogicSensor(..) => 2,
			TileEntityInfo::Mannequin(..) => 3,
			TileEntityInfo::WeaponRack(..) => 4,
			TileEntityInfo::HatRack(..) => 5,
			TileEntityInfo::FoodPlatter(..) => 6,
			TileEntityInfo::Pylon => 7
		});
		w.u32(t.id);
		w.u16(t.x);
		w.u16(t.y);
		match t.info {
			TileEntityInfo::Pylon => {}
			TileEntityInfo::TargetDummy(n) => w.u16(n),
			TileEntityInfo::LogicSensor(n, b) => { w.u8(n); w.bool(b) }
			TileEntityInfo::ItemFrame(None) => { w.u32(0); w.u8(0) }
			TileEntityInfo::ItemFrame(Some(item)) => { w.u16(item.id as u16); w.u8(item.prefix); w.u16(item.count) }
			TileEntityInfo::WeaponRack(None) => { w.u32(0); w.u8(0) }
			TileEntityInfo::WeaponRack(Some(item)) => { w.u16(item.id as u16); w.u8(item.prefix); w.u16(item.count) }
			TileEntityInfo::FoodPlatter(None) => { w.u32(0); w.u8(0) }
			TileEntityInfo::FoodPlatter(Some(item)) => { w.u16(item.id as u16); w.u8(item.prefix); w.u16(item.count) }
			TileEntityInfo::Mannequin(items) => {
				w.u16((0..16).map(|i| match items[i] { Some(_) => 1 << i, None => 0 }).sum());
				for item in items {
					if let Some(item) = item {
						w.u16(item.id as u16);
						w.u8(item.prefix);
						w.u16(item.count);
					}
				}
			}
			TileEntityInfo::HatRack(items) => {
				w.u8((0..4).map(|i| match items[i] { Some(_) => 1 << i, None => 0 }).sum());
				for item in items {
					if let Some(item) = item {
						w.u16(item.id as u16);
						w.u8(item.prefix);
						w.u16(item.count);
					}
				}
			}
		};
	}
	
	w.mark(6);
	
	w.u32(wld.weighted_pressure_plates.len() as u32);
	for p in &wld.weighted_pressure_plates {
		w.u32(p.0);
		w.u32(p.1);
	}
	
	w.mark(7);
	
	w.u32(wld.npc_rooms.len() as u32);
	for r in &wld.npc_rooms {
		w.u32(r.id);
		w.u32(r.x);
		w.u32(r.y);
	}
	
	w.mark(8);
	
	w.u32(wld.bestiary_kills.len() as u32);
	for k in &wld.bestiary_kills {
		w.string(&k.0);
		w.u32(k.1);
	}
	w.u32(wld.bestiary_sights.len() as u32);
	for s in &wld.bestiary_sights {
		w.string(&s);
	}
	w.u32(wld.bestiary_chats.len() as u32);
	for s in &wld.bestiary_chats {
		w.string(&s);
	}
	
	w.mark(9);
	
	for p in &wld.creative_powers {
		w.bool(true);
		w.u16(match p {
			CreativePower::FreezeTime(_) => 0,
			CreativePower::TimeRate(_) => 8,
			CreativePower::FreezeWeather(_) => 9,
			CreativePower::FreezeWind(_) => 10,
			CreativePower::DifficultySlider(_) => 12,
			CreativePower::FreezeSpread(_) => 13
		});
		match p {
			CreativePower::FreezeTime(b) => w.bool(*b),
			CreativePower::FreezeWeather(b) => w.bool(*b),
			CreativePower::FreezeWind(b) => w.bool(*b),
			CreativePower::FreezeSpread(b) => w.bool(*b),
			CreativePower::TimeRate(n) => w.f32(*n),
			CreativePower::DifficultySlider(n) => w.f32(*n)
		}
	}
	w.bool(false);
	
	w.mark(10);
	
	w.bool(true);
	w.string(&wld.name);
	w.u32(wld.id);
	
	println!("{}: Done", wld.name);
	
	w.data
}