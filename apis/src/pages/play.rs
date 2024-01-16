use crate::{
    components::{
        atoms::history_button::{HistoryButton, HistoryNavigation},
        molecules::{control_buttons::ControlButtons, user_with_rating::UserWithRating},
        organisms::{
            board::Board,
            display_timer::{DisplayTimer, Placement},
            reserve::{Alignment, Reserve},
            side_board::SideboardTabs,
        },
    },
    providers::{auth_context::AuthContext, game_state::GameStateSignal},
};
use hive_lib::{color::Color, position::Position};
use leptos::*;
use leptos_use::use_media_query;

#[derive(Clone)]
pub struct TargetStack(pub RwSignal<Option<Position>>);

#[component]
pub fn Play(#[prop(optional)] extend_tw_classes: &'static str) -> impl IntoView {
    provide_context(TargetStack(RwSignal::new(None)));
    let auth_context = expect_context::<AuthContext>();
    let user = move || match (auth_context.user)() {
        Some(Ok(Some(user))) => Some(user),
        _ => None,
    };

    let is_tall = use_media_query("(min-height: 100vw)");
    let nav_buttons_style = "flex place-items-center justify-center hover:bg-green-400 dark:hover:bg-green-500 transform transition-transform duration-300 active:scale-95 my-1 h-6 rounded-md border-cyan-500 border-2 drop-shadow-lg";
    let game_state = expect_context::<GameStateSignal>();
    let parent_container_style = move || {
        if is_tall() {
            "flex flex-col"
        } else {
            "grid grid-cols-board-xs sm:grid-cols-board-sm lg:grid-cols-board-lg xxl:grid-cols-board-xxl grid-rows-6 pr-1"
        }
    };
    let show_buttons = move || {
        user().map_or(false, |user| {
            let game_state = game_state.signal.get();
            Some(user.id) == game_state.black_id || Some(user.id) == game_state.white_id
        })
    };
    let player_is_black = create_memo(move |_| {
        user().map_or(false, |user| {
            let game_state = game_state.signal.get();
            Some(user.id) == game_state.black_id
        })
    });
    let go_to_game = Callback::new(move |()| {
        let mut game_state = expect_context::<GameStateSignal>();
        if game_state.signal.get_untracked().is_last_turn() {
            game_state.view_game();
        }
    });
    let bottom_color = move || {
        if player_is_black() {
            Color::Black
        } else {
            Color::White
        }
    };
    let top_color = move || bottom_color().opposite_color();

    view! {
        <div class=move || {
            format!(
                "max-h-[100dvh] min-h-[100dvh] pt-10 {} {extend_tw_classes}",
                parent_container_style(),
            )
        }>
            <Show
                when=is_tall
                fallback=move || {
                    view! {
                        <Board/>
                        <div class="grid col-start-9 col-span-2 row-span-full grid-cols-2 grid-rows-6">
                            <DisplayTimer placement=Placement::Top vertical=false/>
                            <SideboardTabs player_is_black=player_is_black/>
                            <DisplayTimer placement=Placement::Bottom vertical=false/>
                        </div>
                    }
                }
            >

                <div class="flex flex-col flex-grow h-full min-h-0">
                    <div class="flex flex-col shrink flex-grow">
                        <div class="flex justify-between shrink">
                            <Show when=show_buttons>
                                <ControlButtons/>
                            </Show>
                        </div>
                        <div class="flex max-h-16 justify-between h-full">
                            <Reserve alignment=Alignment::SingleRow color=top_color()/>
                            <DisplayTimer vertical=true placement=Placement::Top/>
                        </div>
                        <div class="ml-2 flex gap-1">
                            <UserWithRating side=top_color()/>
                        </div>

                    </div>
                    <Board overwrite_tw_classes="flex grow min-h-0"/>
                    <div class="flex flex-col shrink flex-grow">
                        <div class="ml-2 flex gap-1">
                            <UserWithRating side=bottom_color()/>
                        </div>
                        <div class="flex max-h-16 justify-between h-full">
                            <Reserve alignment=Alignment::SingleRow color=bottom_color()/>
                            <DisplayTimer vertical=true placement=Placement::Bottom/>
                        </div>
                        <div class="grid grid-cols-4 gap-8">
                            <HistoryButton
                                nav_buttons_style=nav_buttons_style
                                action=HistoryNavigation::First
                            />

                            <HistoryButton
                                nav_buttons_style=nav_buttons_style
                                action=HistoryNavigation::Previous
                            />

                            <HistoryButton
                                nav_buttons_style=nav_buttons_style
                                action=HistoryNavigation::Next
                                post_action=go_to_game
                            />

                            <HistoryButton
                                nav_buttons_style=nav_buttons_style
                                action=HistoryNavigation::MobileLast
                            />

                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
