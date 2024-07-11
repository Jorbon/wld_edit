

use std::rc::Rc;

use glium::{glutin::{dpi::{LogicalSize, PhysicalPosition, PhysicalSize}, event::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, ContextBuilder}, index::PrimitiveType, texture::{MipmapsOption, RawImage2d, SrgbTexture2d, UncompressedUintFormat, UnsignedTexture2d}, uniform, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction}, vertex::Attribute, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};

extern crate hashbrown;
extern crate rand;
extern crate glium;
//use rand::Rng;

use hashbrown::HashMap;
use structs::{LiquidType, Slope, Tile};
use wld::Wld;

mod wld;
mod structs;
mod read;
mod write;


#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2(pub f32, pub f32);
impl glium::vertex::Vertex for Vec2 {
	fn build_bindings() -> glium::VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <(f32, f32)>::get_type(), false)])
	}
}

static DEFAULT_VERTEX_SHADER: &str = "#version 150
in vec2 position;
out vec2 screen_position;
void main() {
	screen_position = (position + 1.0) * 0.5;
	gl_Position = vec4(position, 0.0, 1.0);
}";

static DEFAULT_VERTICES: [Vec2; 4] = [Vec2(-1.0, -1.0), Vec2(1.0, -1.0), Vec2(1.0, 1.0), Vec2(-1.0, 1.0)];
static DEFAULT_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];


fn load_texture(display: &Display, path: &str) -> SrgbTexture2d {
	let mut path_buf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
	path_buf.push("textures");
	path_buf.push(path);
	match std::fs::File::open(&path_buf) {
		Ok(file) => {
			let img_buffer = image::load(std::io::BufReader::new(file), image::ImageFormat::Png).unwrap().to_rgba8();
			let dimensions = img_buffer.dimensions();
			let img = RawImage2d::from_raw_rgba(img_buffer.into_raw(), dimensions);
			SrgbTexture2d::new(display, img).unwrap()
		}
		Err(e) => panic!("{} - Path: {}", e, path_buf.display())
	}
}


impl Tile {
	pub fn encode(&self) -> (u32, u32, u32) {
		let (block_exists, block_id, block_color, (u, v), inactive, slope) = match self.block {
			Some(block) => (
				1,
				block.id as u32,
				block.color.unwrap_or(0) as u32,
				block.uv.unwrap_or((0, 0)),
				u32::from(block.inactive),
				match block.slope {
					Slope::Full => 0,
					Slope::Half => 1,
					Slope::LowerLeft => 2,
					Slope::LowerRight => 3,
					Slope::UpperLeft => 4,
					Slope::UpperRight => 5
				}
			),
			None => (0, 0, 0, (0, 0), 0, 0)
		};
		
		let (wall_exists, wall_id, wall_color) = match self.wall {
			Some(wall) => (1, wall.id as u32, wall.color.unwrap_or(0) as u32),
			None => (0, 0, 0)
		};
		
		let (liquid_id, liquid_amount) = match self.liquid {
			Some(liquid) => (match liquid.kind {
				LiquidType::Water => 1,
				LiquidType::Lava => 2,
				LiquidType::Honey => 3,
				LiquidType::Shimmer => 4
			}, liquid.amount as u32),
			None => (0, 0)
		};
		
		(
			block_exists
		  | wall_exists << 1
		  | liquid_id << 2
		  | slope << 5
		  | inactive << 8
		  | u32::from(self.red_wire) << 9
		  | u32::from(self.green_wire) << 10
		  | u32::from(self.blue_wire) << 11
		  | u32::from(self.yellow_wire) << 12
		  | u32::from(self.actuator) << 13
		  | (block_color & 0b11111) << 14
		  | (wall_color & 0b11111) << 19
		  | liquid_amount << 24,
			
			block_id | wall_id << 16,
			(u as u32 / 2) | (v as u32 / 2) << 16
		)
	}
}




