extern crate pleco;
extern crate rand;
extern crate rayon;

use rayon::prelude::*;


use std::{thread, time};

use std::cmp::{max, min};
use std::io::stdin;
use std::ops::Index;
use std::thread::scope;
use pleco::board::Board;
use pleco::core::piece_move::BitMove;
use pleco::{PieceType, Player};
use rand::seq::SliceRandom; // 0.7.2

fn count_pieces_sore(board: &Board, player: Player) -> i8 {
    let mut score = board.count_piece(player, PieceType::P);
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
    let score = minmax(&mut board, 3, -99, 99, false, player);
    board.undo_move();
    score
}


fn process(mut board: Board, depth: i8, player: Player) -> BitMove {
    let moves = board.generate_moves();
    let scores: Vec<i8> = moves.par_iter().map(|chess_move| compute_move(board.clone(), player, *chess_move)).collect();
    let max_value = scores.iter().max().unwrap();
    let max_idx = scores.iter().position(|x| x == max_value).unwrap();
    return moves[max_idx];
}

fn minmax(board: & mut Board, depth: i8, mut alpha: i8, mut beta: i8, maximising: bool, player: Player) -> i8 {
    if depth == 0 {
        return count_score(board, player);
    }
    let mut value = 0;
    // max
    if maximising {
        value = -99;
        let moves = board.generate_moves();
        for chess_move in moves {
            board.apply_move(chess_move);
            let leaf_value = minmax(board, depth - 1, alpha, beta, !maximising, player);
            if value < leaf_value {
                value = leaf_value;
            }
            if value >= beta {
                board.undo_move();
                break;
            }
            alpha = min(alpha, value);
            board.undo_move();
        }
    } else {
        value = 99;
        let moves = board.generate_moves();
        for chess_move in moves {
            board.apply_move(chess_move);
            let leaf_value = minmax(board, depth - 1, alpha, beta, !maximising, player);
            if value > leaf_value {
                value = leaf_value;
            }
            if value <= alpha {
                board.undo_move();
                break;
            }
            beta = max(beta, value);
            board.undo_move();
        }
    }
    return value;
}


fn play_game() {
    let mut board = Board::start_pos(); // create a board of the starting position
    let mut s= String::new();
    board.pretty_print();
    while !board.checkmate() || !board.stalemate() {
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        s.pop();
        // board.apply_uci_move(s.as_str());
        let first_player_move = process(board.clone(), 2, board.turn());
        board.apply_move(first_player_move);
        board.pretty_print();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let engine_move = process(board.clone(), 2, board.turn());
        board.apply_move(engine_move);
        board.pretty_print();
        println!("{} moves played", board.moves_played())
    }
}


fn main() {
    play_game()
}
