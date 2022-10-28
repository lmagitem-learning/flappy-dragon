use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, context: &mut BTerm) {
        context.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity += -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&self, context: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Draw top half
        for y in 0..self.gap_y - half_size {
            context.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        // Draw bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            context.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "Welcome to Flappy Dragon");
        context.print_centered(8, "(P)lay Game");
        context.print_centered(9, "(Q)uit Game");

        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, context: &mut BTerm) {
        context.cls_bg(NAVY);
        self.frame_time += context.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = context.key {
            self.player.flap();
        }
        self.player.render(context);
        context.print(0, 0, "Press SPACE to flap.");
        context.print(0, 1, format!("Score {}", self.score));

        self.obstacle.render(context, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "You are dead!");
        context.print_centered(6, &format!("You earned {} points", self.score));
        context.print_centered(8, "(P)lay More");
        context.print_centered(9, "(Q)uit Game");

        if let Some(key) = context.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => context.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.player = Player::new(5, 25);
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(context),
            GameMode::Playing => self.play(context),
            GameMode::End => self.dead(context),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
