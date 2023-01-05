use std::cmp;
use crate::board::Board;
use crate::pieces::PieceKind;
use crate::pieces::Piece;
use crate::movements::Movement;

pub fn search(board: &Board, depth: usize) -> Option<Movement> {
    let mut best_movement: Option<Movement> = None;
    let mut best_score = -i32::MAX;

    let avaliable_moves = Movement::avaliable_moves(board);
    let current_score = count_material(board);
    
    for movement in avaliable_moves {
        let current_score = current_score + move_points(&movement);
        let tmp_board = board.copy_and_move(movement.origin, movement.target);
        let tmp_score = -evaluate_recursive(&tmp_board, depth-1, -i32::MAX, i32::MAX, -current_score);

        if best_score < tmp_score {
            best_score = tmp_score;
            best_movement = Some(movement);
        }
    }

    best_movement
}

pub fn evaluate(board: &Board, depth: usize) -> i32 {
    let current_score = count_material(board);
    evaluate_recursive(board, depth, -i32::MAX, i32::MAX, current_score)
}

fn evaluate_static(board: &Board) -> i32 {
    count_material(board)
}

fn evaluate_recursive(board: &Board, depth: usize, alpha: i32, beta: i32, current_score: i32) -> i32 {
    if depth == 0 {
        return current_score;
        // return evaluate_static(board);
    }
    
    let mut alpha = alpha;
    let mut avaliable_moves = Movement::avaliable_moves(board);
    
    avaliable_moves.sort_by_cached_key(|x| -estimate_movement(&x));

    if avaliable_moves.len() == 0 {
        return evaluate_static(board);
    }

    for movement in avaliable_moves {
        let current_score = current_score + move_points(&movement);
        let tmp_board = board.copy_and_move(movement.origin, movement.target);
        let tmp_score = -evaluate_recursive(&tmp_board, depth - 1, -i32::MAX, i32::MAX, -current_score);
        
        if tmp_score >= beta {
            return beta;
        }
        alpha = cmp::max(alpha, tmp_score);
    }

    alpha
}

fn move_points(movement: &Movement) -> i32 {
    let mut score = 0;

    if let Some(captured) = movement.captured {
        score += piece_value(captured);
    }

    if let Some(promotion) = movement.promotion {
        score -= piece_value(PieceKind::Pawn);
        score += piece_value(promotion);
    }

    score
}

fn estimate_movement(movement: &Movement) -> i32 {
    let mut score = 0;

    if let Some(captured) = movement.captured {
        score += piece_value(captured);
    }

    if let Some(promotion) = movement.promotion {
        score += 2 * piece_value(promotion);
    }

    score
}

fn count_material(board: &Board) -> i32 {
    let mut score: i32 = 0;
    
    for piece in board.ocuppied_squares() {            
        if piece.color == board.active_color {
            score = score + piece_value(piece.kind);
        } else {
            score = score - piece_value(piece.kind);
        }
    }

    score
}

fn piece_value(piece_kind: PieceKind) -> i32 {
    match piece_kind {
        PieceKind::Pawn => 100,
        PieceKind::Bishop => 300,
        PieceKind::Knight => 300,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 1_000_000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::Position;

    #[test]
    fn test_obvious() {
        let board = Board::from_fen("4k3/8/5r2/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, 2);
        
        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();
        
        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(5, 5));
    }
    
    #[test]
    fn test_forks() {
        let board = Board::from_fen("4k3/8/4r3/2KN4/8/8/8/8 w - - 0 1");
        let best_move = search(&board, 3);

        if let None = best_move {
            panic!("No moves found");
        }
        let best_move = best_move.unwrap();

        assert_eq!(best_move.origin, Position(3, 4));
        assert_eq!(best_move.target, Position(2, 6));
    }
}