use godot::{
    engine::{ILabel, Label},
    prelude::*,
};

use crate::{SCORE1, SCORE2};

#[derive(GodotClass)]
#[class(base=Label)]
struct Score1 {
    #[base]
    base: Base<Label>,
}

#[derive(GodotClass)]
#[class(base=Label)]
struct Score2 {
    #[base]
    base: Base<Label>,
}

#[godot_api]
impl ILabel for Score1 {
    fn init(base: Base<Label>) -> Self {
        Self { base }
    }

    fn process(&mut self, _: f64) {
        self.base.set_text(unsafe { SCORE1.to_string().into() })
    }
}

#[godot_api]
impl ILabel for Score2 {
    fn init(base: Base<Label>) -> Self {
        Self { base }
    }

    fn process(&mut self, _: f64) {
        self.base.set_text(unsafe { SCORE2.to_string().into() })
    }
}
