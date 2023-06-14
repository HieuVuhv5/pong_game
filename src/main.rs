/*
   Pong Game - 2D game
   Author: Hieu Vu
   Version: 0.1.0

*/
use nannou::prelude::*;
use rand::Rng;
/*
//Game  setting
 */

/*
const BG_COLOR: &str = "BLACK"; // Black
const BALL_COLOR: &str = "BLUE"; // Blue
const PADD_COLOR: &str ="WHITE"; // white
const OBSTACLE_COLOR: &str ="RED";
const TEXT_COLOR: &str ="GREEN"; // Green
*/
const DIFF_LV_1: u32 = 5;
const DIFF_LV_2: u32 = 10;
const DIFF_LV_3: u32 = 15;
const WD_WIDTH: u32 = 800;
const WD_HEIGHT: u32 = 600;
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 5.0;
const BALL_RADIUS: f32 = 10.0;
const OBSTACLE_WIDTH: f32 = 250.0;
const OBSTACLE_HEIGHT: f32 = 5.0;
const ADDJ: f32 = 140.0;

struct Paddle {
    x: f32,
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    speed: f32,
}

struct Score {
    value: u32,
}
struct Obstacle {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> (Paddle, Obstacle, Ball, Score, Score) {
    // Set window size and title
    // set fixed size.
    app.new_window()
        .size(WD_WIDTH, WD_HEIGHT)
        .title("Pong Game- Hieu Vu")
        .resizable(false)
        .view(view)
        .build()
        .unwrap();
    let paddle = Paddle {
        x: WD_WIDTH as f32 / 3.0,
    };
    let ball = Ball {
        //x: WIDTH as f32 / 2.0,
        //y: HEIGHT as f32 / 2.0,
        x: 20.0,
        y: 100.0,
        dx: 1.0,
        dy: 1.0,
        speed: 2.0,
    };
    // set random display position for Obstacle Object
    // This object have to show up in the living zone of the ball.

    let mut rng = rand::thread_rng();
    let rand_x: f32 = rng.gen_range(-100.0..((WD_WIDTH / 2) as f32) - OBSTACLE_WIDTH);
    let rand_y: f32 = rng.gen_range(OBSTACLE_HEIGHT..((WD_HEIGHT / 2) as f32));

    let obstacle = Obstacle {
        x: rand_x,
        y: rand_y,
        width: OBSTACLE_WIDTH,
        height: OBSTACLE_HEIGHT,
    };

