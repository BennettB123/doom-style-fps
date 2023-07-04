use macroquad::prelude::*;

#[macroquad::main(get_window_conf())]
async fn main() {
    let mut state_items: Vec<Ball> = vec![];
    for _ in 0..100 {
        state_items.push(Ball::new_rand());
    }

    while !exit_button_pressed() {
        clear_background(BLACK);

        for i in 0..state_items.len() {
            state_items[i].update();
        }

        for item in &state_items {
            draw_circle(item.x, item.y, item.radius, item.color);
        }

        next_frame().await
    }
}

fn get_window_conf() -> Conf {
    Conf {
        window_title: "doom-style-fps".to_owned(),
        high_dpi: true,
        fullscreen: true,
        ..Conf::default()
    }
}

fn exit_button_pressed() -> bool {
    is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q)
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    radius: f32,
    color: Color,
}

impl Ball {
    fn new_rand() -> Self {
        Ball {
            x: 100.0,
            y: 100.0,
            dx: rand::gen_range(-20.0, 20.0),
            dy: rand::gen_range(-20.0, 20.0),
            radius: rand::gen_range(10.0, 100.0),
            color: GOLD,
        }
    }

    fn update(&mut self) {
        if self.x + self.radius > screen_width() || self.x - self.radius < 0.0 {
            self.dx = self.dx * -1.0;
        }
        if self.y + self.radius > screen_height() || self.y - self.radius < 0.0 {
            self.dy = self.dy * -1.0;
        }

        self.x += self.dx;
        self.y += self.dy;
    }
}