fn main() {
	
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { mut width, mut height } = display.gl_window().window().inner_size();
	
	let shader_program = Program::from_source(&display, DEFAULT_VERTEX_SHADER, include_str!("shader.frag"), None).unwrap();
	
	let default_vertex_buffer = VertexBuffer::new(&display, &DEFAULT_VERTICES).unwrap();
	let default_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &DEFAULT_INDICES).unwrap();
	
	let block_texture = load_texture(&display, &format!("tiles/21.png"));
	
	
	let w = Wld::read("C:\\Users\\benap\\OneDrive\\Documents\\My Games\\Terraria\\Worlds\\no.wld").unwrap();
	
	
	let mut encoded_tile_set: HashMap<Rc<Tile>, (u32, u32, u32)> = HashMap::new();
	for tile in &w.tile_set {
		encoded_tile_set.insert(Rc::clone(&tile), tile.encode());
	}
	
	let mut encoded_tile_data = vec![];
	encoded_tile_data.reserve((w.width * w.height * 3) as usize);
	for y in 0..w.height {
		for x in 0..w.width {
			let encoded = encoded_tile_set.get(&w.tiles[(x * w.height + y) as usize]).unwrap();
			encoded_tile_data.push(encoded.0);
			encoded_tile_data.push(encoded.1);
			encoded_tile_data.push(encoded.2);
		}
	}
	
	let data = RawImage2d::from_raw_rgb(encoded_tile_data, (w.width, w.height));
	let tile_data_buffer = UnsignedTexture2d::with_format(&display, data, UncompressedUintFormat::U32U32U32, MipmapsOption::NoMipmap).unwrap();
	
	
	
	
	let mut x = w.spawn_x as f32;
	let mut y = w.spawn_y as f32;
	let mut z = 50f32;
	
	let mut previous_mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);
	let mut mouse_down = false;
	
	
	
	event_loop.run(move |ev, _, control_flow| {
		match ev {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(code) = input.virtual_keycode {
						let state = match input.state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
						match code {
							VirtualKeyCode::Up => if state { z /= 1.25 }
							VirtualKeyCode::Down => if state { z *= 1.25 }
							VirtualKeyCode::W => if state { y += z * 0.05 }
							VirtualKeyCode::S => if state { y -= z * 0.05 }
							VirtualKeyCode::A => if state { x -= z * 0.05 }
							VirtualKeyCode::D => if state { x += z * 0.05 }
							
							_ => ()
						}
						
						if state {
							display.gl_window().window().request_redraw();
						}
						
					}
				}
				WindowEvent::MouseInput { state, button, .. } => {
					if let MouseButton::Left = button {
						mouse_down = match state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
					}
				}
				WindowEvent::CursorMoved { position, .. } => {
					if mouse_down {
						let dx = (position.x - previous_mouse_pos.x) as f32;
						let dy = (position.y - previous_mouse_pos.y) as f32;
						x -= dx * z / width as f32;
						y -= dy * z / width as f32;
						display.gl_window().window().request_redraw();
					}
					previous_mouse_pos = position;
				}
				WindowEvent::MouseWheel { delta, .. } => {
					let distance = match delta {
						MouseScrollDelta::LineDelta(_, dy) => dy as f32,
						MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y as f32 / 20.0
					};
					
					x += (previous_mouse_pos.x as f32 / width as f32 - 0.5) * z;
					y += (previous_mouse_pos.y as f32 - 0.5 * height as f32) / width as f32 * z;
					z *= 1.0 - 0.1 * distance;
					x -= (previous_mouse_pos.x as f32 / width as f32 - 0.5) * z;
					y -= (previous_mouse_pos.y as f32 - 0.5 * height as f32) / width as f32 * z;
					
					display.gl_window().window().request_redraw();
				}
				WindowEvent::Resized(new_size) => {
					width = new_size.width;
					height = new_size.height;
					//main_texture = SrgbTexture2d::empty(&display, width, height).unwrap();
				}
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				_ => ()
			}
			Event::RedrawEventsCleared => {
				//display.gl_window().window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				
				let mut target = display.draw();
				target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
				
				let uniforms = uniform! {
					aspect_ratio: height as f32 / width as f32,
					camera_position: (x, y),
					zoom: z,
					wld_size: (w.width as f32, w.height as f32),
					tile_data_buffer: tile_data_buffer.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Linear).wrap_function(SamplerWrapFunction::Repeat),
					block_texture: block_texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Linear).wrap_function(SamplerWrapFunction::Repeat)
				};
				
				target.draw(&default_vertex_buffer, &default_index_buffer, &shader_program, &uniforms, &DrawParameters::default()).unwrap();
				
				target.finish().unwrap();
				
			}
			_ => ()
		}
	});
}