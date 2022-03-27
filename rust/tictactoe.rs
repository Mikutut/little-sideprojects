use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}
fn switch_player(player: &mut i32) {
    if *player == 1 {
        *player = 2;
    } else {
        *player = 1;
    }
}
fn check_board(board: &[[i32; 3]; 3]) -> i32 {
    //check for rows
    for i in 0..2 {
        if (board[i][0] == board[i][1]) && (board[i][0] == board[i][2]) && (board[i][1] == board[i][2]) {
            if board[i][0] != 0 {
                return board[i][0];
            }
        }
    }

    //check for columns
    for i in 0..2 {
        if (board[0][i] == board[1][i]) && (board[0][i] == board[2][i]) && (board[1][i] == board[2][i]) {
            if board[0][i] != 0 {
                return board[0][i];
            }
        }
    }

    //check for diagonals
    if (board[0][0] == board[1][1]) && (board[0][0] == board[2][2]) && (board[1][1] == board[2][2]) {
        if board[0][0] != 0 {
            return board[0][0];
        }
    } else if (board[0][2] == board[1][1]) && (board[0][2] == board[2][0]) && (board[1][1] == board[2][0]) {
        if board[0][2] != 0 {
            return board[0][2];
        }
    }

    return 0;
}
fn occupy_field(board: &mut [[i32; 3]; 3], row: usize, column: usize, current_player: &i32) -> bool {
    if board[row][column] != 0 {
        false
    } else {
        board[row][column] = *current_player;
        true
    }
}
fn print_board(board: &[[i32; 3]; 3], specified_player: Option<i32>) {
    if let None = specified_player {
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    print!(".");
                } else if board[i][j] == 1 {
                    print!("O");
                } else if board[i][j] == 2 {
                    print!("X");
                } else {
                    print!("?");
                }
                
                if j != 2 {
                    print!("\t");
                } else { 
                    print!("\n");
                }
            }
        }
    } else if let Some(k) = specified_player {
        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == k {
                    print!("{}", if k == 1 { "O" } else { "X" });
                } else {
                    print!(".");
                }

                if j != 2 {
                    print!("\t");
                } else {
                    print!("\n");
                }
            }
        }
    }
}

fn main() {
    let mut win_condition_met = false;
    let mut board: [[i32; 3]; 3] = [[0; 3]; 3];
    let mut current_player: i32 = 1;
    let mut round_count = 1;

    while !win_condition_met {
        clear_console();
        println!("Runda nr {}", round_count);
        println!("Tura należy do: {}", if current_player == 1 { "O" } else { "X" });
        print_board(&mut board, None);
        sleep(Duration::from_secs(1));
        print!("Podaj pole, które chcesz zająć (format: rząd kolumna): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_split = input.split_whitespace();
        let coords: Vec<i32> = input_split
            .map(|x| x.to_string()
                .parse::<i32>()
                .unwrap()
            ).collect();
        if occupy_field(&mut board, (coords[0] - 1) as usize, (coords[1] - 1) as usize, &current_player) {
            println!("Pole {}:{} zostało zajęte przez {}!", coords[0], coords[1], if current_player == 1 { "O" } else { "X" });
            sleep(Duration::from_secs(1));
            let board_status = check_board(&board);

            if board_status != 0 {
                clear_console();
                println!("KONIEC GRY!");
                println!("Wygrał {} w {} rund!", if current_player == 1 { "O" } else { "X" }, round_count);
                print_board(&board, Some(current_player));
                sleep(Duration::from_secs(3));
                win_condition_met = true;
            } else {
                round_count += 1;
                switch_player(&mut current_player);
            }
        } else {
            println!("To pole jest już zajęte! Wybierz inne pole!");
            sleep(Duration::from_secs(1));
        }
    }
}
