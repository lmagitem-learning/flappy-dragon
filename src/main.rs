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

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(5, 25),
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
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, context: &mut BTerm) {
        context.cls();
        context.print_centered(5, "You are dead!");
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
