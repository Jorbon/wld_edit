
use std::{fs::File, io::{BufReader, Read, Write}, rc::Rc};
use hashbrown::hash_set::HashSet;
use crate::structs::{Chest, Sign, Tile, NPC, TileEntity, NPCRoom, CreativePower};

pub struct Wld {
	pub version: u32,
	pub magic_number: [u8; 7],
	pub file_type: u8,
	pub revision: u32,
	pub is_favorite: u64,
	pub importance: Vec<bool>,
	
	pub name: String,
	pub seed: String,
	pub world_gen_version: u64,
	pub guid: u128,
	pub id: u32,
	pub left: u32,
	pub right: u32,
	pub top: u32,
	pub bottom: u32,
	pub height: u32,
	pub width: u32,
	pub gamemode: u32,
	pub drunk_world: bool,
	pub good_world: bool,
	pub tenth_anniversary_world: bool,
	pub dont_starve_world: bool,
	pub notthebees_world: bool,
	pub remix_world: bool,
	pub notraps_world: bool,
	pub zenith_world: bool,
	pub creation_time: u64,
	pub moon_type: u8,
	pub tree_type_xcoords: [u32; 3],
	pub tree_types: [u32; 4],
	pub cave_bg_xcoords: [u32; 3],
	pub cave_bgs: [u32; 4],
	pub ice_bg: u32,
	pub jungle_bg: u32,
	pub hell_bg: u32,
	pub spawn_x: u32,
	pub spawn_y: u32,
	pub world_surface_y: f64,
	pub rock_layer_y: f64,
	pub game_time: f64,
	pub is_day: bool,
	pub moon_phase: u32,
	pub blood_moon: bool,
	pub eclipse: bool,
	pub dungeon_x: u32,
	pub dungeon_y: u32,
	pub crimson_world: bool,
	pub killed_eye_of_cthulu: bool,
	pub killed_eater_of_worlds: bool,
	pub killed_skeletron: bool,
	pub killed_queen_bee: bool,
	pub killed_the_destroyer: bool,
	pub killed_the_twins: bool,
	pub killed_skeletron_prime: bool,
	pub killed_any_hardmode_boss: bool,
	pub killed_plantera: bool,
	pub killed_golem: bool,
	pub killed_slime_king: bool,
	pub saved_goblin_tinkerer: bool,
	pub saved_wizard: bool,
	pub saved_mechanic: bool,
	pub defeated_goblin_invasion: bool,
	pub killed_clown: bool,
	pub defeated_frost_legion: bool,
	pub defeated_pirates: bool,
	pub broken_shadow_orb: bool,
	pub meteor_spawned: bool,
	pub shadow_orbs_broken_mod3: u8,
	pub altars_smashed: u32,
	pub hard_mode: bool,
	pub after_party_of_doom: bool,
	pub goblin_invasion_delay: u32,
	pub goblin_invasion_size: u32,
	pub goblin_invasion_type: u32,
	pub goblin_invasion_x: f64,
	pub slime_rain_time: f64,
	pub sundial_cooldown: u8,
	pub is_raining: bool,
	pub rain_time: u32,
	pub max_rain: f32,
	pub tier_1_ore_id: u32,
	pub tier_2_ore_id: u32,
	pub tier_3_ore_id: u32,
	pub tree_style: u8,
	pub corruption_style: u8,
	pub jungle_style: u8,
	pub snow_style: u8,
	pub hallow_style: u8,
	pub crimson_style: u8,
	pub desert_style: u8,
	pub ocean_style: u8,
	pub cloud_bg: u32,
	pub num_clouds: u16,
	pub wind_speed: f32,
	pub angler_finishers: Vec<String>,
	pub saved_angler: bool,
	pub angler_quest: u32,
	pub saved_stylist: bool,
	pub saved_tax_collector: bool,
	pub saved_golfer: bool,
	pub invasion_size_start: u32,
	pub temp_cultist_delay: u32,
	pub kill_counts: Vec<u32>,
	pub fast_forward_time: bool,
	pub downed_fishron: bool,
	pub downed_martians: bool,
	pub downed_ancient_cultist: bool,
	pub downed_moonlord: bool,
	pub downed_halloween_king: bool,
	pub downed_halloween_tree: bool,
	pub downed_christmas_ice_queen: bool,
	pub downed_christmas_santank: bool,
	pub downed_christmas_tree: bool,
	pub downed_tower_solar: bool,
	pub downed_tower_vortex: bool,
	pub downed_tower_nebula: bool,
	pub downed_tower_stardust: bool,
	pub tower_active_solar: bool,
	pub tower_active_vortex: bool,
	pub tower_active_nebula: bool,
	pub tower_active_stardust: bool,
	pub lunar_apocalypse_is_up: bool,
	pub party_manual: bool,
	pub party_genuine: bool,
	pub party_cooldown: u32,
	pub party_celebrating_npcs: Vec<u32>,
	pub sandstorm_happening: bool,
	pub sandstorm_time_left: u32,
	pub sandstorm_severity: f32,
	pub sandstorm_intended_severity: f32,
	pub saved_bartender: bool,
	pub downed_invasion_tier_1: bool,
	pub downed_invasion_tier_2: bool,
	pub downed_invasion_tier_3: bool,
	pub mushroom_bg: u8,
	pub underworld_bg: u8,
	pub tree2_bg: u8,
	pub tree3_bg: u8,
	pub tree4_bg: u8,
	pub combat_book_was_used: bool,
	pub lantern_night_stuff: u32,
	pub lantern_night_more_stuff: [bool; 3],
	pub tree_top_stuff: Vec<u32>,
	pub force_halloween_for_today: bool,
	pub force_xmas_for_today: bool,
	pub copper_tier: u32,
	pub iron_tier: u32,
	pub silver_tier: u32,
	pub gold_tier: u32,
	pub bought_cat: bool,
	pub bought_dog: bool,
	pub bought_bunny: bool,
	pub downed_empress_of_light: bool,
	pub downed_queen_slime: bool,
	pub downed_deerclops: bool,
	pub unlocked_slime_blue_spawn: bool,
	pub unlocked_merchant_spawn: bool,
	pub unlocked_demolitionist_spawn: bool,
	pub unlocked_party_girl_spawn: bool,
	pub unlocked_dye_trader_spawn: bool,
	pub unlocked_truffle_spawn: bool,
	pub unlocked_arms_dealer_spawn: bool,
	pub unlocked_nurse_spawn: bool,
	pub unlocked_princess_spawn: bool,
	pub combat_book_v2_was_used: bool,
	pub peddlers_satched_was_used: bool,
	pub unlocked_slime_green_spawn: bool,
	pub unlocked_slime_old_spawn: bool,
	pub unlocked_slime_purple_spawn: bool,
	pub unlocked_slime_rainbow_spawn: bool,
	pub unlocked_slime_red_spawn: bool,
	pub unlocked_slime_yellow_spawn: bool,
	pub unlocked_slime_copper_spawn: bool,
	pub fast_forward_to_dusk: bool,
	pub moondial_cooldown: u8,
	
