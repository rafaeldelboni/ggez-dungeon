use specs::{Component, VecStorage};

#[derive(Clone)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub slide: bool,
    pub jump: bool,
    pub attack: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            slide: false,
            jump: false,
            attack: false,
        }
    }

    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
        self.slide = false;
        self.jump = false;
        self.attack = false;
    }
}

impl Default for Input {
    fn default() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            slide: false,
            jump: false,
            attack: false,
        }
    }
}

#[derive(Debug)]
pub struct Controlable;

impl Component for Controlable {
    type Storage = VecStorage<Self>;
}
