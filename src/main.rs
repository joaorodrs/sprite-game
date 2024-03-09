use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::path::Path;
use std::time::Duration;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;

    canvas.present();
    
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Sprite Game", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(Path::new("assets/bardo.png"))?;

    let mut position = Point::new(0, 0);
    // Source position in the spritesheet
    let sprite = Rect::new(0, 0, 26, 36);

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Game events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    position.y = position.y() - 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    position.y = position.y() + 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    position.x = position.x() + 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    position.x = position.x() - 10;
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(30, 30, 30), &texture, position, sprite)?;

        // Time management
        ::std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