    let score = Score { value: 0 };
    let high_score = Score { value: 0 };
    (paddle, obstacle, ball, score, high_score)
}

fn update(
    app: &App,
    (paddle, obstalec, ball, score, high_score): &mut (Paddle, Obstacle, Ball, Score, Score),
    _update: Update,
) {
    let window_rect = app.window_rect();

    // Move the paddle by left and right key - speed not change
    // It also change the ball direction by the key direction when they are hitting.
    // not change the speed of the ball.
    if app.keys.down.contains(&Key::Left) {
        paddle.x -= PADDLE_SPEED;
        // check if ball is hit as key press.
        // change ball direction
        if ball.y - BALL_RADIUS < (PADDLE_HEIGHT - ADDJ)
            && ball.x > paddle.x - PADDLE_WIDTH / 2.0
            && ball.x < paddle.x + PADDLE_WIDTH / 2.0
        {
            if ball.dx > 0.0 {
                ball.dx = -ball.dx;
            }
        }
    }
    if app.keys.down.contains(&Key::Right) {
        paddle.x += PADDLE_SPEED;
        // check if ball is hit as key press.
        // change ball direction
        if ball.y - BALL_RADIUS < (PADDLE_HEIGHT - ADDJ)
            && ball.x > (paddle.x - PADDLE_WIDTH / 2.0)
            && ball.x < (paddle.x + PADDLE_WIDTH / 2.0)
        {
            if ball.dx < 0.0 {
                ball.dx = -ball.dx;
            }
        }
    }

    // The ball is moving by the direction dx dy and it's speed.
    ball.x += ball.dx * ball.speed;
    ball.y += ball.dy * ball.speed;

    // The ball hit the wall
    if ball.x - BALL_RADIUS < window_rect.left() || ball.x + BALL_RADIUS > window_rect.right() {
        ball.dx = -ball.dx;
    }
    if ball.y - BALL_RADIUS < window_rect.bottom() || ball.y + BALL_RADIUS > window_rect.top() {
        ball.dy = -ball.dy;
    }

    // The BAll hit the paddle
    if ball.y - BALL_RADIUS < (PADDLE_HEIGHT - ADDJ)
        && ball.x > paddle.x - PADDLE_WIDTH / 2.0
        && ball.x < paddle.x + PADDLE_WIDTH / 2.0
    {
        ball.dy = -ball.dy;
        score.value += 1;
    }

    // Speed control by key pressed Up and Down key.
    if app.keys.down.contains(&Key::Up) {
        ball.speed += 0.1;
    }
    if app.keys.down.contains(&Key::Down) {
        ball.speed -= 0.1;
        if ball.speed < 0.1 {
            ball.speed = 0.1;
        }
    }

    // speed adjusting by level difficult  base by current score

    if score.value == DIFF_LV_1 {
        ball.speed = 2.0;
    }
    // At the difficult level 1
    // ball's speed is increasing
    // Obstacle object will appeared
    if score.value >= DIFF_LV_1 {
        // check collision ball vs obstacle .

        // case hit from the bottom

        if ball.y < obstalec.y
            && ball.y > obstalec.y - obstalec.height
            && ball.x > obstalec.x - obstalec.width / 2.0
            && ball.x < obstalec.x + obstalec.width / 2.0
        {
            ball.dy = -ball.dy;
            // score.value += 1;
        }
    }
    if score.value == DIFF_LV_2 {
        ball.speed = 6.0;
    }
    if score.value == DIFF_LV_3 {
        ball.speed += 0.1
    }

    // The paddle moving within the window game size
    paddle.x = paddle.x.min(window_rect.right() - PADDLE_WIDTH / 2.0);
    paddle.x = paddle.x.max(window_rect.left() + PADDLE_WIDTH / 2.0);

    // User was loose - Reset the ball position when missing hit.

    if ball.y - BALL_RADIUS < PADDLE_HEIGHT - ((WD_HEIGHT / 2) as f32) + ADDJ {
        ball.x = 20.0;
        ball.y = 200.0;
        ball.speed = 1.5;
        // Update User's high score
        if high_score.value < score.value {
            high_score.value = score.value;
        }
        // reset current score to 0
        score.value = 0;
    }
}

fn view(
    app: &App,
    (paddle, obstale, ball, score, high_score): &(Paddle, Obstacle, Ball, Score, Score),
    frame: Frame,
) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.rect()
        .x_y(paddle.x, PADDLE_HEIGHT / 2.0 - ADDJ)
        .w_h(PADDLE_WIDTH, 10.0)
        .color(WHITE);

    // Draw the ball
    draw.ellipse()
        .x_y(ball.x, ball.y)
        .w_h(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0)
        .color(BLUE);
    // Draw obstacles object
    if score.value >= DIFF_LV_1 {
        /*
        println!(
            " ball.x {}, ball.y {} vs obstacle.x {} obstacle .y {}",
            ball.x, ball.y, obstale.x, obstale.y
        );
        */

        draw.rect()
            .x_y(obstale.x, obstale.y)
            .w_h(obstale.width, obstale.height)
            .color(RED);
    }
    // Display the highest score
    let high_score_text = format!("Your Highest Score: {}", high_score.value);
    draw.text(&high_score_text)
        .x_y(
            -(WD_WIDTH as f32) / 2.0 + 60.0,
            (WD_HEIGHT as f32) / 2.0 - 40.0,
        )
        .color(GREEN);

    // Display the current score
    let score_text = format!("Your Score: {}", score.value);
    draw.text(&score_text)
        .x_y(
            -(WD_WIDTH as f32) / 2.0 + 40.0,
            (WD_HEIGHT as f32) / 2.0 - 20.0,
        )
        .color(GREEN);

    draw.to_frame(app, &frame).unwrap();
}
