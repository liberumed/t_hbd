use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;
use crate::creatures::ClownFishSvg;

const TOTAL_STARFISH: usize = 10;
const TICK_MS: u32 = 33; // ~30fps
const SPEED: f64 = 0.3;
const SPEED_BOOST: f64 = 0.75;

// (x%, y%) positions of starfish in the field
const STARFISH_POS: [(f64, f64); TOTAL_STARFISH] = [
    (12.0, 30.0),
    (22.0, 68.0),
    (33.0, 18.0),
    (43.0, 75.0),
    (52.0, 40.0),
    (62.0, 22.0),
    (70.0, 60.0),
    (78.0, 35.0),
    (86.0, 72.0),
    (93.0, 20.0),
];

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
    let fish_x = RwSignal::new(5.0_f64);
    let fish_y = RwSignal::new(50.0_f64);
    let game_done = RwSignal::new(false);
    let boosted = RwSignal::new(false);
    let message = RwSignal::new("Move up and down — ride the current!".to_string());

    let starfish_caught: Vec<RwSignal<bool>> = (0..TOTAL_STARFISH)
        .map(|_| RwSignal::new(false))
        .collect();

    // Game loop: advance fish_x, check collisions
    let caught_clone = starfish_caught.clone();
    let closure = Closure::wrap(Box::new(move || {
        let speed = if boosted.get() { SPEED_BOOST } else { SPEED };
        let new_x = fish_x.get() + speed;
        fish_x.set(new_x);

        let fy = fish_y.get();

        for (i, (sx, sy)) in STARFISH_POS.iter().enumerate() {
            if caught_clone[i].get() {
                continue;
            }
            if (new_x - sx).abs() < 7.0 && (fy - sy).abs() < 13.0 {
                caught_clone[i].set(true);
                collected.update(|c| *c += 1);
                let cnt = collected.get();
                if cnt >= TOTAL_STARFISH {
                    message.set("You collected them all! Amazing!".into());
                    game_done.set(true);
                    complete.trigger();
                } else {
                    message.set(format!("{} more to go — keep swimming!", TOTAL_STARFISH - cnt));
                }
            }
        }

        // Fish reached the end — loop back to start
        if new_x > 105.0 {
            fish_x.set(0.0);
            let cnt = collected.get();
            if cnt < TOTAL_STARFISH {
                message.set(format!("Keep going! {} left!", TOTAL_STARFISH - cnt));
            }
        }
    }) as Box<dyn Fn()>);

    let window = web_sys::window().unwrap();
    let interval_id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            TICK_MS as i32,
        )
        .unwrap();

    // Keep closure alive and cancel interval on unmount
    closure.forget();
    let interval_id_copy = interval_id;
    on_cleanup(move || {
        if let Some(win) = web_sys::window() {
            win.clear_interval_with_handle(interval_id_copy);
        }
    });

    let on_mouse_move = move |ev: web_sys::MouseEvent| {
        let target = ev.current_target().unwrap();
        let element: web_sys::HtmlElement = target.unchecked_into();
        let rect = element.get_bounding_client_rect();
        let y = (ev.client_y() as f64 - rect.top()) / rect.height() * 100.0;
        fish_y.set(y.clamp(5.0, 95.0));
    };

    let on_touch_move = move |ev: web_sys::TouchEvent| {
        ev.prevent_default();
        if let Some(touch) = ev.touches().get(0) {
            let target = ev.current_target().unwrap();
            let element: web_sys::HtmlElement = target.unchecked_into();
            let rect = element.get_bounding_client_rect();
            let y = (touch.client_y() as f64 - rect.top()) / rect.height() * 100.0;
            fish_y.set(y.clamp(5.0, 95.0));
        }
    };

    view! {
        <div class="current-rider">
            <div class="current-speech">
                <div class="speech-bubble fish-speech">
                    <span class="speech-creature"><ClownFishSvg size=48 /></span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="current-counter">
                {move || format!("⭐ {} / {}", collected.get(), TOTAL_STARFISH)}
            </div>

            <div class="current-field"
                on:mousemove=on_mouse_move
                on:touchmove=on_touch_move
                on:mousedown=move |_| boosted.set(true)
                on:mouseup=move |_| boosted.set(false)
                on:mouseleave=move |_| boosted.set(false)
                on:touchstart=move |_| boosted.set(true)
                on:touchend=move |_| boosted.set(false)
            >
                <div class="current-stream stream-1"></div>
                <div class="current-stream stream-2"></div>
                <div class="current-stream stream-3"></div>

                // Player fish — moves right automatically, Y follows mouse
                <div class="current-player"
                    style=move || format!("left: {}%; top: {}%;", fish_x.get(), fish_y.get())
                >
                    <ClownFishSvg size=64 />
                </div>

                // Starfish
                {starfish_caught.into_iter().enumerate().map(|(i, caught)| {
                    let (sx, sy) = STARFISH_POS[i];
                    let style = format!("left: {}%; top: {}%;", sx, sy);
                    view! {
                        <Show when=move || !caught.get()>
                            <div class="starfish-item" style=style.clone()>"⭐"</div>
                        </Show>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
