use rltk::VirtualKeyCode;
use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs_derive::Component;
use crate::items::ItemFlags;

#[derive(Component, Serialize, Deserialize)]
pub struct CombatStats {
    pub health: i32,
    pub max_health: i32,
}

#[derive(Component)]
pub struct AppliesDamage {
    pub damage: i32,
}

#[derive(Component)]
pub struct ApplyDamageComponent {
    pub amounts: Vec::<i32>,
    pub instigator: Entity,
}

#[derive(Component)]
pub struct WantsToAttack {
    pub target: Entity,
}

#[derive(Component)]
pub struct DeadTag {}

#[derive(Component, Serialize, Deserialize)]
pub struct CombatLog {
    pub logs: Vec<String>,
}

impl CombatLog {
    pub fn new() -> Self {
        Self {
            logs: vec!["".to_string(), "".to_string()],
        }
    }

    pub fn push(&mut self, text: String) {
        self.logs[0] = self.logs[1].clone();
        self.logs[1] = text.clone();
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct PlayerInputComponent {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
}

impl PlayerInputComponent {
    pub fn new() -> Self {
        Self {
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
        }
    }

    pub fn clear(&mut self) { 
        self.move_left = false;
        self.move_right = false;
        self.move_up = false;
        self.move_down = false;
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct PlayerInputMappingComponent {}

impl PlayerInputMappingComponent {
    pub fn process_key(&self, key: VirtualKeyCode, player_input: &mut PlayerInputComponent) {
        match key {
            // Movement
            VirtualKeyCode::Left => player_input.move_left = true,
            VirtualKeyCode::Right => player_input.move_right = true,
            VirtualKeyCode::Up => player_input.move_up = true,
            VirtualKeyCode::Down => player_input.move_down = true,            
            // Default
            _ => {}
        }
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct PlayerTextInputComponent {
    input_text: String,
    submitted: bool,
}

impl PlayerTextInputComponent {
    pub fn new() -> Self {
        Self {
            input_text: String::from(""),
            submitted: false,
        }
    }

    pub fn add_character(&mut self, c: char) {
        if !self.submitted {
            self.input_text.push(c);
        }
    }

    pub fn add_space(&mut self) {
        if !self.submitted {
            self.add_character(' ');
        }
    }

    pub fn backspace(&mut self) {
        if !self.submitted {
            self.input_text.pop();
        }
    }

    pub fn clear_text(&mut self) {
        if !self.submitted {
            self.input_text.clear();
        }
    }

    pub fn get_preview(&self) -> String {
        self.input_text.clone()
    }

    pub fn submit(&mut self) {
        self.submitted = true;
    }

    pub fn consume(&mut self) -> Option<String> {
        if self.submitted {
            let text = self.input_text.clone();
        
            self.submitted = false;
            self.input_text.clear();
            Some(text)
        }
        else
        {
            None
        }
    }
}

#[derive(Component, Debug)]
pub struct PickupTrigger {
    pub item_to_pickup: ItemFlags,
}

#[derive(Component, Serialize, Deserialize)]
pub struct InventoryComponent {
    items: ItemFlags,
}

impl InventoryComponent {
    pub fn new() -> Self {
        Self {
            items: ItemFlags::EMPTY,
        }
    }
    pub fn add(&mut self, item: ItemFlags) {
        self.items |= item;
    }
}
