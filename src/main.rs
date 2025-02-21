mod ball;
mod player;

use macroquad::prelude::*;

const GAME_SIZE_X: i32 = 800;
const GAME_SIZE_Y: i32 = 480;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fong".to_owned(),
        window_width: GAME_SIZE_X,
        window_height: GAME_SIZE_Y,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
       
    let mut p = player::Player::new();
    let mut b = ball::Ball::new(vec2(
        screen_width() * 0.5f32 - ball::BALL_SIZE * 0.5f32,
        screen_height() * 0.15f32,
    ));

    let texture: Texture2D = load_texture("resources/background.png").await.unwrap();

    loop {
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        resolve_collision(&mut b.rect, &mut b.vel, &p.rect);
        p.update(get_frame_time());
        b.update(get_frame_time());
        p.draw();
        b.draw();
        next_frame().await
    }

    fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
        // early exit
				let intersection = match a.intersect(*b) {
						Some(intersection) => intersection,
						None => return false,
				};
				let a_center = a.point() + a.size() * 0.5f32;
				let b_center = b.point() + b.size() * 0.5f32;
				let to = b_center - a_center;
				let to_signum = to.signum();
				match intersection.w > intersection.h {
						true => {
								// bounce on y
								a.y -= to_signum.y * intersection.h;
								vel.y = -to_signum.y * vel.y.abs();
						}
						false => {
								// bounce on x
								a.x -= to_signum.x * intersection.w;
								vel.x = -to_signum.x * vel.x.abs();
						}
				}
				true         
    }
}

