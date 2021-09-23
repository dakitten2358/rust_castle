use rltk::{RGB};
use specs::prelude::*;
use specs_derive::Component;

use crate::components::{Position};

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub color: RGB,
    pub zorder: i32,
}

impl Renderable {
    pub fn new(icon: char, color: (u8, u8, u8)) -> Self {
        Self {
            glyph: rltk::to_cp437(icon),
            color: RGB::named(color),
            zorder: 0,
        }
    }

    pub fn new_with_z(icon: char, color: (u8, u8, u8), zorder: i32) -> Self {
        let mut new_renderable = Renderable::new(icon, color);
        new_renderable.zorder = zorder;
        return new_renderable;
    }
}

pub struct RenderSystem<'a> {
    context: &'a mut rltk::Rltk
}

impl<'a> RenderSystem<'a> {
    pub fn new(with_context: &'a mut rltk::Rltk) -> Self {
        Self {
            context: with_context
        }
    }
}

impl<'a> System<'a> for RenderSystem<'_> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        for zorder in 0..2 {
            for m in (&positions, &renderables).join().filter(|a| a.1.zorder == zorder) {
                let position = &m.0;
                let renderable = &m.1;

                self.context.set(position.x, position.y, renderable.color, rltk::RGB::named(rltk::BLACK), renderable.glyph);
            }
        }
        
    }
}
