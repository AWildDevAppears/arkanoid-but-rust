use raylib::prelude::*;

mod physics;

struct GameWindow {
    width: i32,
    height: i32,
    title: &'static str,
}

const GAME_WINDOW: GameWindow = GameWindow {
    width: 640,
    height: 480,
    title: "Arkanoid",
};

#[derive(Clone, Copy)]
enum PillVariant {
    GREEN,
    PINK,
    BLUE,
    YELLOW,
    RED,
    GREY,
}

impl PillVariant {
    fn to_color(&self) -> Color {
        match self {
            PillVariant::GREEN => Color::GREEN,
            PillVariant::PINK => Color::PINK,
            PillVariant::BLUE => Color::BLUE,
            PillVariant::YELLOW => Color::YELLOW,
            PillVariant::RED => Color::RED,
            PillVariant::GREY => Color::GRAY,
        }
    }

}

#[derive(Clone, Copy)]
struct BreakablePill {
    x: f32,
    y: f32,
    variant: PillVariant,
    alive: bool,
}

struct GameSettings {
    player_default_speed: f32,
    player_default_width: f32,
    player_default_height: f32,
    pill_width: f32,
    pill_height: f32,
    pill_default_rows: [PillVariant; 6],
    pill_offset: f32,
    pill_column_length: usize,
    ball_velocity: f32,
    ball_radius: f32,
}

const GAME_SETTINGS: GameSettings = GameSettings {
    player_default_speed: 0.5,
    player_default_width: 50.0,
    player_default_height: 10.0,
    pill_width: 50.0,
    pill_height: 10.0,
    pill_default_rows: [PillVariant::GREY, PillVariant::RED, PillVariant::YELLOW, PillVariant::BLUE, PillVariant::PINK, PillVariant::GREEN],
    pill_offset: 2.0,
    pill_column_length: 12,
    ball_velocity: 1.0,
    ball_radius: 6.0,
};


enum BallState {
    Stuck,
    Free,
}

struct BallInformation {
    state: BallState,
    position: Vector2,
    last_position: Vector2,
}

enum CollisionSide {
    None,
    Top,
    Left,
    Right,
    Bottom,
}

fn pill_x_index_offset() -> f32 {
    let taken_space = GAME_SETTINGS.pill_column_length as f32 * (GAME_SETTINGS.pill_width + GAME_SETTINGS.pill_offset);
    return (GAME_WINDOW.width as f32 - taken_space) / 2.0;
}

fn get_collision_side(obstacle: Rectangle, ball: Vector2, radius: f32) -> CollisionSide {
    let closest_x = ball.x.clamp(obstacle.x, obstacle.x + obstacle.width);
    let closest_y = ball.y.clamp(obstacle.y, obstacle.y + obstacle.height);

    let distance_x = ball.x - closest_x;
    let distance_y = ball.y - closest_y;

    let distance_squared = (distance_x * distance_x) + (distance_y * distance_y);

    if distance_squared > radius * radius {
        return CollisionSide::None;
    }

    if distance_x == 0.0 && distance_y == 0.0 {
        let left_dist = (ball.x - obstacle.x).abs();
        let right_dist = (ball.x - (obstacle.x + obstacle.width)).abs();
        let top_dist = (ball.y - obstacle.y).abs();
        let bottom_dist = (ball.y - (obstacle.y + obstacle.height)).abs();

        let min_dist = left_dist.min(right_dist).min(top_dist).min(bottom_dist);

        if min_dist == left_dist { return CollisionSide::Left; }
        if min_dist == right_dist { return CollisionSide::Right; }
        if min_dist == top_dist { return CollisionSide::Top; }
        return CollisionSide::Bottom;
    }

    if distance_x.abs() > distance_y.abs() {
        if distance_x > 0.0 { CollisionSide::Right } else { CollisionSide::Left }
    } else {
        if distance_y > 0.0 { CollisionSide::Bottom } else { CollisionSide::Top }
    }
}

fn handle_ball_free(ball: Vector2, last_position: Vector2, colliders: Vec<Rectangle>) -> Vector2 {
    // TODO: Ball currently gets stuck inside the paddle
    let mut x_position = ball.x;
    let mut y_position = ball.y;

    let mut applied_vert = false;
    let mut applied_hor = false;

    for collider in colliders {
        let side = get_collision_side(collider, ball, GAME_SETTINGS.ball_radius);
        match side {
            CollisionSide::Top => {
                if !applied_vert {
                    y_position = y_position - GAME_SETTINGS.ball_velocity;
                    applied_vert = true;
                }
            },
            CollisionSide::Bottom => {
                if !applied_vert {
                    y_position = y_position + GAME_SETTINGS.ball_velocity;
                    applied_vert = true;
                }
            },
            CollisionSide::Left => {
                if !applied_hor {
                    x_position = x_position - GAME_SETTINGS.ball_velocity;
                    applied_hor = true;
                }
            },
            CollisionSide::Right => {
                if !applied_hor {
                    x_position = x_position + GAME_SETTINGS.ball_velocity;
                    applied_hor = true;
                }
            },
            _ =>  {}
        }
    }

    if y_position == ball.y && x_position == ball.x {
        x_position = if ball.x - last_position.x > 0.0 { x_position + GAME_SETTINGS.ball_velocity } else { x_position - GAME_SETTINGS.ball_velocity };
        y_position = if ball.y - last_position.y >= 0.0 { y_position + GAME_SETTINGS.ball_velocity } else { y_position - GAME_SETTINGS.ball_velocity };
    }

    return Vector2 { x: x_position, y: y_position  };

}

