use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use rand::Rng;
use rand::seq::SliceRandom;

use merlin::board::{Mino, MINO_LIST};
use merlin::key::Key;
use merlin::game::Game;
use merlin::viewer::Viewer;

fn main() {
    test();
}

fn test() {
    let mut test = Command::new("python")
        .arg("viewer.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let input = test.stdin.as_mut().unwrap();
    let output = test.stdout.as_mut().unwrap();
    let reader = BufReader::new(output);
    let mut game_over = false;
    let mut game = Game::new();
    let mut viewer = Viewer::new();
    let mut rng = rand::thread_rng();
    let mut m: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
    m.shuffle(&mut rng);
    for &i in &m[0..6] {
        game.new_next(MINO_LIST[i]);
    }
    game.board.spawn(game.next.pop_front().unwrap());
    viewer.update(&mut game);
    viewer.write(&game, input);
    for line in reader.lines() {
        let key = Key::new(&line.unwrap());
        if key == Key::Exit { break; }
        if key == Key::Retry {
            game_over = false;
            game = Game::new();
            viewer = Viewer::new();
            let mut m: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
            m.shuffle(&mut rng);
            for &i in &m[0..6] {
                game.new_next(MINO_LIST[i]);
            }
            game.board.spawn(game.next.pop_front().unwrap());
        }
        if game_over { continue; }
        match key {
            Key::Left => game.move_left(),
            Key::Right => game.move_right(),
            Key::Clockwise => game.rotate_clockwise(),
            Key::Counterclockwise => game.rotate_counterclockwise(),
            Key::SoftDrop => game.soft_drop(),
            Key::HardDrop => {
                let grounded = game.hard_drop();
                let mut r = rng.gen_range(0..7);
                while !game.bag[r] {
                    r = rng.gen_range(0..7);
                }
                game.new_next(MINO_LIST[r]);
                viewer.ground(grounded);
                viewer.clear_lines();
                if game.game_over(grounded) { game_over = true; }
            },
            Key::Hold => {
                if game.hold == Mino::None {
                    game.hold();
                    let mut r = rng.gen_range(0..7);
                    while !game.bag[r] {
                        r = rng.gen_range(0..7);
                    }
                    game.new_next(MINO_LIST[r]);
                } else {
                    game.hold();
                }
            },
            _ => {},
        }
        viewer.update(&mut game);
        viewer.write(&game, input);
    }
    test.kill().unwrap();
}
