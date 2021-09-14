use specs::prelude::*;
use specs_derive::Component;

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

    fn print_input_text(&mut self, input_text: &String) {
        self.context.print(0, 24, "> ");
        self.context.print(2, 24, input_text);
    }

    fn print_action_result(&mut self, active_description: &String) {
        let start_x = 25;
        let start_y = 12;

        let mut current_x = start_x;
        let mut current_y = start_y;
        let tokens = active_description.split_whitespace();
        for token in tokens {
            let should_add_space = current_x != start_x;
            let text_length = token.len();
            let mut space_length = if should_add_space { 1 } else { 0 };

            // check to see if we should wrap to the next line
            if (current_x + text_length + space_length) >= 40 {
                current_y += 1;
                current_x = start_x;

                space_length = 0;
            }

            self.context.print(current_x + space_length, current_y, token);
            current_x += text_length + space_length;
        }
    }
}

fn draw_border_piece(context: &mut rltk::Rltk, x: i32, y: i32, glyph: char) {
    context.set(x, y, rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK), rltk::to_cp437(glyph));
}

impl<'a> System<'a> for HudSystem<'_> {
    type SystemData = (ReadStorage<'a, crate::game::Player>, ReadStorage<'a, crate::input::PlayerTextInputComponent>, ReadStorage<'a, crate::game::ActiveDescriptionComponent>, ReadExpect<'a, Vec<crate::room::RoomData>>);

    fn run(&mut self, (_players, player_text_inputs, active_descriptions, room_datas): Self::SystemData) {
        self.draw_map_border();

        let room_data = &room_datas[self.room as usize];
        self.print_description(room_data);

        for player_input in player_text_inputs.join() {
            self.print_input_text(&player_input.get_preview());
        }

        for active_description in active_descriptions.join() {
            self.print_action_result(&active_description.description);
        }
    }
}

#[derive(Component)]
pub struct DebugHudComponent {}

#[allow(dead_code)]
pub struct DebugHudSystem<'a> {
    context: &'a mut rltk::Rltk,
    state: &'a crate::State,
    room: i32,
}

impl<'a> DebugHudSystem<'a> {
    pub fn new(state: &'a crate::State, with_context: &'a mut rltk::Rltk, room: i32) -> Self {
        Self {
            context: with_context,
            state: state,
            room: room,
        }
    }
}

impl<'a> System<'a> for DebugHudSystem<'_> {
    type SystemData = (ReadStorage<'a, crate::game::Player>, ReadStorage<'a, DebugHudComponent>, ReadExpect<'a, Vec<crate::room::RoomData>>);

    fn run(&mut self, (players, debug_huds, _room_datas): Self::SystemData) {
        // debug
        for (_player, _debug) in (&players, &debug_huds).join() {
            self.context.print(37, 24, self.room.to_string());
        }        
    }
}

    