	pub tile_set: HashSet<Rc<Tile>>,
	pub tiles: Vec<Rc<Tile>>,
	
	pub chests: Vec<Chest>,
	pub signs: Vec<Sign>,
	pub npcs: Vec<NPC>,
	pub tile_entities: Vec<TileEntity>,
	pub weighted_pressure_plates: Vec<(u32, u32)>,
	pub npc_rooms: Vec<NPCRoom>,
	
	pub bestiary_kills: Vec<(String, u32)>,
	pub bestiary_sights: Vec<String>,
	pub bestiary_chats: Vec<String>,
	
	pub creative_powers: Vec<CreativePower>,
}

#[allow(dead_code)]
impl Wld {
	pub fn _new() -> Wld {
		let mut tile_set = HashSet::new();
		let p = tile_set.get_or_insert(Rc::new(Tile { block: None, wall: None, liquid: None, red_wire: false, green_wire: false, blue_wire: false, yellow_wire: false, actuator: false }));
		let tiles = vec![Rc::clone(p)];
		Wld { version: 279, magic_number: [0; 7], file_type: 2, revision: 1, is_favorite: 0, importance: vec![], name: String::new(), seed: String::new(), world_gen_version: 279, guid: 0, id: 0, left: 0, right: 1600, top: 0, bottom: 1600, height: 100, width: 100, gamemode: 0, drunk_world: false, good_world: false, tenth_anniversary_world: false, dont_starve_world: false, notthebees_world: false, remix_world: false, notraps_world: false, zenith_world: false, creation_time: 0, moon_type: 0, tree_type_xcoords: [0; 3], tree_types: [0; 4], cave_bg_xcoords: [0; 3], cave_bgs: [0; 4], ice_bg: 0, jungle_bg: 0, hell_bg: 0, spawn_x: 0, spawn_y: 0, world_surface_y: 0.0, rock_layer_y: 0.0, game_time: 0.0, is_day: true, moon_phase: 0, blood_moon: false, eclipse: false, dungeon_x: 0, dungeon_y: 0, crimson_world: false, killed_eye_of_cthulu: false, killed_eater_of_worlds: false, killed_skeletron: false, killed_queen_bee: false, killed_the_destroyer: false, killed_the_twins: false, killed_skeletron_prime: false, killed_any_hardmode_boss: false, killed_plantera: false, killed_golem: false, killed_slime_king: false, saved_goblin_tinkerer: false, saved_wizard: false, saved_mechanic: false, defeated_goblin_invasion: false, killed_clown: false, defeated_frost_legion: false, defeated_pirates: false, broken_shadow_orb: false, meteor_spawned: false, shadow_orbs_broken_mod3: 0, altars_smashed: 0, hard_mode: false, after_party_of_doom: false, goblin_invasion_delay: 0, goblin_invasion_size: 0, goblin_invasion_type: 0, goblin_invasion_x: 0.0, slime_rain_time: 0.0, sundial_cooldown: 0, is_raining: false, rain_time: 0, max_rain: 0.0, tier_1_ore_id: 0, tier_2_ore_id: 0, tier_3_ore_id: 0, tree_style: 0, corruption_style: 0, jungle_style: 0, snow_style: 0, hallow_style: 0, crimson_style: 0, desert_style: 0, ocean_style: 0, cloud_bg: 0, num_clouds: 0, wind_speed: 0.0, angler_finishers: vec![], saved_angler: false, angler_quest: 0, saved_stylist: false, saved_tax_collector: false, saved_golfer: false, invasion_size_start: 0, temp_cultist_delay: 0, kill_counts: vec![], fast_forward_time: false, downed_fishron: false, downed_martians: false, downed_ancient_cultist: false, downed_moonlord: false, downed_halloween_king: false, downed_halloween_tree: false, downed_christmas_ice_queen: false, downed_christmas_santank: false, downed_christmas_tree: false, downed_tower_solar: false, downed_tower_vortex: false, downed_tower_nebula: false, downed_tower_stardust: false, tower_active_solar: false, tower_active_vortex: false, tower_active_nebula: false, tower_active_stardust: false, lunar_apocalypse_is_up: false, party_manual: false, party_genuine: false, party_cooldown: 0, party_celebrating_npcs: vec![], sandstorm_happening: false, sandstorm_time_left: 0, sandstorm_severity: 0.0, sandstorm_intended_severity: 0.0, saved_bartender: false, downed_invasion_tier_1: false, downed_invasion_tier_2: false, downed_invasion_tier_3: false, mushroom_bg: 0, underworld_bg: 0, tree2_bg: 0, tree3_bg: 0, tree4_bg: 0, combat_book_was_used: false, lantern_night_stuff: 0, lantern_night_more_stuff: [false; 3], tree_top_stuff: vec![], force_halloween_for_today: false, force_xmas_for_today: false, copper_tier: 0, iron_tier: 0, silver_tier: 0, gold_tier: 0, bought_cat: false, bought_dog: false, bought_bunny: false, downed_empress_of_light: false, downed_queen_slime: false, downed_deerclops: false, unlocked_slime_blue_spawn: false, unlocked_merchant_spawn: false, unlocked_demolitionist_spawn: false, unlocked_party_girl_spawn: false, unlocked_dye_trader_spawn: false, unlocked_truffle_spawn: false, unlocked_arms_dealer_spawn: false, unlocked_nurse_spawn: false, unlocked_princess_spawn: false, combat_book_v2_was_used: false, peddlers_satched_was_used: false, unlocked_slime_green_spawn: false, unlocked_slime_old_spawn: false, unlocked_slime_purple_spawn: false, unlocked_slime_rainbow_spawn: false, unlocked_slime_red_spawn: false, unlocked_slime_yellow_spawn: false, unlocked_slime_copper_spawn: false, fast_forward_to_dusk: false, moondial_cooldown: 0,
		
		tile_set, tiles, chests: vec![], signs: vec![], npcs: vec![], tile_entities: vec![], weighted_pressure_plates: vec![], npc_rooms: vec![], bestiary_kills: vec![], bestiary_sights: vec![], bestiary_chats: vec![], creative_powers: vec![] }
	}
	
	pub fn read(path: &str) -> std::io::Result<Wld> {
		let file = File::open(path)?;
		let mut buf_reader = BufReader::new(file);
		
		let mut buffer = vec![];
		buf_reader.read_to_end(&mut buffer)?;
		
		Ok(crate::read::read(buffer))
	}
	pub fn write(&self, path: &str) -> std::io::Result<()> {
		let mut file = File::create(path)?;
		file.write_all(&crate::write::write(self))?;
		Ok(())
	}
	
	pub fn set_block(&mut self, x: u32, y: u32, tile: Tile) {
		let p = self.tile_set.get_or_insert(Rc::new(tile));
		self.tiles[(x*self.height + y) as usize] = Rc::clone(p);
	}
}



