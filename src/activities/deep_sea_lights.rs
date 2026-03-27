use leptos::prelude::*;
use rand::seq::SliceRandom;
use gloo_timers::callback::Timeout;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;
use crate::creatures::AnglerFishSvg;

const CREATURES: [(&str, &str); 5] = [
    ("Jellyfish", "#00FFFF"),
    ("Anglerfish", "#ADFF2F"),
    ("Sea Fireflies", "#FF69B4"),
    ("Comb Jelly", "#BA68C8"),
    ("Lanternfish", "#FFA726"),
];

#[component]
pub fn DeepSeaLightsActivity() -> impl IntoView {
    view! {
        <ActivityWrapper id=ActivityId::DeepSeaLights>
            <LightsGame />
        </ActivityWrapper>
    }
}

#[component]
fn LightsGame() -> impl IntoView {
    let complete = expect_context::<ActivityCompleteSignal>();
    let round = RwSignal::new(1_usize);
    let sequence = RwSignal::new(Vec::<usize>::new());
    let player_input = RwSignal::new(Vec::<usize>::new());
    let active_light = RwSignal::new(Option::<usize>::None);
    let phase = RwSignal::new(Phase::Waiting);
    let message = RwSignal::new(String::from("Watch the lights carefully!"));
    let permanently_lit = RwSignal::new(vec![false; 5]);

    let generate_sequence = move |len: usize| {
        let mut rng = rand::thread_rng();
        let mut seq: Vec<usize> = (0..5).collect();
        seq.shuffle(&mut rng);
        seq.truncate(len);
        seq
    };

    let start_round = move || {
        let len = match round.get() {
            1 | 2 => 3,
            3 | 4 => 4,
            _     => 5,
        };
        let seq = generate_sequence(len);
        sequence.set(seq);
        player_input.set(Vec::new());
        permanently_lit.set(vec![false; 5]);
        active_light.set(None);
        phase.set(Phase::Showing);
        message.set("Watch...".into());
        play_sequence(sequence, active_light, phase, message);
    };

    let on_creature_click = move |idx: usize| {
        if phase.get() != Phase::Input { return; }

        active_light.set(Some(idx));
        let timeout = Timeout::new(300, move || active_light.set(None));
        timeout.forget();

        player_input.update(|p| p.push(idx));
        let input = player_input.get();
        let seq = sequence.get();

        let pos = input.len() - 1;
        if input[pos] != seq[pos] {
            message.set("Not quite! The blowfish says try again!".into());
            phase.set(Phase::Waiting);
            player_input.set(Vec::new());
        } else if input.len() == seq.len() {
            permanently_lit.update(|lit| {
                for &idx in &seq {
                    lit[idx] = true;
                }
            });

            let r = round.get();
            if r >= 5 {
                message.set("The deep sea is alive with light!".into());
                permanently_lit.set(vec![true; 5]);
                complete.trigger();
            } else {
                round.set(r + 1);
                message.set(format!("Round {} complete! Ready for the next?", r));
                phase.set(Phase::Waiting);
            }
        }
    };

    view! {
        <div class="deep-blob">
            <svg viewBox="0 0 500 400" xmlns="http://www.w3.org/2000/svg">
                // main body
                <ellipse cx="250" cy="325" rx="235" ry="190" fill="#030C12"/>
                <ellipse cx="250" cy="315" rx="205" ry="165" fill="#050F18" opacity="0.7"/>
                // side fins
                <path d="M20 285 Q-15 240 12 305 Q32 355 72 305 Z" fill="#030C12"/>
                <path d="M480 285 Q515 240 488 305 Q468 355 428 305 Z" fill="#030C12"/>
                // small dorsal lumps
                <ellipse cx="175" cy="168" rx="18" ry="10" fill="#030C12"/>
                <ellipse cx="250" cy="155" rx="22" ry="12" fill="#030C12"/>
                <ellipse cx="325" cy="168" rx="18" ry="10" fill="#030C12"/>
                // eye sockets
                <circle cx="170" cy="215" r="38" fill="#000A00"/>
                <circle cx="330" cy="215" r="38" fill="#000A00"/>
                // iris
                <circle cx="170" cy="215" r="27" fill="#00BB33"/>
                <circle cx="330" cy="215" r="27" fill="#00BB33"/>
                // pupil
                <circle cx="170" cy="215" r="13" fill="#001200"/>
                <circle cx="330" cy="215" r="13" fill="#001200"/>
                // specular highlight
                <circle cx="160" cy="205" r="7" fill="#88FFAA" opacity="0.7"/>
                <circle cx="320" cy="205" r="7" fill="#88FFAA" opacity="0.7"/>
                // outer glow rings
                <circle cx="170" cy="215" r="50" fill="#00FF55" opacity="0.05"/>
                <circle cx="330" cy="215" r="50" fill="#00FF55" opacity="0.05"/>
                // creepy mouth
                <path d="M165 278 Q250 308 335 278" fill="none" stroke="#030C12" stroke-width="5" stroke-linecap="round"/>
                // faint gill lines
                <path d="M82 260 Q90 240 98 260 Q106 280 114 260" fill="none" stroke="#060F18" stroke-width="3" opacity="0.6"/>
                <path d="M386 260 Q394 240 402 260 Q410 280 418 260" fill="none" stroke="#060F18" stroke-width="3" opacity="0.6"/>
            </svg>
        </div>

        <div class="deep-sea-lights">
            <div class="lights-speech">
                <div class="speech-bubble blowfish-speech">
                    <span class="speech-creature"><AnglerFishSvg size=52 /></span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="lights-round">
                {move || format!("Round {} / 5", round.get())}
            </div>

            <div class="lights-arena">
                {CREATURES.iter().enumerate().map(|(i, (name, color))| {
                    let color = *color;
                    let name = *name;
                    let is_lit = move || {
                        active_light.get() == Some(i) || permanently_lit.get()[i]
                    };
                    let class = move || {
                        if is_lit() { "light-creature lit" } else { "light-creature" }
                    };

                    view! {
                        <div class="creature-slot">
                            <button
                                class=class
                                style=format!("--glow-color: {};", color)
                                on:click=move |_| on_creature_click(i)
                            >
                                <div class="creature-glow"></div>
                            </button>
                            <span class="creature-name">{name}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <Show when=move || phase.get() == Phase::Input>
                <div class="lights-your-turn">"✨ Your turn! ✨"</div>
            </Show>

            <Show when=move || phase.get() == Phase::Waiting>
                <button class="btn-primary lights-start-btn" on:click=move |_| start_round()>
                    {move || if round.get() == 1 { "Start" } else { "Next Round" }}
                </button>
            </Show>
        </div>
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Waiting,
    Showing,
    Input,
}

fn play_sequence(
    sequence: RwSignal<Vec<usize>>,
    active_light: RwSignal<Option<usize>>,
    phase: RwSignal<Phase>,
    message: RwSignal<String>,
) {
    let seq = sequence.get();
    let total = seq.len();

    for (i, &idx) in seq.iter().enumerate() {
        let delay_on = (i as u32) * 1000 + 500;
        let delay_off = delay_on + 600;

        let timeout_on = Timeout::new(delay_on, move || {
            active_light.set(Some(idx));
        });
        timeout_on.forget();

        let timeout_off = Timeout::new(delay_off, move || {
            active_light.set(None);
        });
        timeout_off.forget();
    }

    let finish_delay = (total as u32) * 1000 + 800;
    let timeout_done = Timeout::new(finish_delay, move || {
        phase.set(Phase::Input);
        message.set("Your turn! Repeat the sequence!".into());
    });
    timeout_done.forget();
}
