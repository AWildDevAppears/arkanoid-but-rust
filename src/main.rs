use raylib::prelude::*;

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
};


fn pill_x_index_offset() -> f32 {
    let taken_space = GAME_SETTINGS.pill_column_length as f32 * (GAME_SETTINGS.pill_width + GAME_SETTINGS.pill_offset);
    return (GAME_WINDOW.width as f32 - taken_space) / 2.0;
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
    };
    let mut pills: [[BreakablePill; GAME_SETTINGS.pill_column_length]; 6] = [[default_pill; GAME_SETTINGS.pill_column_length]; 6];

    for (row_idx, row) in pills.iter_mut().enumerate() {
        for (col_idx, pill) in row.iter_mut().enumerate() {
            pill.x = (col_idx as f32 * (GAME_SETTINGS.pill_width + GAME_SETTINGS.pill_offset)) + pill_x_index_offset();
            pill.y = row_idx as f32 * (GAME_SETTINGS.pill_height + GAME_SETTINGS.pill_offset);
            pill.variant = GAME_SETTINGS.pill_default_rows[row_idx];
        }
    }

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

            }
        }
    }
}
