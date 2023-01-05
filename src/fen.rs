use crate::board::Board;
// use crate::board::Square;
use crate::pieces::PieceKind;
use crate::pieces::Position;
use crate::pieces::Piece;
use crate::pieces::Color;

pub fn fen_to_board(notation: &str) -> Board {
    let mut board = Board::new();
    let mut notation_parts = notation.split(" ");

    let pieces_part = notation_parts.nth(0).unwrap();
    let color_part = notation_parts.nth(0).unwrap();
    // let castle_part = notation_parts.nth(0).unwrap();
    // let en_passant_part = notation_parts.nth(0).unwrap();

    _pieces_decode(&mut board, pieces_part);
    _color_decode(&mut board, color_part);
    // _castle_decode(&mut board, castle_part);
    // _en_passant_decode(&mut board, en_passant_part);

    board
}

pub fn board_to_fen(board: &Board) -> String {
    String::from(
        format!("{} {} {} {} {} {}",
            _pieces_encode(board),
            _color_encode(board),
            _castle_encode(board),
            _en_passant_encode(board),
            board.move_counter,
            1,
        ))
}

fn _pieces_encode(board: &Board) -> String {
    let mut notation = String::new();
    let mut counter;

    for i in (0..8).rev() {
        counter = 0;
        for j in 0..8 {
            let pos = Position(j, i);
            let square = board.get_square(pos);
            match square {
                Some(piece) => {
                    if counter > 0 {
                        notation.push_str(&format!("{}", counter));
                        counter = 0;
                    }
                    notation.push(piece_to_fen(&piece));
                },
                None => counter = counter + 1,
            }
        }
        if counter > 0 {
            notation.push_str(&format!("{}", counter));
        }
        notation.push('/');
    }
    notation.pop(); // remove last slash 

    notation
}

fn _pieces_decode(board: &mut Board, notation_part: &str) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for c in notation_part.chars() {
        match c {
            ' ' => break,

            '/' => {
                x = 0;
                y = y + 1;
            },

            '1'..='8' => {
                let int_c: i32 = c as i32 - 0x30;
                x = x + int_c;
            },
            
            _ => {
                let pos = Position(x, 7-y);
                let square = Some(fen_to_piece(c, pos));
                board.set_square(pos, square);
                x = x + 1;
            }
        }
    }
}

fn _color_encode(board: &Board) -> String {
    match board.active_color {
        Color::White => String::from("w"),
        Color::Black => String::from("b"),
    }
}

fn _color_decode(board: &mut Board, notation_part: &str) {
    match notation_part {
        "w" => board.active_color = Color::White,
        "b" => board.active_color = Color::Black,
        _ => panic!("Invalid color")
    }
}

fn _castle_encode(board: &Board) -> String {
    let mut notation = String::new();
    let no_castle = !(board.white_castle.short || board.white_castle.long || board.black_castle.short || board.black_castle.long);

    if no_castle {
        notation.push('-');
        return notation;
    }

    if board.white_castle.short {
        notation.push('K')
    }
    if board.white_castle.long {
        notation.push('Q')
    }
    if board.black_castle.short {
        notation.push('k')
    }
    if board.black_castle.short {
        notation.push('q')
    }

    notation
}

// fn _castle_decode(board: &mut Board, notation_part: &str) {
// }

fn _en_passant_encode(board: &Board) -> String {
    if !board.en_passant {
        return String::from("-");
    }

    if let Some(movement) = board.last_move {
        let indexes = "abcdefgh";
        if let Some(char_index) = indexes.chars().nth(movement.target.0 as usize) {
            return String::from(format!("{}{}", char_index, movement.target.1 + 1));
        }
    }

    String::from("-")
}

// fn _en_passant_decode(board: &mut Board, notation_part: &str) {
// }

fn piece_to_fen(piece: &Piece) -> char {
    match (piece.color, piece.kind) {
        (Color::Black, PieceKind::Rook) => 'r',
        (Color::Black, PieceKind::Knight) => 'n',
        (Color::Black, PieceKind::Bishop) => 'b',
        (Color::Black, PieceKind::Queen) => 'q',
        (Color::Black, PieceKind::King) => 'k',
        (Color::Black, PieceKind::Pawn) => 'p',

        (Color::White, PieceKind::Rook) => 'R',
        (Color::White, PieceKind::Knight) => 'N',
        (Color::White, PieceKind::Bishop) => 'B',
        (Color::White, PieceKind::Queen) => 'Q',
        (Color::White, PieceKind::King) => 'K',
        (Color::White, PieceKind::Pawn) => 'P',
    }    
}

fn fen_to_piece(notation: char, pos: Position) -> Piece {
    match notation {
        'r' => Piece{pos, color:Color::Black, kind:PieceKind::Rook},
        'n' => Piece{pos, color:Color::Black, kind:PieceKind::Knight},
        'b' => Piece{pos, color:Color::Black, kind:PieceKind::Bishop},
        'q' => Piece{pos, color:Color::Black, kind:PieceKind::Queen},
        'k' => Piece{pos, color:Color::Black, kind:PieceKind::King},
        'p' => Piece{pos, color:Color::Black, kind:PieceKind::Pawn},

        'R' => Piece{pos, color:Color::White, kind:PieceKind::Rook},
        'N' => Piece{pos, color:Color::White, kind:PieceKind::Knight},
        'B' => Piece{pos, color:Color::White, kind:PieceKind::Bishop},
        'Q' => Piece{pos, color:Color::White, kind:PieceKind::Queen},
        'K' => Piece{pos, color:Color::White, kind:PieceKind::King},
        'P' => Piece{pos, color:Color::White, kind:PieceKind::Pawn},

        _ => panic!("Unkown fen piece"),
    }
}