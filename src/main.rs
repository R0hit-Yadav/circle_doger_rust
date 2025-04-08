use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
// falling circle struct 
struct FallingCricle 
{
    x:f32,
    y:f32,
    radius:f32,
    speed:f32,
    color: Color,
}

impl FallingCricle
{

    fn new()->Self
    {
        // generate random x position, speed and color
        let colors = vec![RED, BLUE, GREEN, YELLOW, PURPLE, ORANGE, PINK,
            WHITE, LIGHTGRAY, DARKGRAY, BEIGE, BROWN, MAROON, PURPLE, VIOLET];
        let random_color = colors[rand::gen_range(0, colors.len())];
        
        Self { 
            x:rand::gen_range(0.0, screen_width()), 
            y: -50.0, 
            radius: 20.0, 
            speed: rand::gen_range(100.0, 250.0),
            color: random_color,
        }
    }

    // update the position of the circle
    fn update(&mut self,dt:f32)
    {
        self.y+= self.speed*dt;
    }

    // draw the circle on screen 
    fn draw(&self)
    {
        draw_circle(self.x, self.y, self.radius, self.color);
    }

    fn is_of_screen(&self)-> bool
    {
        self.y - self.radius > screen_height()
    }

    // logic of collision detection
    fn collides_with(&self,px:f32,py:f32,pw:f32,ph:f32)->bool
    {
        // find closest point rectangle to circle
        let closest_x=self.x.clamp(px, px + pw);
        let closest_y=self.y.clamp(py, py + ph);

        // calculate the distance between the circle and the closest point
        let dx=self.x - closest_x;
        let dy =self.y - closest_y;

        // check if the distance is less than the radius
        dx*dx + dy*dy < self.radius*self.radius
    }

}

// Bullet struct for firing mechanism
struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    active: bool,
}

impl Bullet {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            speed: 800.0,
            active: true,
        }
    }

    fn update(&mut self, dt: f32) {
        self.y -= self.speed * dt;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, 5.0, WHITE);
    }

    fn is_off_screen(&self) -> bool {
        self.y < 0.0
    }

    fn collides_with_circle(&self, circle: &FallingCricle) -> bool {
        let dx = self.x - circle.x;
        let dy = self.y - circle.y;
        dx * dx + dy * dy < circle.radius * circle.radius
    }
}

#[macroquad::main("Circle Dodger")]
async fn main()
{
    // Load sound effect at startup
    let explosion_sound = load_sound("./explosion.wav").await.unwrap();

    // initialize the window and player properties
    let mut player_x = screen_width()/2.0;
    let mut player_y = screen_height()-50.0;
    let player_size = 50.0;
    let player_speed = 300.0;

    let mut circles = Vec::new();
    let mut bullets = Vec::new();
    let mut spawn_timer = 0.0;
    let mut ammo = 10000; // Limited ammunition
    let max_ammo = 10000;

    let mut score = 0;
    loop 
    {
        let dt = get_frame_time(); // time 

        // for key movements
        if is_key_down(KeyCode::Left)
        {
            player_x -= player_speed * dt;
        }
        if is_key_down(KeyCode::Right)
        {
            player_x += player_speed * dt;
        }
        //for up and down movements
        // if is_key_down(KeyCode::Up)
        // {
        //     player_y -= player_speed * dt;
        // }
        // if is_key_down(KeyCode::Down)
        // {
        //     player_y += player_speed * dt;
        // }

        // Bullet firing with Space key
        if is_key_pressed(KeyCode::Space) && ammo > 0 {
            bullets.push(Bullet::new(player_x + player_size / 2.0, player_y));
            ammo -= 1;
        }

        // keep player in the screen 
        player_x = player_x.clamp(0.0, screen_width() - player_size);
        player_y = player_y.clamp(0.0, screen_height() - player_size);

        spawn_timer += dt;

        // Update bullets
        for bullet in &mut bullets {
            bullet.update(dt);
        }

        // Remove off-screen bullets
        bullets.retain(|bullet| !bullet.is_off_screen());

        // Check bullet collisions with circles
        let mut circles_to_remove = Vec::new();
        for (circle_idx, circle) in circles.iter().enumerate() {
            for bullet in &mut bullets {
                if bullet.active && bullet.collides_with_circle(circle) {
                    circles_to_remove.push(circle_idx);
                    bullet.active = false;
                    score += 1000; // Bonus points for shooting circles
                    
                    // Play explosion sound
                    play_sound(&explosion_sound, PlaySoundParams {
                        looped: false,
                        volume: 0.5,
                    });
                }
            }
        }

        // Remove hit circles and inactive bullets
        for &idx in circles_to_remove.iter().rev() 
        {
            circles.remove(idx);
        }
        bullets.retain(|bullet| bullet.active);

        // cricle spawn logic
        if spawn_timer > 0.5
        {
            circles.push(FallingCricle::new());
            spawn_timer = 0.0;
        }

        // update circle positions
        for circle in &mut circles
        {
            circle.update(dt);
        }

        // remove circles that are off the screen
        circles.retain(|circle| !circle.is_of_screen());

        // collision detection
        for circle in &circles
        {
            if circle.collides_with(player_x,player_y, player_size, player_size)
            {
                // Show game over message
                clear_background(BLACK);
                
                let game_over_text = "Game Over";
                let score_text = format!("Final Score: {}", score);
                
                // Calculate text dimensions for centering
                let game_over_size = 50.0;
                let score_size = 40.0;
                let screen_center_x = screen_width() / 2.0;
                let screen_center_y = screen_height() / 2.0;
                
                // Calculate text positions for perfect centering
                let game_over_x = screen_center_x - measure_text(game_over_text, None, game_over_size as u16, 1.0).width / 2.0;
                let score_x = screen_center_x - measure_text(&score_text, None, score_size as u16, 1.0).width / 2.0;
                
                // Draw centered text
                draw_text(game_over_text, game_over_x, screen_center_y - 30.0, game_over_size, RED);
                draw_text(&score_text, score_x, screen_center_y + 30.0, score_size, WHITE);
                
                next_frame().await;
                // after game finish wait for 5 sec
                std::thread::sleep(std::time::Duration::from_secs(3)); 
                std::process::exit(0); 
            }
        }

        clear_background(BLACK);

        // Draw UI: Score and Ammo
        score += 1;
        draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Ammo: {}/{}", ammo, max_ammo), 10.0, 50.0, 30.0, YELLOW);

        // Draw player
        draw_rectangle(player_x, player_y, player_size, player_size, BLUE);

        // Draw bullets
        for bullet in &bullets {
            bullet.draw();
        }

        // Draw circles
        for circle in &circles
        {
            circle.draw();
        }
        next_frame().await;
    }
}







