use godot::{
    engine::{CharacterBody2D, ICharacterBody2D, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Paddle1 {
    #[export]
    speed: f32,

    #[base]
    sprite: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Paddle1 {
    fn init(sprite: Base<CharacterBody2D>) -> Self {
        godot_print!("Iniciei o Paddle 1!");

        Self { speed: 0., sprite }
    }

    fn process(&mut self, delta: f64) {
        //Variables
        let mut position = self.sprite.get_position();
        let input = Input::singleton();

        //Moviment
        let dir = input.get_axis("up".into(), "down".into());
        if dir > 0. || dir < 0. {
            position.y += dir * self.speed * delta as f32;
        }

        self.sprite.set_position(position);
        self.sprite.move_and_slide();
    }
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Paddle2 {
    #[export]
    speed: f32,

    #[base]
    sprite: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Paddle2 {
    fn init(sprite: Base<CharacterBody2D>) -> Self {
        godot_print!("Iniciei o Paddle 2!");

        Self { speed: 0., sprite }
    }

    fn process(&mut self, delta: f64) {
        //Variables
        let mut position = self.sprite.get_position();
        let input = Input::singleton();

        //Moviment
        let dir = input.get_axis("up2".into(), "down2".into());
        if dir > 0. || dir < 0. {
            position.y += dir * self.speed * delta as f32;
        }

        self.sprite.set_position(position);
        self.sprite.move_and_slide();
    }
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PaddleCPU {
    #[export]
    speed: f32,

    #[base]
    sprite: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for PaddleCPU {
    fn init(sprite: Base<CharacterBody2D>) -> Self {
        godot_print!("Iniciei o Paddle 2!");

        Self { speed: 0., sprite }
    }

    fn process(&mut self, delta: f64) {
        //Variables
        let mut position = self.sprite.get_position();
        let mut dir = Vector2::ZERO;

        //MOVIMENT
        let ball_position = self
            .sprite
            .get_parent()
            .unwrap()
            .get_node_as::<Sprite2D>("Ball")
            .get_position();

        if ball_position.y - 85. < position.y {
            dir.y = -1.
        }
        if ball_position.y - 85. > position.y {
            dir.y = 1.
        }

        position.y += dir.y * self.speed * delta as f32;

        self.sprite.set_position(position);
        self.sprite.move_and_slide();
    }
}
