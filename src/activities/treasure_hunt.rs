use leptos::prelude::*;
use wasm_bindgen::JsCast;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;
use crate::creatures::SebastianSvg;

struct Riddle {
    question: &'static str,
    answers: &'static [&'static str],
    hint: &'static str,
    hint_icon: &'static str,
    digit: char,
}

const RIDDLES: [Riddle; 3] = [
    Riddle {
        question: "I have cities but no houses, forests but no trees, and water but no fish. What am I?",
        answers: &["map", "a map"],
        hint: "Explorers carry me in their pockets...",
        hint_icon: "🗺️",
        digit: '3',
    },
    Riddle {
        question: "The more you take, the more you leave behind. What am I?",
        answers: &["footsteps", "steps", "footstep"],
        hint: "Look down at the sand when you walk...",
        hint_icon: "👣",
        digit: '7',
    },
    Riddle {
        question: "I speak without a mouth and hear without ears. I have no body, but I come alive with the wind. What am I?",
        answers: &["echo", "an echo"],
        hint: "Try shouting into a cave...",
        hint_icon: "🪨",
        digit: '1',
    },
];

#[component]
pub fn TreasureHuntActivity() -> impl IntoView {
    view! {
        <ActivityWrapper id=ActivityId::TreasureHunt>
            <TreasureGame />
        </ActivityWrapper>
    }
}

