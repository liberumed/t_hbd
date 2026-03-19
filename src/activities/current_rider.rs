use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;

const TOTAL_STARFISH: usize = 7;

#[component]
pub fn CurrentRiderActivity() -> impl IntoView {
    view! {
        <ActivityWrapper id=ActivityId::CurrentRider>
            <CurrentGame />
        </ActivityWrapper>
    }
}

#[component]
fn CurrentGame() -> impl IntoView {
    let complete = expect_context::<ActivityCompleteSignal>();
    let collected = RwSignal::new(0_usize);
    let player_y = RwSignal::new(50.0_f64);
    let message = RwSignal::new(String::from("Move up and down to collect the starfish!"));

    let starfish_positions = vec![
        (15.0, 30.0), (28.0, 60.0), (40.0, 25.0), (52.0, 70.0),
        (65.0, 40.0), (78.0, 55.0), (90.0, 35.0),
    ];
    let starfish_caught = RwSignal::new(vec![false; TOTAL_STARFISH]);

    let on_mouse_move = move |ev: web_sys::MouseEvent| {
        let target = ev.current_target().unwrap();
        let element: web_sys::HtmlElement = target.unchecked_into();
        let rect = element.get_bounding_client_rect();
        let relative_y = (ev.client_y() as f64 - rect.top()) / rect.height() * 100.0;
        player_y.set(relative_y.clamp(5.0, 95.0));
    };

    let on_touch_move = move |ev: web_sys::TouchEvent| {
        ev.prevent_default();
        if let Some(touch) = ev.touches().get(0) {
            let target = ev.current_target().unwrap();
            let element: web_sys::HtmlElement = target.unchecked_into();
            let rect = element.get_bounding_client_rect();
            let relative_y = (touch.client_y() as f64 - rect.top()) / rect.height() * 100.0;
            player_y.set(relative_y.clamp(5.0, 95.0));
        }
    };

    let check_collection = move |star_idx: usize, star_y: f64| {
        let py = player_y.get();
        if (py - star_y).abs() < 12.0 {
            starfish_caught.update(|sc| {
                if !sc[star_idx] {
                    sc[star_idx] = true;
                    collected.update(|c| *c += 1);
                }
            });

            if collected.get() >= TOTAL_STARFISH {
                message.set("You collected all the starfish!".into());
                complete.trigger();
            } else {
                message.set(format!("Starfish: {} / {}", collected.get(), TOTAL_STARFISH));
            }
        }
    };

    view! {
        <div class="current-rider">
            <div class="current-speech">
                <div class="speech-bubble fish-speech">
                    <span class="speech-creature">"🐟"</span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="current-counter">
                {move || format!("⭐ {} / {}", collected.get(), TOTAL_STARFISH)}
            </div>

            <div class="current-field"
                 on:mousemove=on_mouse_move
                 on:touchmove=on_touch_move
            >
                // Current streams
                <div class="current-stream stream-1"></div>
                <div class="current-stream stream-2"></div>
                <div class="current-stream stream-3"></div>

                // Player fish
                <div class="current-player" style=move || format!("top: {}%;", player_y.get())>
                    <svg viewBox="0 0 50 30" width="50" height="30" xmlns="http://www.w3.org/2000/svg">
                        <ellipse cx="28" cy="15" rx="18" ry="10" fill="var(--fish-primary)"/>
                        <polygon points="10,15 0,8 0,22" fill="var(--fish-secondary)"/>
                        <circle cx="38" cy="12" r="3" fill="var(--ocean-abyss)"/>
                        <circle cx="39" cy="11" r="1.2" fill="white"/>
                    </svg>
                </div>

                // Starfish
                {starfish_positions.clone().into_iter().enumerate().map(|(i, (x, y))| {
                    let caught = move || starfish_caught.get()[i];
                    let style = format!("left: {}%; top: {}%;", x, y);
                    view! {
                        <Show when=move || !caught()>
                            <button
                                class="starfish-item"
                                style=style.clone()
                                on:click=move |_| check_collection(i, y)
                            >
                                "⭐"
                            </button>
                        </Show>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
