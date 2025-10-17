// stuff in standard crate
use std::time::Duration;
use std::env::current_exe;
// sdl2
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::render::{WindowCanvas, Texture, Canvas};
use sdl2::rect::{Point, Rect};
use sdl2::image::LoadTexture;
use sdl2::mouse::MouseState;
use sdl2::video::Window;
use sdl2::surface::Surface;

// rendering functions
fn render(canvas: &mut WindowCanvas, colour: Color, texture: &Vec<Texture>, position: Vec<Point>, sprite: Vec<Rect>) -> Result<(), String> {
    canvas.set_draw_color(colour);
    let (width, height) = canvas.output_size()?;
    for i in 0..texture.len() {
        let screen_position = position[i] + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite[i].width(), sprite[i].height());
        canvas.copy(&texture[i], sprite[i], screen_rect)?;
    }
    canvas.present();
    Ok(())
}

// mouse functions
fn mouse_x(canvas: &mut WindowCanvas, e: &sdl2::EventPump) -> i32 {
    let (width, _height) = canvas.output_size().unwrap();
    let mouse_state: MouseState = e.mouse_state();
    let x = mouse_state.x();
    return x - width as i32 / 2;
}
fn mouse_x_abs(e: &sdl2::EventPump) -> i32 {
    let mouse_state: MouseState = e.mouse_state();
    let x = mouse_state.x();
    return x;
}
fn mouse_y(canvas: &mut WindowCanvas, e: &sdl2::EventPump) -> i32 {
    let (_width, height) = canvas.output_size().unwrap();
    let mouse_state: MouseState = e.mouse_state();
    let y = mouse_state.y();
    return y - height as i32 / 2;
}
fn mouse_y_abs(e: &sdl2::EventPump) -> i32 {
    let mouse_state: MouseState = e.mouse_state();
    let y = mouse_state.y();
    return y;
}
fn pressed(x: i32, y: i32, width: i32, height: i32, canvas: &mut WindowCanvas, e: &sdl2::EventPump) -> bool {
    if e.mouse_state().left() && mouse_x(canvas, e) > x - (width / 2) && mouse_x(canvas, e) < width - (width / 2) && mouse_y(canvas, e) > y - (height / 2) && mouse_y(canvas, e) < height - (height / 2) {
        return true;
    } else {
        return false;
    }
}
fn pressed_abs(x: i32, y: i32, width: i32, height: i32, e: &sdl2::EventPump) -> bool {
    if e.mouse_state().left() && mouse_x_abs(e) > x - (width / 2) && mouse_x_abs(e) < width && mouse_y_abs(e) > y - (height / 2) && mouse_y_abs(e) < height {
        return true;
    } else {
        return false;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialise sdl2 and create window
    let sdl_context = sdl2::init().expect("failed to init SDL");
    let video_subsystem = sdl_context.video().expect("failed to get video context");
    let window = video_subsystem.window("Milorad Image Creation Application", 800, 600)
        .build()
        .expect("failed to build window");
    let mut canvas: Canvas<Window> = window.into_canvas()
        .build()
        .expect("failed to build window's canvas");
    let texture_creator = canvas.texture_creator();
    // surface data
    let mut surface = Surface::new(1600, 800, PixelFormatEnum::RGBA32)?;
    // texture data
    let exe_dir = current_exe()?.parent().expect("").display().to_string();
    let mut textures = vec![texture_creator.load_texture(format!("{}/mica_assets/menu/logo.png", exe_dir)).expect(""), texture_creator.load_texture(format!("{}/mica_assets/menu/image.png", exe_dir)).expect("")];
    let mut positions: Vec<Point>;
    let mut sprites = vec![Rect::new(0, 0, 500, 166), Rect::new(0, 0, 300, 100)];
    let mut lastdrawnx = -1;
    let mut lastdrawny = -1;
    // other data
    let mut room = "menu";
    let mut event_pump = sdl_context.event_pump()?;
    let mut selected = 0;
    let mut loaded_textures = false;
    // enter loop
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        // check if the room is image, change sprites if it is, then render them
        let (width, height) = canvas.output_size().unwrap();
        let width_abs = ((width as f64 - width as f64 * 1.5) + 16.0) as i32;
        let height_abs = ((height as f64 - height as f64 * 1.5) + 16.0) as i32;
        positions = vec![Point::new(0, (height_abs as f64 / 1.25) as i32), Point::new(0, 0)];
        if room == "image" {
            if !loaded_textures {
                textures = vec![
                    texture_creator.load_texture(format!("{}/mica_assets/image/menubar.png", exe_dir))?,
                    texture_creator.load_texture(format!("{}/mica_assets/image/paint.png", exe_dir))?,
                    texture_creator.load_texture(format!("{}/mica_assets/image/erase.png", exe_dir))?,
                ];
                sprites = vec![
                    Rect::new(0, 0, 1632, 916),
                    Rect::new(0, 0, 32, 32),
                    Rect::new(0, 0, 32, 32),
                ];
                loaded_textures = true;
            }
            positions = vec![
                Point::new(width_abs + 801, height_abs + 442),
                Point::new(width_abs, height_abs),
                Point::new(width_abs, height_abs + 32),
            ];
            if pressed_abs(0, 0, 32, 32, &event_pump) {
                selected = 1;
            }
            if pressed_abs(0, 64, 32, 64, &event_pump) {
                selected = 2;
            }
            if selected != 0 && event_pump.mouse_state().left() {
                if selected == 1 {
                    if lastdrawnx != -1 && lastdrawny != -1 {
                        let xdif: f64 = mouse_x_abs(&event_pump) as f64 - lastdrawnx as f64 - 32.0;
                        let xstep: f64 = xdif / 1600.0;
                        let mut x = lastdrawnx as f64;
                        let ydif: f64 = mouse_y_abs(&event_pump) as f64 - lastdrawny as f64;
                        let ystep: f64 = ydif / 1600.0;
                        let mut y = lastdrawny as f64;
                        for _ in 0..1600 {
                            x += xstep;
                            y += ystep;
                            let surfrect = Rect::from_center(Point::new(x as i32, y as i32), 16, 16);
                            surface.fill_rect(Some(surfrect), Color::RGBA(0, 0, 0, 255))?;
                        }
                        lastdrawnx = mouse_x_abs(&event_pump) - 32;
                        lastdrawny = mouse_y_abs(&event_pump);
                    } else {
                        let surfrect = Rect::from_center(Point::new(mouse_x_abs(&event_pump) - 32, mouse_y_abs(&event_pump)), 16, 16);
                        surface.fill_rect(Some(surfrect), Color::RGBA(0, 0, 0, 255))?;
                        lastdrawnx = mouse_x_abs(&event_pump) - 32;
                        lastdrawny = mouse_y_abs(&event_pump);
                    }
                }
                if selected == 2 {
                    if lastdrawnx != -1 && lastdrawny != -1 {
                        let xdif: f64 = mouse_x_abs(&event_pump) as f64 - lastdrawnx as f64 - 32.0;
                        let xstep: f64 = xdif / 1600.0;
                        let mut x = lastdrawnx as f64;
                        let ydif: f64 = mouse_y_abs(&event_pump) as f64 - lastdrawny as f64;
                        let ystep: f64 = ydif / 1600.0;
                        let mut y = lastdrawny as f64;
                        for _ in 0..1600             {
                            x += xstep;
                            y += ystep;
                            let surfrect = Rect::from_center(Point::new(x as i32, y as i32), 16, 16);
                            surface.fill_rect(Some(surfrect), Color::RGBA(0, 0, 0, 0))?;
                        }
                        lastdrawnx = mouse_x_abs(&event_pump) - 32;
                        lastdrawny = mouse_y_abs(&event_pump);
                    } else {
                        let surfrect = Rect::from_center(Point::new(mouse_x_abs(&event_pump) - 32, mouse_y_abs(&event_pump)), 16, 16);
                        surface.fill_rect(Some(surfrect), Color::RGBA(0, 0, 0, 0))?;
                        lastdrawnx = mouse_x_abs(&event_pump) - 32;
                        lastdrawny = mouse_y_abs(&event_pump);
                    }
                }
            } else {
                lastdrawnx = -1;
                lastdrawny = -1;
            }
        } else if pressed(0, 0, 300, 100, &mut canvas, &event_pump) {
            room = "image";
        }
        let texture = surface.as_texture(&texture_creator)?;
        canvas.copy(&texture, None, Some(Rect::new(32, 0, 1600, 800)))?;
        render(&mut canvas, Color::RGBA(255, 255, 255, 255), &textures, positions.clone(), sprites.clone())?;
        canvas.clear();
        // time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
