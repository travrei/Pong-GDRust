use godot::{
    engine::{AudioStreamPlayer2D, ISprite2D, Sprite2D, Timer},
    prelude::*,
};
use rand::Rng;

use crate::{SCORE1, SCORE2};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Ball {
    direction: Vector2,
    #[export]
    speed: f32,
    #[base]
    base: Base<Sprite2D>,
}

impl Ball {
    fn randon_dir_x(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let mut rand = 0;
        while rand == 0 {
            rand = rng.gen_range(-1..2);
        }
        return rand;
    }
    fn randon_dir_y(&mut self) -> f32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(-0.45..0.45)
    }
}

#[godot_api]
impl Ball {
    #[func]
    fn _on_start_timer_timeout(&mut self) {
        self.direction.x = self.randon_dir_x() as f32;
        self.direction.y = self.randon_dir_y();
    }
    #[func]
    fn _on_body_entered(&mut self) {
        self.direction.x *= -1.;
        self.base
            .get_node_as::<AudioStreamPlayer2D>("collision")
            .play();
    }
    #[func]
    fn _on_wall_entered(&mut self) {
        self.base
            .get_node_as::<AudioStreamPlayer2D>("collision")
            .play();
        self.direction.y *= -1.
    }
}

#[godot_api]
impl ISprite2D for Ball {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Iniciado a Bola!");

        Self {
            direction: Vector2::ZERO,
            speed: 0.,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut position = self.base.get_position();
        let mut starttimer = self.base.get_node_as::<Timer>("StartTimer");
        let mut point_sfx = self.base.get_node_as::<AudioStreamPlayer2D>("point");
        position += self.direction.normalized() * self.speed * delta as f32;

        //ScoreSystem
        if self.base.get_position().x > 1280. {
            unsafe { SCORE1 += 1 }
            position = Vector2::new(1280. / 2., 720. / 2.);
            self.direction = Vector2::ZERO;
            point_sfx.play();
            starttimer.start()
        }
        if self.base.get_position().x < 0. {
            unsafe { SCORE2 += 1 }
            position = Vector2::new(1280. / 2., 720. / 2.);
            self.direction = Vector2::ZERO;
            point_sfx.play();
            starttimer.start()
        }
        self.base.set_position(position);
    }
}
