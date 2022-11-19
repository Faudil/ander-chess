extern crate pleco;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;


use std::{thread, time};

use std::cmp::{max, min};
use std::io::{stdin, stdout, Write};
use std::ops::Index;
use std::thread::scope;
use pleco::board::Board;
use pleco::core::piece_move::BitMove;
use pleco::{PieceType, Player};
use rand::seq::SliceRandom; // 0.7.2

fn count_pieces_sore(board: &Board, player: Player) -> i8 {
    let mut score = 0;
    score += board.count_piece(player, PieceType::P) * 1;
    score += board.count_piece(player, PieceType::B) * 3;
    score += board.count_piece(player, PieceType::N) * 3;
    score += board.count_piece(player, PieceType::R) * 5;
    score += board.count_piece(player, PieceType::Q) * 9;
    return score as i8;
}

fn count_score(board: &Board, player: Player) -> i8 {
    let score = count_pieces_sore(board, player);
    return score - count_pieces_sore(board, player.other_player());
}

fn compute_move(mut board: Board, player: Player, chess_move: BitMove) -> i8 {
    board.apply_move(chess_move);
    let score = -negamax(&mut board, 3, -120, 120, player.other_player());
    board.undo_move();
    score
}


fn process(mut board: Board, depth: i8, player: Player) -> BitMove {
    let moves = board.generate_moves();
    let scores: Vec<i8> = moves.par_iter().map(|chess_move| compute_move(board.clone(), player, *chess_move)).collect();
    let max_value = scores.iter().max().unwrap();
    let max_idx = scores.iter().position(|x| x == max_value).unwrap();
    for i in 0..moves.len() {
        print!("{}:{} ", moves[i], scores[i])
    }
    println!();
    return moves[max_idx];
}

fn negamax(board: &mut Board, depth: i8, mut alpha: i8, mut beta: i8, player: Player) -> i8 {
    if depth == 0 {
        return if board.turn() == player {
            count_score(board, player)
        } else {
            -count_score(board, player)
        }
    }
    let mut value = -120;
    let moves = board.generate_moves();
    for chess_move in moves {
        board.apply_move(chess_move);
        value = max(value, -negamax(board, depth - 1, -beta, -alpha, player.other_player()));
        alpha = max(alpha, value);
        if alpha >= beta {
            board.undo_move();
            break;
        }
        board.undo_move();
    }
    return value;
}


fn play_game() {
    let mut board = Board::start_pos(); // create a board of the starting position
    let mut s= String::new();
    board.pretty_print();
    while !board.checkmate() || !board.stalemate() {
        let mut applied = false;
        while !applied {
            print!("Please enter your move: ");
            stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            s.pop();
            println!("{}", s.as_str());
            applied = board.apply_uci_move(s.as_str());
            s = String::new();
        }
        // let first_player_move = process(board.clone(), 2, board.turn());
        // board.apply_move(first_player_move);
        board.pretty_print();
        // stdin().read_line(&mut s).expect("Did not enter a correct string");
        let engine_move = process(board.clone(), 2, board.turn());
        board.apply_move(engine_move);
        board.pretty_print();
        println!("{} moves played", board.moves_played())
    }
}


fn main() {
    play_game()
}
