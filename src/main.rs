

use glium::{glutin::{dpi::{LogicalSize, PhysicalPosition, PhysicalSize}, event::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, ContextBuilder}, index::PrimitiveType, texture::{MipmapsOption, RawImage2d, SrgbFormat, SrgbTexture2d}, uniform, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerWrapFunction}, vertex::Attribute, Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};

extern crate hashbrown;
extern crate rand;
extern crate glium;
//use rand::Rng;

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



fn main() {
	
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { mut width, mut height } = display.gl_window().window().inner_size();
	
	let shader_program = Program::from_source(&display, DEFAULT_VERTEX_SHADER, include_str!("shader.frag"), None).unwrap();
	
	let default_vertex_buffer = VertexBuffer::new(&display, &DEFAULT_VERTICES).unwrap();
	let default_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &DEFAULT_INDICES).unwrap();
	
	
	
	
	
	
	
	let path = String::from("C:\\Users\\benap\\OneDrive\\Documents\\My Games\\Terraria\\Worlds\\");
	
	let w = Wld::read(&(path.clone() + "no.wld")).unwrap();
	
	let mut data: Box<[(u8, u8, u8)]> = vec![(0, 0, 0); (w.width * w.height) as usize].into_boxed_slice();
	for y in 0..w.height {
		for x in 0..w.width {
			data[(y * w.width + x) as usize] = if let Some(_) = w.tiles[(x * w.height + y) as usize].block {
				(255, 255, 255)
			} else if let Some(_) = w.tiles[(x * w.height + y) as usize].wall {
				(127, 127, 127)
			} else {
				(0, 0, 0)
			};
		}
	}
	
	let data = RawImage2d::from_raw_rgb_reversed(&unsafe {
		Vec::from_raw_parts(data.as_mut_ptr() as *mut u8, data.len() * 3, data.len() * 3)
	}, (w.width, w.height));
	
	let tex = SrgbTexture2d::with_format(&display, data, SrgbFormat::U8U8U8, MipmapsOption::NoMipmap).unwrap();
	
	
	
	let mut x = w.spawn_x as f32;
	let mut y = w.height as f32 - w.spawn_y as f32;
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
						y += dy * z / width as f32;
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
					y -= (previous_mouse_pos.y as f32 - 0.5 * height as f32) / width as f32 * z;
					z *= 1.0 - 0.1 * distance;
					x -= (previous_mouse_pos.x as f32 / width as f32 - 0.5) * z;
					y += (previous_mouse_pos.y as f32 - 0.5 * height as f32) / width as f32 * z;
					
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
				
				let sampler = tex.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Linear).wrap_function(SamplerWrapFunction::Repeat);
				
				target.draw(&default_vertex_buffer, &default_index_buffer, &shader_program, &uniform! {
					aspect_ratio: height as f32 / width as f32,
					camera_position: (x, y),
					zoom: z,
					wld_size: (w.width as f32, w.height as f32),
					wld_texture: sampler
				}, &DrawParameters::default()).unwrap();
				
				target.finish().unwrap();
				
			}
			_ => ()
		}
	});
}