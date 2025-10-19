use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];

    let mut turn_count = 0;
    let mut current_player = 'X';

    loop {
        view_board(&board);
        println!("Vez do jogador '{}'. Escolha uma posição (1 a 9):", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

        let index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida, digite um número de 1 a 9!");
                continue;
            }
        };

        if let Some((row, col)) = index_to_coord(index) {
            if board[row][col] == ' ' {
                board[row][col] = current_player;

                turn_count += 1;

                if turn_count > 4 {
                    if check_winner(&board, current_player) {
                        view_board(&board);
                        println!("O Jogador '{}' venceu!", current_player);
                        break;
                    }
                }

                if turn_count == 9 {
                    view_board(&board);
                    println!("O jogo empatou!");
                    break;
                }

                current_player = if current_player == 'X' { 'O' } else { 'X' };
            } else {
                println!("Essa posição já está ocupada!");
            }
        } else {
            println!("Escolha um número entre 1 e 9!");
        }
    }
}

fn view_board(board: &[[char; 3]; 3]) {
    println!();
    for i in 0..3 {
        println!(" {} | {} | {} ", board[i][0], board[i][1], board[i][2]);
        if i < 2 {
            println!("-----------");
        }
    }
    println!();
}

fn index_to_coord(index: usize) -> Option<(usize, usize)> {
    match index {
        1 => Some((0, 0)),
        2 => Some((0, 1)),
        3 => Some((0, 2)),
        4 => Some((1, 0)),
        5 => Some((1, 1)),
        6 => Some((1, 2)),
        7 => Some((2, 0)),
        8 => Some((2, 1)),
        9 => Some((2, 2)),
        _ => None,
    }
}

fn check_winner(board: &[[char; 3]; 3], player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }

    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}
