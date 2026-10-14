use godot::prelude::*;
use godot::classes::Input;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {

    // 重力加速度
    gravity: f32,

    // 当前速度向量
    velocity: Vector2,

    // 水平移动速度
    move_speed: f32,

    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!"); // 输出到 Godot 控制台
        
        Self {
            gravity: 9.8 * 100.0, // 乘以 100 是因为 Godot 单位是像素，按需调整
            velocity: Vector2::ZERO,
            move_speed: 100.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {

        let delta = delta as f32;

        // 计算重力加速度
        let new_y = self.velocity.y + self.gravity * delta;
        //把算好的速度重新写入self.velocity变量
        self.velocity = Vector2::new(self.velocity.x, new_y);

         // —— 按键输入 ——
        let input = Input::singleton();
        let mut direction = 0.0;

         if input.is_action_pressed("右") {
            direction += 1.0;
        }
        if input.is_action_pressed("左") {
            direction -= 1.0;
        }

        self.velocity = Vector2::new(direction * self.move_speed, self.velocity.y);

        let velocity = self.velocity;
        self.base_mut().set_velocity(velocity);     //告诉引擎想以这个速度移动
        self.base_mut().move_and_slide();           //此函数让它真正移动并处理碰撞
        self.velocity = self.base().get_velocity(); //读取引擎处理后的实际速度，同步回 self.velocity。
        
    }
}

