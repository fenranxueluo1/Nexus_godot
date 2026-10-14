use godot::prelude::*;
use godot::classes::Sprite2D;
use godot::classes::ISprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // 输出到 Godot 控制台
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // 在 GDScript中，这将是： 
        // rotation += angular_speed * delta
        
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // 'rotate' 方法需要一个 f32， 
        // 因此我们将 'self.angular_speed * delta' 的f64 转换为 f32

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased", &[]);
    }

    #[signal]
    fn speed_increased();
}
