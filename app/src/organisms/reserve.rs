use std::str::FromStr;

use crate::common::hex::{Hex, HexType};
use crate::common::hex_stack::HexStack;
use crate::common::piece_type::PieceType;
use crate::molecules::hex_stack::HexStack;
use crate::{atoms::svgs::Svgs, common::game_state::GameStateSignal};
use hive_lib::bug_stack::BugStack;
use hive_lib::{bug::Bug, color::Color, piece::Piece, position::Position, state::State};
use leptos::*;

fn piece_active(state: &State, piece: &Piece) -> bool {
    // #TODO make this come from global state
    if !piece.is_color(state.turn_color) {
        return false;
    };
    // first and second turn
    // -> disable queen
    if piece.bug() == Bug::Queen && state.turn < 2 {
        return false;
    };
    // if queen_required
    // -> disable all but queen
    if state.board.queen_required(state.turn, state.turn_color) && piece.bug() != Bug::Queen {
        return false;
    };
    true
}

#[derive(PartialEq, Eq, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn Reserve(cx: Scope, color: Color, orientation: Orientation) -> impl IntoView {
    let game_state_signal = use_context::<RwSignal<GameStateSignal>>(cx)
        .expect("there to be a `GameState` signal provided");

    let stacked_pieces = move || {
        let game_state = game_state_signal.get().signal.get();
        let reserve = game_state
            .state
            .board
            .reserve(color, game_state.state.game_type);
        let mut clicked_position = None;
        if color == game_state.state.turn_color {
            clicked_position = game_state.reserve_position;
        }
        let mut seen = -1;
        let mut res = Vec::new();
        for bug in Bug::all().into_iter() {
            if let Some(piece_strings) = reserve.get(&bug) {
                seen += 1;
                let position = if orientation == Orientation::Vertical {
                    Position::new(4 - seen / 2, seen)
                } else {
                    Position::new(seen % 4, seen / 4)
                };
                let bs = BugStack::new();
                let mut hs = HexStack::new(&bs, position);
                for (i, piece_str) in piece_strings.iter().rev().enumerate() {
                    let piece = Piece::from_str(piece_str).unwrap();
                    let piece_type = if piece_active(&game_state.state, &piece) {
                        PieceType::Reserve
                    } else {
                        PieceType::Inactive
                    };
                    hs.hexes.push(Hex {
                        kind: HexType::Tile(piece, piece_type),
                        position,
                        level: i,
                    });
                }
                if let Some(click) = clicked_position {
                    if click == position {
                        if game_state.target_position.is_some() {
                            hs.add_active(true);
                        } else {
                            hs.add_active(false);
                        }
                    }
                }
                res.push(hs);
            }
        }
        res
    };

    let pieces_view = move || {
        stacked_pieces()
            .into_iter()
            .map(|hex_stack| {
                view! { cx,
                    <HexStack hex_stack=hex_stack/>
                }
            })
            .collect_view(cx)
    };

    view! { cx,
        <svg viewBox="180 -50 100 1000" class="col-start-1 col-span-1" xmlns="http://www.w3.org/2000/svg">
            <Svgs/>
            { pieces_view }
        </svg>
    }
}
