use hive_lib::{piece::Piece, position::Position, color::Color};
use crate::common::{svg_pos::SvgPos, piece_type::PieceType};
use leptos::*;

#[component]
pub fn Piece(cx: Scope, piece: Piece, position: Position, level: usize, #[prop(optional)] piece_type: PieceType) -> impl IntoView {
    let svg_pos = SvgPos::new(position.q, position.r);
    let center = svg_pos.center_from_level(level);
    let transform = format!("translate({},{})", center.0, center.1);

    let mut filter = String::from("filter: drop-shadow(0.3px 0.3px 0.3px");
    if piece.color() == Color::White {
        filter.push_str(" #000)");
    } else {
        filter.push_str(" #FFF)");
    }
    //let mut filter = String::from("filter: drop-shadow(0.3px 0.3px 0.3px #000)");
    if piece_type == PieceType::Inactive {
        filter.push_str(" sepia(1)");
    }
    let color = piece.color().to_string();
    let bug = piece.bug().to_string();
    // let order = piece.order().to_string();
    view! { cx,
        <g class="piece" style={filter}>
            <g id="Ant" transform=format!("{}", transform)>
                <use_ href=format!("#{}", color) transform="scale(0.56, 0.56) translate(-45, -50)" />
                <use_ href=format!("#{}", bug) transform="scale(0.56, 0.56) translate(-50, -45)"/>
                // <use_ href=format!("#{}", order) transform="scale(0.56, 0.56) translate(-50, -45)"/>
            </g>
        </g>
    }
}

