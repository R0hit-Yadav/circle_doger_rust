use macroquad::audio::{PlaySoundParams, load_sound, play_sound};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum CircleType {
    Normal,
    Fast,
    Big,
}

impl CircleType {
    fn get_points(&self) -> i32 {
        match self {
            CircleType::Normal => 100,
            CircleType::Fast => 150,
            CircleType::Big => 200,
        }
    }
}

struct FallingCricle {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
    color: Color,
    circle_type: CircleType,
    health: i32,
}

impl FallingCricle {
    fn new() -> Self {
        let colors = vec![RED, BLUE, GREEN, YELLOW, PURPLE, ORANGE, PINK, WHITE, LIGHTGRAY, DARKGRAY, BEIGE, BROWN, MAROON, PURPLE, VIOLET];
        let random_color = colors[rand::gen_range(0, colors.len())];

        let circle_type = match rand::gen_range(0, 100) {
            0..=70 => CircleType::Normal,
            71..=85 => CircleType::Fast,
            _ => CircleType::Big,
        };

        let (radius, speed, health) = match circle_type {
            CircleType::Normal => (20.0, rand::gen_range(100.0, 250.0), 1),
            CircleType::Fast => (15.0, rand::gen_range(250.0, 1000.0), 1),
            CircleType::Big => (35.0, rand::gen_range(80.0, 150.0), 2),
        };

        Self {
            x: rand::gen_range(0.0, screen_width()),
            y: -50.0,
            radius,
            speed,
            color: random_color,
            circle_type,
            health,
        }
    }

    fn update(&mut self, dt: f32) {
        self.y += self.speed * dt;
    }

    fn draw(&self) {
        match self.circle_type {
            CircleType::Normal => draw_circle(self.x, self.y, self.radius, self.color),
            CircleType::Fast => {
                draw_circle(self.x, self.y, self.radius, self.color);
                draw_circle(self.x, self.y - 10.0, self.radius * 0.8, Color::new(self.color.r, self.color.g, self.color.b, 0.5));
            }
            CircleType::Big => {
                draw_circle(self.x, self.y, self.radius, self.color);
                draw_circle_lines(self.x, self.y, self.radius + 5.0, 2.0, WHITE);
            }
        }
    }

    fn take_damage(&mut self) -> bool {
        self.health -= 1;
        self.health <= 0
    }

    fn is_of_screen(&self) -> bool {
        self.y - self.radius > screen_height()
    }

    fn collides_with(&self, px: f32, py: f32, pw: f32, ph: f32) -> bool {
        let closest_x = self.x.clamp(px, px + pw);
        let closest_y = self.y.clamp(py, py + ph);
        let dx = self.x - closest_x;
        let dy = self.y - closest_y;
        dx * dx + dy * dy < self.radius * self.radius
    }
}

struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    active: bool,
}

impl Bullet {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, speed: 800.0, active: true }
    }
    fn update(&mut self, dt: f32) { self.y -= self.speed * dt; }
    fn draw(&self) { draw_circle(self.x, self.y, 5.0, WHITE); }
    fn is_off_screen(&self) -> bool { self.y < 0.0 }
    fn collides_with_circle(&self, circle: &FallingCricle) -> bool {
        let dx = self.x - circle.x;
        let dy = self.y - circle.y;
        dx * dx + dy * dy < circle.radius * circle.radius
    }
}

