use specs::prelude::*;

#[allow(dead_code)]
pub struct HudSystem<'a> {
    context: &'a mut rltk::Rltk,
    state: &'a crate::State,
    room: i32,
}

impl<'a> HudSystem<'a> {
    pub fn new(state: &'a crate::State, with_context: &'a mut rltk::Rltk, room: i32) -> Self {
        Self {
            context: with_context,
            state: state,
            room: room,
        }
    }

    fn draw_map_border(&mut self)
    {
        for row in 0..18 {
            draw_border_piece(self.context, 24, row, '│');
        }
        for col in 0..24 {
            draw_border_piece(self.context, col, 18, '─');
        }
        draw_border_piece(self.context, 24, 18, '┘');
    }

    fn print_description(&mut self, room_data: &crate::room::RoomData) {
        for row in 0..5 {
            self.context.print(0, 19 + row, &room_data.description[row]);
        }
        
    }
}

fn draw_border_piece(context: &mut rltk::Rltk, x: i32, y: i32, glyph: char) {
    context.set(x, y, rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK), rltk::to_cp437(glyph));
}

impl<'a> System<'a> for HudSystem<'_> {
    type SystemData = (ReadStorage<'a, crate::game::Player>, ReadExpect<'a, Vec<crate::room::RoomData>>);

    fn run(&mut self, (_players, room_datas): Self::SystemData) {
        self.draw_map_border();

        let room_data = &room_datas[self.room as usize];
        self.print_description(room_data);
    }
}