#[component]
fn TreasureGame() -> impl IntoView {
    let complete = expect_context::<ActivityCompleteSignal>();
    let riddle_index = RwSignal::new(0_usize);
    let input_value = RwSignal::new(String::new());
    let wrong_count = RwSignal::new(0_usize);
    let show_hint = RwSignal::new(false);
    let solved_digits = RwSignal::new(vec![false; 3]);
    let message = RwSignal::new(String::from("Answer my riddles three!"));
    let chest_open = RwSignal::new(false);

    let check_answer = move |_| {
        let idx = riddle_index.get();
        if idx >= RIDDLES.len() { return; }

        let answer = input_value.get().trim().to_lowercase();
        let riddle = &RIDDLES[idx];

        if riddle.answers.contains(&answer.as_str()) {
            solved_digits.update(|d| d[idx] = true);
            input_value.set(String::new());
            wrong_count.set(0);
            show_hint.set(false);

            if idx + 1 >= RIDDLES.len() {
                message.set("All riddles solved! Opening the chest...".into());
                chest_open.set(true);
                complete.trigger();
            } else {
                riddle_index.set(idx + 1);
                message.set("Correct! Next riddle...".into());
            }
        } else {
            wrong_count.update(|c| *c += 1);
            if wrong_count.get() >= 2 {
                show_hint.set(true);
                message.set("Hmm, here's a hint...".into());
            } else {
                message.set("Not quite... try again!".into());
            }
            input_value.set(String::new());
        }
    };

    let on_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        input_value.set(input.value());
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            check_answer(());
        }
    };

    view! {
        <div class="treasure-hunt">
            <div class="treasure-speech">
                <div class="speech-bubble crab-speech">
                    <span class="speech-creature"><SebastianSvg size=52 /></span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="combination-lock">
                {(0..3).map(|i| {
                    let solved = move || solved_digits.get()[i];
                    let digit = RIDDLES[i].digit;
                    view! {
                        <div class=move || if solved() { "lock-digit solved" } else { "lock-digit" }>
                            {move || if solved() { digit.to_string() } else { "?".to_string() }}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <Show when=move || !chest_open.get()>
                <div class="riddle-area">
                    <p class="riddle-question">
                        {move || {
                            let idx = riddle_index.get();
                            if idx < RIDDLES.len() { RIDDLES[idx].question } else { "" }
                        }}
                    </p>

                    <Show when=move || show_hint.get()>
                        <p class="riddle-hint">
                            {move || {
                                let idx = riddle_index.get();
                                if idx < RIDDLES.len() { RIDDLES[idx].hint } else { "" }
                            }}
                        </p>
                    </Show>

                    <div class="riddle-input-row">
                        <input
                            class="riddle-input"
                            type="text"
                            placeholder="Your answer..."
                            prop:value=input_value
                            on:input=on_input
                            on:keydown=on_keydown
                        />
                        <button class="btn-primary riddle-submit" on:click=move |_| check_answer(())>
                            "Submit"
                        </button>
                    </div>
                </div>
            </Show>

            <div class="treasure-area">
                <Show when=move || chest_open.get()>
                    <div class="treasure-pile">
                        {treasure_pile()}
                    </div>
                </Show>
                <div class=move || if chest_open.get() { "treasure-chest open" } else { "treasure-chest" }>
                    <svg viewBox="0 0 120 100" width="480" height="400" xmlns="http://www.w3.org/2000/svg">
                        // chest body
                        <rect x="10" y="45" width="100" height="50" rx="5" fill="var(--sand-dark)"/>
                        <rect x="15" y="50" width="90" height="40" rx="3" fill="var(--sand-mid)"/>
                        // bands
                        <rect x="10" y="55" width="100" height="6" fill="var(--gold)" opacity="0.6"/>
                        <rect x="10" y="75" width="100" height="6" fill="var(--gold)" opacity="0.6"/>
                        // lock
                        <circle cx="60" cy="65" r="8" fill="var(--gold)"/>
                        <rect x="56" y="63" width="8" height="10" rx="2" fill="var(--sand-dark)"/>
                        // lid
                        <g class="chest-lid">
                            <path d="M10 45 Q60 10 110 45 L110 45 L10 45 Z" fill="var(--sand-dark)"/>
                            <path d="M15 43 Q60 15 105 43" fill="none" stroke="var(--gold)"
                                  stroke-width="3" opacity="0.6"/>
                        </g>
                    </svg>
                </div>
            </div>
        </div>
    }
}

fn treasure_pile() -> Vec<impl IntoView> {
    const EMOJI: &[&str] = &[
        "💰", "💎", "💰", "💍", "💎", "💰", "🏅", "💎", "💰", "🚀",
        "💎", "💍", "💰", "💎", "🧸", "💰", "💎", "💍", "💰", "⭐",
        "💰", "🎁", "💎", "💰", "✨", "💍", "💰", "🔑", "💎", "💰",
        "🏆", "💎", "💰", "💍", "💰", "🪄", "💎", "💰", "🎯", "💎",
        "🚀", "💰", "💎", "💍", "💰", "🌙", "💎", "💰", "🌟", "💍",
        "🎀", "💎", "💰", "🎊", "💍", "💰", "🦋", "💎", "💰", "🎆",
        "💰", "💎", "💍", "🍀", "💰", "🎈", "💎", "💰", "🧿", "💍",
        "💎", "💰", "🌈", "💎", "💰", "💍", "🎠", "💰", "💎", "🚂",
        "💰", "💍", "💎", "💰", "🎡", "💰", "💎", "💍", "💰", "🤖",
        "💎", "💰", "🍭", "💍", "💰", "💎", "🎪", "💰", "💍", "💎",
    ];
    const ROWS: usize = 22;
    let mut items: Vec<(f64, f64, i32, f64, &'static str)> = Vec::new();
    let mut ei = 0usize;

    for row in 1..=ROWS {
        // row 1 = peak at top of div, row ROWS = base at bottom
        let top_pct = (row - 1) as f64 / ROWS as f64 * 95.0;
        // base pops in first, peak last
        let delay = (ROWS - row) as f64 * 0.025;
        // each row's spread contracts toward the peak
        let spread = row as f64 / ROWS as f64 * 88.0;
        let left_offset = (88.0 - spread) / 2.0;
        for pos in 0..row {
            let left_pct = if row == 1 {
                44.0
            } else {
                left_offset + pos as f64 / (row - 1) as f64 * spread
            };
            let rot = ((pos * 7 + row * 3) % 31) as i32 - 15;
            items.push((left_pct, top_pct, rot, delay, EMOJI[ei % EMOJI.len()]));
            ei += 1;
        }
    }

    items.into_iter().map(|(left, top, rot, delay, emoji)| {
        let wrap_style = format!("left:{left:.1}%;top:{top:.1}%;transform:rotate({rot}deg)");
        let item_style = format!("animation-delay:{delay:.3}s");
        view! {
            <div class="treasure-item-wrap" style=wrap_style>
                <span class="treasure-item" style=item_style>{emoji}</span>
            </div>
        }
    }).collect()
}