#[macroquad::main("Circle Dodger")]
async fn main() {
    let explosion_sound = load_sound("./shoot.wav").await.unwrap();
    let background_sound = load_sound("./background.wav").await.unwrap();

    play_sound(
        &background_sound,
        PlaySoundParams { looped: true, volume: 0.3 },
    );

    loop { // 🔄 Game loop with restart support
        let mut player_x = screen_width() / 2.0;
        let mut player_y = screen_height() - 50.0;
        let player_size = 50.0;
        let player_speed = 300.0;

        let mut circles = Vec::new();
        let mut bullets = Vec::new();
        let mut spawn_timer = 0.0;

        let mut ammo = 20;
        let max_ammo = 20;
        let mut ammo_recharge_timer = 0.0; // ⏳ ammo recharge timer

        let mut score = 0;
        let mut lives = 3; // ❤️ multiple lives
        let mut hit_timer= 0.0;

        'game: loop {
            let dt = get_frame_time();

            if is_key_down(KeyCode::Left) { player_x -= player_speed * dt; }
            if is_key_down(KeyCode::Right) { player_x += player_speed * dt; }

            if is_key_pressed(KeyCode::Space) && ammo > 0 {
                bullets.push(Bullet::new(player_x + player_size / 2.0, player_y));
                ammo -= 1;
            }

            player_x = player_x.clamp(0.0, screen_width() - player_size);
            player_y = player_y.clamp(0.0, screen_height() - player_size);

            spawn_timer += dt;
            ammo_recharge_timer += dt;
            hit_timer -= dt;

            // 🔫 recharge ammo every 2 seconds
            if ammo < max_ammo && ammo_recharge_timer > 2.0 {
                ammo += 1;
                ammo_recharge_timer = 0.0;
            }

            for bullet in &mut bullets { bullet.update(dt); }
            bullets.retain(|b| !b.is_off_screen() && b.active);

            let mut circles_to_remove = Vec::new();
            for (circle_idx, circle) in circles.iter_mut().enumerate() {
                for bullet in &mut bullets {
                    if bullet.active && bullet.collides_with_circle(circle) {
                        bullet.active = false;
                        if circle.take_damage() {
                            circles_to_remove.push(circle_idx);
                            score += circle.circle_type.get_points();
                            play_sound(&explosion_sound, PlaySoundParams { looped: false, volume: 0.5 });
                        }
                    }
                }
            }
            
            if spawn_timer > 0.5 {
                circles.push(FallingCricle::new());
                spawn_timer = 0.0;
            }
            
            for circle in &mut circles { circle.update(dt); }
            circles.retain(|c| !c.is_of_screen());
            
            for (i, circle) in circles.iter().enumerate() {
                if circle.collides_with(player_x, player_y, player_size, player_size) {
                    lives -= 1;
                    hit_timer = 3.0; // 🛡️ hit invincibility timer
                    circles_to_remove.push(i);
                    if lives <= 0 {
                        break 'game;
                    }
                }
            }
            for &idx in circles_to_remove.iter().rev() { circles.remove(idx); }

            // clear_background(BLACK);
            draw_gradient_background();

            // 🎯 score = time survived + circle kills
            score += (dt * 10.0) as i32;

            draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE);
            draw_text(&format!("Ammo: {}/{}", ammo, max_ammo), 10.0, 50.0, 30.0, YELLOW);

            // ❤️ draw lives
            for i in 0..lives {
                draw_circle(20.0 + i as f32 * 30.0, 80.0, 10.0, RED);
            }

            if hit_timer <= 0.0 || (get_time() * 10.0) as i32 % 2 == 0 {
                let p1 = vec2(player_x + player_size / 2.0, player_y);          // nose
                let p2 = vec2(player_x, player_y + player_size);                // left wing
                let p3 = vec2(player_x + player_size, player_y + player_size);  // right wing
                draw_triangle(p1, p2, p3, BLUE);
            }
            for bullet in &bullets { bullet.draw(); }
            for circle in &circles { circle.draw(); }
            next_frame().await;
        }

        // 🟥 GAME OVER SCREEN
        loop {
            clear_background(BLACK);
            let game_over_text = "Game Over";
            let score_text = format!("Final Score: {}", score);
            let restart_text = "Press R to Restart";

            let game_over_x = screen_width()/2.0 - measure_text(game_over_text, None, 50, 1.0).width/2.0;
            let score_x = screen_width()/2.0 - measure_text(&score_text, None, 40, 1.0).width/2.0;
            let restart_x = screen_width()/2.0 - measure_text(restart_text, None, 30, 1.0).width/2.0;

            draw_text(game_over_text, game_over_x, screen_height()/2.0 - 40.0, 50.0, RED);
            draw_text(&score_text, score_x, screen_height()/2.0, 40.0, WHITE);
            draw_text(restart_text, restart_x, screen_height()/2.0 + 50.0, 30.0, GREEN);

            if is_key_pressed(KeyCode::R) { break; } // 🔄 Restart

            next_frame().await;
        }
    }
    
}


fn draw_gradient_background() {
    let time = get_time() as f32;

    let top_color = Color::new(
        0.1 + 0.2 * (time * 0.5).sin(),
        0.0,
        0.3 + 0.3 * (time * 0.3).cos(),
        1.0,
    );

    let bottom_color = Color::new(
        0.0,
        0.3 + 0.3 * (time * 0.4).sin(),
        0.6 + 0.3 * (time * 0.2).cos(),
        1.0,
    );

    let steps = 50; // smoother gradient = more steps
    let h = screen_height() / steps as f32;

    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let r = top_color.r * (1.0 - t) + bottom_color.r * t;
        let g = top_color.g * (1.0 - t) + bottom_color.g * t;
        let b = top_color.b * (1.0 - t) + bottom_color.b * t;

        let y = i as f32 * h;
        draw_rectangle(0.0, y, screen_width(), h, Color::new(r, g, b, 1.0));
    }
}