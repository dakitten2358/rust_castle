use specs::prelude::*;
use specs_derive::Component;

use crate::room::{RoomData};
use crate::game::{Player, ActiveDescriptionComponent, Description};
use crate::input::{PlayerTextInputComponent};
use crate::combat::{CombatLog};
use crate::render::Renderable;

#[allow(dead_code)]
pub struct HudSystem<'a> {
    context: &'a mut rltk::Rltk,
    state: &'a crate::State,
    room: i32,
}

// 10 lines for on screen items
// 2 lines blank
// 5 lines for log

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

    fn print_description(&mut self, room_data: &RoomData) {
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
        let start_y = 11;

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

    fn print_combat_logs(&mut self, combat_log: &CombatLog) {
        let start_x = 25;
        let start_y = 17;
        let mut current_y = start_y;
        for text in combat_log.logs.iter() {
            self.context.print(start_x, current_y, text);
            current_y += 1;
        }
    }

    fn print_glyph_descriptions(&mut self, renderables: &ReadStorage<'a, Renderable>, descriptions: &ReadStorage<'a, Description>) {
        let start_x = 25;
        let start_y = 0;
        let mut current_y = start_y;

        for (renderable, description) in (renderables, descriptions).join() {
            self.context.set(start_x, current_y, renderable.color, rltk::RGB::named(rltk::BLACK), renderable.glyph);
            self.context.print(start_x+2, current_y, description.name.as_str());
            current_y += 1;
        }
    }
}

fn draw_border_piece(context: &mut rltk::Rltk, x: i32, y: i32, glyph: char) {
    context.set(x, y, rltk::RGB::named(rltk::WHITE), rltk::RGB::named(rltk::BLACK), rltk::to_cp437(glyph));
}

impl<'a> System<'a> for HudSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Player>, 
        ReadStorage<'a, PlayerTextInputComponent>, 
        ReadStorage<'a, ActiveDescriptionComponent>, 
        ReadExpect<'a, Vec<RoomData>>,
        ReadStorage<'a, CombatLog>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Description>,
    );

    fn run(&mut self, (_players, player_text_inputs, active_descriptions, room_datas, combat_logs, renderables, descriptions): Self::SystemData) {
        self.draw_map_border();

        let room_data = &room_datas[self.room as usize];
        self.print_description(room_data);

        for player_input in player_text_inputs.join() {
            self.print_input_text(&player_input.get_preview());
        }

        for active_description in active_descriptions.join() {
            self.print_action_result(&active_description.description);
        }

        for combat_log in combat_logs.join() {
            self.print_combat_logs(&combat_log);
        }

        self.print_glyph_descriptions(&renderables, &descriptions);
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
    type SystemData = (ReadStorage<'a, Player>, ReadStorage<'a, DebugHudComponent>, ReadExpect<'a, Vec<RoomData>>);

    fn run(&mut self, (players, debug_huds, _room_datas): Self::SystemData) {
        // debug
        for (_player, _debug) in (&players, &debug_huds).join() {
            self.context.print(37, 24, self.room.to_string());
        }        
    }
}

    