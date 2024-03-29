mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::World;
use std::time::Duration;

use specs::prelude::*;

use crate::components::*;

pub enum MovementCommand {
    Stop,
    Move(Direction),
}

fn direction_spritesheet_row(last_direction: Option<Direction>, direction: Option<&Direction>) -> i32 {
    use self::Direction::*;

    let direction_to_use: Option<Direction>;

    if direction != None {
        let player_direction = direction.unwrap().to_owned();
        direction_to_use = Some(player_direction);
    } else {
        direction_to_use = last_direction;
    }

    match direction_to_use {
        Some(Up) => 3,
        Some(Down) => 0,
        Some(Left) => 1,
        Some(Right) => 2,
        None => 0,
    }
}

fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direction: Direction) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(Some(direction), None);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            )
        })
    }

    frames
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

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let textures = [texture_creator.load_texture("assets/bardo.png")?];

    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
    };

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity { speed: 0, direction: Direction::Down })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        let mut movement_command = None;
        
        // Game events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up))
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down))
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right))
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left))
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                },
                _ => {}
            }
        }

        *world.write_resource() = movement_command;

        // Update
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(33, 33, 33), &textures,world.system_data())?;

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
