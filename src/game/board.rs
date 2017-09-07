use std::ops::{Index,IndexMut};

pub const SIZE:usize = 15;
pub const GAME_RED:i32 = 1;
pub const GAME_BLACK:i32 = -1;

type BoardLayout = [[i32;SIZE];SIZE];


pub struct Board {
    value_board: BoardLayout,
    board: BoardLayout,
} 

impl Board {
    pub fn new() -> Self {
        Board{value_board:get_values(),board:  [[0;SIZE];SIZE]}
    }

    pub fn down(&mut self,player:i32,i:usize,j:usize) -> bool {
        if self.board[i][j] != 0 {
            return false
        }
        self.board[i][j] = player;
        true
    }


    pub fn render(&self) {


        println!("==================================================");
        println!("--------------------- Go! ------------------------");
        println!("current version: 0.0.1");
        println!("==================================================");

        for i in 0..SIZE {
            print!("|");
            for j in 0..SIZE{

                match self.board[i][j] {
                    0 => print!(" "),
                    1 => print!("o"),
                    -1 => print!("x"),
                    _ => print!(" ")
                }
                print!("|");
            }
            print!("\n");
        }
    }

    pub fn find_next(&self) -> (usize,usize,bool) {

        let mut value = 0;
        let mut ret = (0,0,false);
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] != 0 {
                    continue
                }
                let temp = super::rule::position_value(&self,i,j) * self.value_board[i][j];
                if temp > value {
                    value = temp;
                    ret = (i,j,true);
                    println!("{} {} {} {}",i,j,value,self.value_board[i][j]);
                }
            }
        }
        return ret
    }
}

fn get_values() -> BoardLayout {
    let mut layout:BoardLayout = [[0;SIZE];SIZE];
    let center = (SIZE/2) as usize;

    for i in 0..center+1 {
        for j in 0..center+1 {
            let mut layer = i + 1;
            if j < i {
                layer = j + 1;
            }
            layout[i][j] = layer as i32;
            layout[i][(center + (center -j)) as usize] = layer as i32;
            layout[(center + (center - i)) as usize][j] = layer as i32;
            layout[(center + (center -i) as usize)][(center + (center -j) as usize)] = layer as i32;
        }
    }
    layout
}


impl Index<usize> for Board {
    type Output = [i32];
    fn index(&self, idx: usize) -> &[i32] {
        return &self.board[idx]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, idx:usize) -> &'a mut [i32] {
       return &mut self.board[idx]
    }
}

#[test]
fn test_get_values() {
    let ret = get_values();
    println!("{:?}",ret);
}

#[test]
fn test_render_1() {
    let mut board = Board::new();



    board[5][5] = GAME_RED;
    board[5][4] = GAME_BLACK;
    board[5][3] = GAME_RED;
    board[4][5] = GAME_BLACK;
   // board.render();
}