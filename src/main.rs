use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::collections::VecDeque;
use std::path::Path;
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction { Up, Down, Left, Right }

struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: VecDeque<Direction>,
}

impl Player {
    pub fn new() -> Player {
        let position = Point::new(0, 0);
        let sprite = Rect::new(0, 0, 26, 36);
        let initial_direction = VecDeque::from([]);

        Player {
            position,
            sprite,
            speed: 0,
            direction: initial_direction
        }
    }
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.present();
    
    Ok(())
}

fn update_player(player: &mut Player) {
    // Allows the use of `Left`, `Right`, `Up` and `Down` more easily.
    use self::Direction::*;

    match player.direction.back() {
        Some(Left) => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Some(Right) => {
            player.position = player.position.offset(player.speed, 0);
        },
        Some(Up) => {
            player.position = player.position.offset(0, -player.speed);
        },
        Some(Down) => {
            player.position = player.position.offset(0, player.speed);
        },
        None => {},
    }
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

    let mut player = Player::new();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Game events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    if !player.direction.contains(&Direction::Up) {
                        player.direction.push_back(Direction::Up);
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    println!("Down");
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    if !player.direction.contains(&Direction::Down) {
                        player.direction.push_back(Direction::Down);
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    println!("Right");
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    if !player.direction.contains(&Direction::Right) {
                        player.direction.push_back(Direction::Right);
                    };
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    if !player.direction.contains(&Direction::Left) {
                        player.direction.push_back(Direction::Left);
                    };
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    if let Some(index) = player.direction.iter().position(|&x| x == Direction::Left) {
                        player.direction.remove(index);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    if let Some(index) = player.direction.iter().position(|&x| x == Direction::Right) {
                        player.direction.remove(index);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    if let Some(index) = player.direction.iter().position(|&x| x == Direction::Up) {
                        player.direction.remove(index);
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    if let Some(index) = player.direction.iter().position(|&x| x == Direction::Down) {
                        player.direction.remove(index);
                    }
                },
                _ => {}
            }
        }

        // Update
        update_player(&mut player);

        // Render
        render(&mut canvas, Color::RGB(30, 30, 30), &texture, &player)?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }

    Ok(())
}
