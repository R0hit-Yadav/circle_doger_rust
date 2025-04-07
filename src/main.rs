use macroquad::prelude::*;
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
        let colors = vec![RED, BLUE, GREEN, YELLOW, PURPLE, ORANGE];
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

#[macroquad::main("Circle Dodger")]
async fn main()
{
    // initialize the window and player properties
    let mut player_x = screen_width()/2.0;
    let mut player_y = screen_height()-50.0;
    let player_size = 50.0;
    let player_speed = 300.0;

    let mut circles = Vec::new();
    let mut spawn_timer = 0.0;

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


        // keep player in the screen 
        player_x = player_x.clamp(0.0, screen_width() - player_size);
        player_y = player_y.clamp(0.0, screen_height() - player_size);

        spawn_timer += dt;

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
                draw_text("Game Over", screen_width()/2.0 - 100.0, screen_height()/2.0, 50.0, RED);
                draw_text(&format!("Final Score: {}", score), screen_width()/2.0 - 100.0, screen_height()/2.0 + 60.0, 40.0, WHITE);
                next_frame().await;
                // after game finish wait for 5 sec
                std::thread::sleep(std::time::Duration::from_secs(3)); 
                std::process::exit(0); 
            }
        }

        // score and Drawing
        score += 1;
        draw_text(&format!("Score: {}", score), 10.0, 20.0, 30.0, WHITE);

        draw_rectangle(player_x, player_y, player_size, player_size, BLUE);

        for circle in &circles
        {
            circle.draw();
        }
        next_frame().await;
    }
}
