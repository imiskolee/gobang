mod game;

use std::io::{self, Read,BufRead};
use std::str::FromStr;

fn main() {


    let mut board = game::board::Board::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    board.render();

    println!("Please Type: [0-14] [0-14]");

    loop {
        print!("{}[2J", 27 as char);
        let mut buffer = String::new();
        handle.read_line(&mut buffer);
        let mut iter = buffer.split_whitespace();
        let i = iter.next().unwrap().parse::<usize>().unwrap();
        let j =  iter.next().unwrap().parse::<usize>().unwrap();
        if board.down(game::board::GAME_BLACK,i,j) == false {
            println!("Your Cannt Down Here");
            println!("Please Type: [0-14] [0-14]");
            continue
        }

        let (mi,mj,mb) = board.find_next();
        if mb {
            board.down(game::board::GAME_RED,mi,mj);
        }
        board.render();
    }

    println!("Hello, world!");
}
