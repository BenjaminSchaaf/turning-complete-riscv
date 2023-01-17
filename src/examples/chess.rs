/*
Use image_to_argb32.py to export an image into res/image.rgb
*/
use crate::peripherals::{Peripherals, DSP as Display};
use crate::utils::input_line;
use crate::keyboard::Keyboard;
use crate::console::Console;
use chess::{Board, Square, Color, ChessMove, BoardStatus};
use arrayvec::{ArrayString, ArrayVec};
use core::fmt::Write;

const SQUARE_SIZE: usize = 7;
const DISPLAY_WIDTH: usize = 96;
const DARK_SQUARE_COLOR: u32 = 0x606064;
const LIGHT_SQUARE_COLOR: u32 = 0xa8a4a8;
const HIGHLIGHT_DARK_SQUARE_COLOR: u32 = 0xb5b54f;
const HIGHLIGHT_LIGHT_SQUARE_COLOR: u32 = 0xd0ce86;
const SPRITES_WIDTH: usize = 7 * 7;
const DARK_PIECE_COLOR: u32 = 0xffffff;
const LIGHT_PIECE_COLOR: u32 = 0x000000;

pub fn run(io: Peripherals) {
    let mut keyboard = Keyboard::new(io.kbd);
    let mut console = Console::new(io.con);
    let mut display = io.dsp;

    let mut board = Board::default();

    for i in 0..64 {
        let square = unsafe { Square::new(i) };
        draw_square(&mut display, &board, square, false);
    }
    display.flush();

    let mut highlighted_squares = ArrayVec::<Square, 3>::new();

    while board.status() == BoardStatus::Ongoing {
        write!(console, "{:?} to play: ", board.side_to_move()).unwrap();
        let mut buf = ArrayString::<32>::new();
        input_line(&mut console, &mut keyboard, &mut buf);

        match ChessMove::from_san(&board, &buf) {
            Ok(chess_move) => {
                if board.legal(chess_move) {
                    board = board.make_move_new(chess_move);

                    for square in &highlighted_squares {
                        draw_square(&mut display, &board, *square, false);
                    }
                    highlighted_squares.clear();

                    draw_square(&mut display, &board, chess_move.get_source(), true);
                    draw_square(&mut display, &board, chess_move.get_dest(), true);
                    highlighted_squares.push(chess_move.get_source());
                    highlighted_squares.push(chess_move.get_dest());
                    display.flush();
                } else {
                    write!(console, "Illegal Move!\n").unwrap();
                }
            },
            Err(error) => {
                write!(console, "Error: {:?}\n", error).unwrap();
            },
        }
    }

    match board.status() {
        BoardStatus::Stalemate => {
            write!(console, "Game ends by stalemate!\n").unwrap();
        },
        BoardStatus::Checkmate => {
            match board.side_to_move() {
                Color::White => write!(console, "Black wins by checkmate!\n"),
                Color::Black => write!(console, "White wins by checkmate!\n"),
            }.unwrap();
        },
        _ => panic!(),
    }
}

fn draw_square(display: &mut Display, board: &Board, square: Square, highlight: bool) {
    let index = square.to_index();
    let y = 7 - index / 8;
    let x = index % 8;

    let background_offset = x + y % 2;
    let background = if background_offset % 2 == 0 {
        if highlight { HIGHLIGHT_LIGHT_SQUARE_COLOR } else { LIGHT_SQUARE_COLOR }
    } else {
        if highlight { HIGHLIGHT_DARK_SQUARE_COLOR } else { DARK_SQUARE_COLOR }
    };

    let sprites = get_piece_sprites();
    let mut sprite_offset = if let Some(piece) = board.piece_on(square) {
        piece.to_index()
    } else {
        6
    } * SQUARE_SIZE;

    let piece_color = if let Some(Color::White) = board.color_on(square) {
        DARK_PIECE_COLOR
    } else {
        LIGHT_PIECE_COLOR
    };

    let mut dsp_offset = y * 7 * DISPLAY_WIDTH + x * SQUARE_SIZE;
    for _ in 0..SQUARE_SIZE {
        let mut j = sprite_offset;
        for i in dsp_offset..dsp_offset + SQUARE_SIZE {
            if sprites[j] > 0 {
                display.write_pixel(i, piece_color);
            } else {
                display.write_pixel(i, background);
            }
            j += 1;
        }
        dsp_offset += DISPLAY_WIDTH;
        sprite_offset += SPRITES_WIDTH;
    }
}

fn get_piece_sprites() -> &'static [u32] {
    static TILESET: [u32; 7*7*7] = unsafe { core::mem::transmute(*include_bytes!("../../res/pieces.rgb")) };
    &TILESET
}
