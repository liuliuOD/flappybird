use std::cell::RefCell;
use glium::backend::Facade;
use glium::{Frame, Program};
use std::time::Duration;
use crate::{Drawable, Texture};

pub struct Background {
    block_width: f32,
    start_time: std::time::Instant,
    texture: RefCell<Texture>,
    speed: f32,
    offset: f32,
}

impl Background {
    pub fn new(
        sprite_path: &str,
        block_width: f32,
        display: &dyn Facade,
        speed: f32,
    ) -> Background {
        Background {
            block_width,
            texture: RefCell::new(Texture::new(sprite_path, display, (0., 0.), Some(block_width))),
            start_time: std::time::Instant::now(),
            speed,
            offset: 0f32,
        }
    }
}
impl<'b, 'c> Drawable<'b, 'c> for Background {
    fn draw(
        &self,
        mut frame: Frame,
        facade: &'b dyn Facade,
        program: &'c Program,
    ) -> Frame {
        let blocks_number = (2. / self.block_width).ceil() as i32 + 1;

        let texture = self.texture.borrow();
        let height = texture.get_height();
        drop(texture);

        let mut texture = self.texture.borrow_mut();
        for i in 0..blocks_number {
            let current_offset = -1. + self.block_width * (i as f32) + (self.offset % self.block_width);

            texture.set_pos((current_offset, -1. + height));
            frame = texture.draw(frame, facade, program);
        }

        frame

    }

    fn update(&mut self, dt: Duration) {
        self.offset -= dt.as_secs_f32() * self.speed;
    }
}
