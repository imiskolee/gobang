use super::board::{Board,SIZE,GAME_BLACK,GAME_RED};

pub fn position_value(board:&Board,i:usize,j:usize) -> i32 {

    let mut v:i32 = 1;
    println!("\n\n postion:{} {} \n\n",i,j);
    for m in 0..4 {
        let mut value = 1;
        let n = 5 - m;
        println!("n:{}",n);
        if i as i32 - n as i32 > 0 {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i-k][j] == board[i-k-1][j] && board[i-k][j] != 0 {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 * n as i32);
                println!("1 {}",value);
            }
        }

        if i+n < SIZE {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i+k][j] == board[i+k+1][j] && board[i+k][j] != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("2 {}",value);
            }
        }

        if j as i32 - n as i32 > 0 {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i][j-k] == board[i][j-k-1] && board[i][j-k] != 0  {
                    valid =true;
                    break
                }
            }
            if valid {
                value += (20 * n as i32);
                println!("4 {}",value);
            }
        }

        if j+n < SIZE {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i][j+k] == board[i][j+k+1] && board[i][j+k] != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("5 {}",value);
            }
        }

        if i as i32 - n as i32 > 0 && j as i32 - n as i32 > 0 {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i-k][j-k] == board[i-k-1][j-k-1] && board[i-k][j-k] != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("6 {}",value);
            }
        }

        if i + n < SIZE && j + n < SIZE {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i+k][j+k] == board[i+k+1][j+k+1] && board[i+k][j+k] != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("7 {}",value);
            }
        }


        if (i as i32 - n as i32) > 0 && j + n < SIZE {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i-k][j+k] == board[i-k-1][j+k+1] && board[i-k][j+k]  != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("8 {}",value);
            }
        }


        if i + n < SIZE && (j as i32 - n as i32) > 0 {
            let mut valid = false ;
            for k in 1..(n-1) {
                if board[i+k][j-k] == board[i+k+1][j-k-1] && board[i+k][j-k] != 0  {
                    valid = true;
                    break
                }
            }
            if valid {
                value += (20 *n as i32);
                println!("9 {}",value);
            }
        }
        if value > v {
            v = value;
        }
    }
    return v
}



#[test]
fn test_three_value() {

    let mut board = Board::new();
    board[5][5] = GAME_RED;
    board[5][4] = GAME_RED;
    board[5][3] = GAME_RED;

    print!("value {}",position_value(&board,5,6));
    print!("value {}",position_value(&board,5,6));

}