fn main() {
    let (mut game, thread) = raylib::init()
        .size(GAME_WINDOW.width, GAME_WINDOW.height)
        .title(&GAME_WINDOW.title)
        .build();

    let mut player = Rectangle::new(
        (GAME_WINDOW.width as f32 / 2.0) - (GAME_SETTINGS.player_default_width / 2.0),
        GAME_WINDOW.height as f32 - (GAME_SETTINGS.player_default_height * 2.0), GAME_SETTINGS.player_default_width,
        GAME_SETTINGS.player_default_height,
    );

    let camera = Camera2D {
        target: Vector2 { x: 0.0, y: 0.0 },
        offset: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    let default_pill = BreakablePill {
        x: 0.0,
        y: 0.0,
        variant: PillVariant::PINK,
        alive: true,
    };

    let mut pills: [[BreakablePill; GAME_SETTINGS.pill_column_length]; 6] = [[default_pill; GAME_SETTINGS.pill_column_length]; 6];

    for (row_idx, row) in pills.iter_mut().enumerate() {
        for (col_idx, pill) in row.iter_mut().enumerate() {
            pill.x = (col_idx as f32 * (GAME_SETTINGS.pill_width + GAME_SETTINGS.pill_offset)) + pill_x_index_offset();
            pill.y = row_idx as f32 * (GAME_SETTINGS.pill_height + GAME_SETTINGS.pill_offset);
            pill.variant = GAME_SETTINGS.pill_default_rows[row_idx];
        }
    }

    let mut ball_information = BallInformation {
        state: BallState::Stuck,
        position: Vector2 { x: 0.0, y: 0.0 },
        last_position: Vector2 { x: 0.0, y: 0.0 },
    };

    let bottom_line = Rectangle::new(0.0, GAME_WINDOW.height as f32 - 1.0, GAME_WINDOW.width as f32, 1.0);
    let top_line = Rectangle::new(0.0, 1.0, GAME_WINDOW.width as f32, 1.0);
    let left_line = Rectangle::new(0.0, 0.0, 1.0, GAME_WINDOW.height as f32);
    let right_line = Rectangle::new(GAME_WINDOW.width as f32 - 1.0, 0.0, 1.0, GAME_WINDOW.height as f32);

    while !game.window_should_close() {
        if game.is_key_down(KeyboardKey::KEY_RIGHT) {
            player.x += GAME_SETTINGS.player_default_speed;

            if player.x + GAME_SETTINGS.player_default_width >= GAME_WINDOW.width as f32 {
                player.x = GAME_WINDOW.width as f32 - GAME_SETTINGS.player_default_width;
            }
        } else if game.is_key_down(KeyboardKey::KEY_LEFT) {
            player.x -= GAME_SETTINGS.player_default_speed;

            if player.x <= 0.0 {
                player.x = 0.0;
            }
        }
        match ball_information.state {
            BallState::Stuck => {
                ball_information.position = Vector2 { x:player.x + (player.width / 2.0), y: player.y - GAME_SETTINGS.ball_radius };

                if game.is_key_down(KeyboardKey::KEY_SPACE) {
                    ball_information.state = BallState::Free;
                    ball_information.last_position = Vector2 {
                        x: ball_information.position.x,
                        y: ball_information.position.y + GAME_SETTINGS.ball_velocity,
                    };
                }
            },
            BallState::Free => {
                let last_known_position = ball_information.position;
                ball_information.position = handle_ball_free(
                    ball_information.position,
                    ball_information.last_position,
                    vec![top_line, bottom_line, left_line, right_line, player],
                );
                ball_information.last_position = last_known_position;
            },
        }

        let mut drawer = game.begin_drawing(&thread);
        {
            drawer.clear_background(Color::WHITE);
            {
                let mut drawer = drawer.begin_mode2D(camera);
                drawer.draw_rectangle_rec(player, Color::RED);

                for row in pills.iter() {
                    for pill in row.iter() {
                        drawer.draw_rectangle(
                            pill.x as i32,
                            pill.y as i32,
                            GAME_SETTINGS.pill_width as i32,
                            GAME_SETTINGS.pill_height as i32,
                            pill.variant.to_color()
                        );
                    }
                }

                drawer.draw_circle_v(
                    ball_information.position,
                    GAME_SETTINGS.ball_radius,
                    Color::TURQUOISE
                );
            }
        }
    }
}
