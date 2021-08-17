use rltk::{RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub color: RGB,
}

impl Renderable {
    pub fn new(icon: char, color: (u8, u8, u8)) -> Self {
        Self {
            glyph: rltk::to_cp437(icon),
            color: RGB::named(color),
        }
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
    type SystemData = (ReadStorage<'a, crate::game::Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        for (position, renderable) in (&positions, &renderables).join() {
            self.context.set(position.x, position.y, renderable.color, rltk::RGB::named(rltk::BLACK), renderable.glyph);
        }
    }
}
