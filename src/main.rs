mod background;
mod bird;
mod pipe_system;
mod traits;
mod uniforms;
mod utils;
mod vertex;
mod transformations;
mod texture;
mod score;
mod state;

use std::time::Duration;
use crate::pipe_system::PipeSystem;
use crate::traits::Drawable;
use glium;
use glium::{Display, Frame, glutin, Program};
use glium::Surface;
use crate::state::{GameOverLayer, GameState};
use crate::texture::Texture;

fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(500, 500))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let program = glium::Program::from_source(
        &display,
        &std::fs::read_to_string("./vert.shader").unwrap(),
        &std::fs::read_to_string("./frag.shader").unwrap(),
        None,
    )
    .unwrap();
    let mut game_over_layer = GameOverLayer::new(GameState::Rolling, &display);

    let mut background =
        background::Background::new("./assets/sprites/background-day.png", 1.4, &display, 0.05);
    let mut base = background::Background::new("./assets/sprites/base.png", 1.4, &display, 0.2);
    let mut bird = bird::Bird::new(&display);
    let mut pipe_system = PipeSystem::new(&display);
    pipe_system.init();

    let mut last_time = std::time::Instant::now();

    events_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::Key(input) => match input.virtual_keycode {
                    Some(keycode) => match keycode {
                        glutin::event::VirtualKeyCode::Space => {
                            bird.space_hit();
                        }
                        _ => {}
                    },
                    None => {}
                },
                _ => {}
            },
            _ => (),
        }

        let now = std::time::Instant::now();
        let mut dt = (now - last_time);
        last_time = now;

        if game_over_layer.game_ended() {
            dt = Duration::from_secs(0); // Stop the time if the game has ended
        }

        let mut layers: Vec<&mut dyn Drawable> = vec![
            &mut background,
            &mut pipe_system,
            &mut base,
            &mut bird,
            &mut game_over_layer,
        ];

        let mut frame = display.draw();
        frame.clear_color(1., 0., 0., 1.);


        for layer in &mut layers {
            frame = layer.draw(frame,&display, &program);
            layer.update(dt);
        }

        frame.finish().unwrap();
        if pipe_system.check_collision(&bird) {
            game_over_layer.set_end();
        }
        pipe_system.check_points(&bird);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(20),
        );
    });

}
