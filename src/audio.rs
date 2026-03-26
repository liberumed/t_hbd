use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlAudioElement;
use crate::state::{ActivityId, AppState, Screen};

const TARGET_VOL: f64 = 0.6;
const FADE_STEPS: u32 = 30;
const FADE_STEP_MS: u32 = 100; // 30 * 100ms = 3s fade

fn track_for_screen(screen: Screen) -> &'static str {
    match screen {
        Screen::Welcome                            => "assets/audio/main_theme.ogg",
        Screen::Hub                                => "assets/audio/main_theme_2.ogg",
        Screen::Activity(ActivityId::PearlWisdom)  => "assets/audio/pearl_wisdom.ogg",
        Screen::Activity(ActivityId::CurrentRider) => "assets/audio/current_rider.ogg",
        Screen::Activity(ActivityId::CoralGarden)  => "assets/audio/coral_garden.ogg",
        Screen::Activity(ActivityId::DeepSeaLights)=> "assets/audio/deep_sea_lights.ogg",
        Screen::Activity(ActivityId::TreasureHunt) => "assets/audio/treasure_hunt.ogg",
        Screen::Finale                             => "assets/audio/finale.ogg",
    }
}

fn fade_in(audio: HtmlAudioElement) {
    spawn_local(async move {
        audio.set_volume(0.0);
        for i in 1..=FADE_STEPS {
            gloo_timers::future::TimeoutFuture::new(FADE_STEP_MS).await;
            let t = i as f64 / FADE_STEPS as f64;
            audio.set_volume(t * t * TARGET_VOL);
        }
    });
}

fn play_with_fade(audio: HtmlAudioElement) {
    use std::rc::Rc;
    use std::cell::Cell;

    audio.set_volume(0.0);

    let fired = Rc::new(Cell::new(false));
    let fired2 = fired.clone();
    let audio_play = audio.clone();
    let audio_fade = audio.clone();

    // Register listener BEFORE load() to avoid race condition
    let cb = Closure::<dyn Fn()>::wrap(Box::new(move || {
        if fired2.get() { return; }
        fired2.set(true);
        let audio = audio_play.clone();
        let audio_f = audio_fade.clone();
        spawn_local(async move {
            let result = JsFuture::from(audio.play().unwrap()).await;
            if result.is_ok() {
                fade_in(audio_f);
            }
        });
    }));
    audio.add_event_listener_with_callback("canplaythrough", cb.as_ref().unchecked_ref()).unwrap();
    cb.forget();

    audio.load(); // trigger buffering after listener is set
}

#[component]
pub fn AudioPlayer() -> impl IntoView {
    let state = expect_context::<AppState>();
    let muted = RwSignal::new(false);
    let autoplay_blocked = RwSignal::new(false);

    let audio = HtmlAudioElement::new().expect("audio element");
    audio.set_loop(true);
    audio.set_volume(0.0);
    audio.set_src(track_for_screen(state.current_screen.get_untracked()));

    // Try autoplay; if blocked, start on first click anywhere
    {
        let audio = audio.clone();
        spawn_local(async move {
            // Probe autoplay with a silent, tiny play attempt
            audio.set_volume(0.0);
            let result = JsFuture::from(audio.play().unwrap()).await;
            if result.is_ok() {
                audio.pause().ok();
                play_with_fade(audio);
            } else {
                autoplay_blocked.set(true);
                let audio2 = audio.clone();
                let document = web_sys::window().unwrap().document().unwrap();
                let cb = Closure::<dyn Fn()>::wrap(Box::new(move || {
                    if autoplay_blocked.get_untracked() {
                        autoplay_blocked.set(false);
                        play_with_fade(audio2.clone());
                    }
                }));
                document.add_event_listener_with_callback_and_bool(
                    "click",
                    cb.as_ref().unchecked_ref(),
                    true,
                ).unwrap();
                cb.forget();
            }
        });
    }

    // Switch track when screen changes
    let generation = RwSignal::new(0u32);
    {
        let audio = audio.clone();
        Effect::new(move |_| {
            let src = track_for_screen(state.current_screen.get());
            if !audio.src().ends_with(src) {
                audio.pause().ok();
                audio.set_src(src);
                generation.update(|g| *g += 1);
                let gen = generation.get_untracked();
                if !muted.get_untracked() {
                    let audio = audio.clone();
                    spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(2000).await;
                        // Only play if no newer switch happened during the wait
                        if generation.get_untracked() == gen {
                            play_with_fade(audio);
                        }
                    });
                }
            }
        });
    }

    let on_click = {
        let audio = audio.clone();
        move |_| {
            if autoplay_blocked.get_untracked() {
                autoplay_blocked.set(false);
                muted.set(false);
                play_with_fade(audio.clone());
            } else {
                muted.update(|m| *m = !*m);
                let now_muted = muted.get_untracked();
                if now_muted {
                    audio.set_muted(true);
                } else {
                    audio.set_muted(false);
                    play_with_fade(audio.clone());
                }
            }
        }
    };

    view! {
        <button class="mute-btn" on:click=on_click title="Toggle music">
            {move || if muted.get() { "🔇" } else { "🔊" }}
        </button>
    }
}
