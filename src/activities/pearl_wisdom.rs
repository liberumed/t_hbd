use leptos::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;
use crate::creatures::ClamSvg;

const DECOYS: &str = "XZQWKJFGHPNM";

struct PearlWord {
    word: &'static str,
    riddle: &'static str,
    hint_svg: &'static str,
}

const WORDS: [PearlWord; 3] = [
    PearlWord {
        word: "DREAM",
        riddle: "It comes when you sleep and flies when you wake.",
        hint_svg: r##"<svg viewBox="0 0 80 80" width="80" height="80" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="80" height="80" rx="16" fill="rgba(13,43,69,0.6)"/><path d="M40 20 Q32 20 32 28 Q32 36 40 36 Q48 36 48 28 Q48 20 40 20 Z" fill="#FFE0B2"/><path d="M42 20 Q50 22 50 28 Q50 34 44 36" fill="rgba(13,43,69,0.3)"/><circle cx="22" cy="22" r="3" fill="#FFD93D"/><circle cx="60" cy="18" r="2" fill="#FFD93D"/><circle cx="55" cy="32" r="2.5" fill="#FFD93D"/><circle cx="25" cy="38" r="1.5" fill="#FFD93D"/><path d="M20 18 L24 22 L20 26" fill="none" stroke="#FFD93D" stroke-width="0.8" opacity="0.5"/><path d="M58 14 L62 18 L58 22" fill="none" stroke="#FFD93D" stroke-width="0.8" opacity="0.5"/><text x="40" y="58" text-anchor="middle" fill="#F5F0FF" font-size="14" font-weight="bold" font-family="sans-serif">Z z z</text><circle cx="30" cy="66" r="2" fill="#F5F0FF" opacity="0.4"/><circle cx="24" cy="70" r="1.5" fill="#F5F0FF" opacity="0.3"/></svg>"##,
    },
    PearlWord {
        word: "BRAVE",
        riddle: "Not without fear, but moving forward anyway.",
        hint_svg: r##"<svg viewBox="0 0 80 80" width="80" height="80" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="80" height="80" rx="16" fill="rgba(13,43,69,0.6)"/><path d="M40 12 L45 28 L62 28 L48 38 L53 54 L40 44 L27 54 L32 38 L18 28 L35 28 Z" fill="#FFD93D" opacity="0.9"/><path d="M40 12 L45 28 L62 28 L48 38 L53 54 L40 44 L27 54 L32 38 L18 28 L35 28 Z" fill="none" stroke="#FFE0B2" stroke-width="1.5"/><text x="40" y="72" text-anchor="middle" fill="#F5F0FF" font-size="10" font-family="sans-serif" opacity="0.7">&#9876;</text></svg>"##,
    },
    PearlWord {
        word: "SHINE",
        riddle: "What the sun and your smile have in common.",
        hint_svg: r##"<svg viewBox="0 0 80 80" width="80" height="80" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="80" height="80" rx="16" fill="rgba(13,43,69,0.6)"/><circle cx="40" cy="38" r="14" fill="#FFE0B2"/><circle cx="40" cy="38" r="14" fill="none" stroke="#FFD93D" stroke-width="2"/><line x1="40" y1="14" x2="40" y2="20" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="40" y1="56" x2="40" y2="62" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="14" y1="38" x2="20" y2="38" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="60" y1="38" x2="66" y2="38" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="21" y1="21" x2="25" y2="25" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="55" y1="55" x2="59" y2="59" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="59" y1="21" x2="55" y2="25" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/><line x1="21" y1="59" x2="25" y2="55" stroke="#FFD93D" stroke-width="2.5" stroke-linecap="round"/></svg>"##,
    },
];

#[component]
pub fn PearlWisdomActivity() -> impl IntoView {
    view! {
        <ActivityWrapper id=ActivityId::PearlWisdom>
            <PearlGame />
        </ActivityWrapper>
    }
}

#[component]
fn PearlGame() -> impl IntoView {
    let complete = expect_context::<ActivityCompleteSignal>();
    let word_index = RwSignal::new(0_usize);
    let collected = RwSignal::new(String::new());
    let pearls = RwSignal::new(Vec::<PearlData>::new());
    let pearl_caught = RwSignal::new(Vec::<RwSignal<bool>>::new());
    let message = RwSignal::new(String::from("Read the riddle, then catch the letters!"));
    let completed_words = RwSignal::new(Vec::<&'static str>::new());

    let current_word = move || {
        let idx = word_index.get();
        if idx < WORDS.len() { WORDS[idx].word } else { "" }
    };

    let current_riddle = move || {
        let idx = word_index.get();
        if idx < WORDS.len() { WORDS[idx].riddle } else { "" }
    };

    let current_hint_svg = move || {
        let idx = word_index.get();
        if idx < WORDS.len() { WORDS[idx].hint_svg } else { "" }
    };

    let spawn_pearls = move || {
        let word = current_word();
        if word.is_empty() { return; }

        let mut rng = rand::thread_rng();
        let mut letters: Vec<char> = word.chars().collect();
        let decoy_count = 3;
        for _ in 0..decoy_count {
            let decoy = DECOYS.as_bytes()[rng.gen_range(0..DECOYS.len())] as char;
            letters.push(decoy);
        }
        letters.shuffle(&mut rng);

        let new_pearls: Vec<PearlData> = letters
            .into_iter()
            .enumerate()
            .map(|(i, ch)| {
                let left = rng.gen_range(5..85);
                let duration = rng.gen_range(6.0..10.0_f64);
                let delay = i as f64 * 1.2 + rng.gen_range(0.0..0.8_f64);
                PearlData {
                    id: i,
                    letter: ch,
                    left,
                    duration,
                    delay,
                    caught: false,
                }
            })
            .collect();

        let signals: Vec<RwSignal<bool>> = new_pearls.iter().map(|_| RwSignal::new(false)).collect();
        pearl_caught.set(signals);
        pearls.set(new_pearls);
        collected.set(String::new());
    };

    spawn_pearls();

    let on_pearl_click = move |pearl_id: usize, letter: char| {
        let word = current_word();
        if word.is_empty() { return; }

        let current_collected = collected.get();
        let mut remaining: Vec<char> = word.chars().collect();
        for ch in current_collected.chars() {
            if let Some(pos) = remaining.iter().position(|&c| c == ch) {
                remaining.remove(pos);
            }
        }

        if remaining.contains(&letter) {
            collected.update(|c| c.push(letter));
            let signals = pearl_caught.get();
            if pearl_id < signals.len() {
                signals[pearl_id].set(true);
            }

            let new_collected = collected.get();
            if new_collected.len() == word.len() {
                completed_words.update(|w| w.push(word));
                let idx = word_index.get();
                if idx + 1 >= WORDS.len() {
                    message.set("All words found! You did it!".into());
                    complete.trigger();
                } else {
                    word_index.set(idx + 1);
                    message.set("Well done! Here's the next riddle...".into());
                    spawn_pearls();
                }
            } else {
                message.set("Got it! Find the next letter...".into());
            }
        } else {
            message.set("That letter isn't needed... try another!".into());
        }
    };

    view! {
        <div class="pearl-wisdom">
            <div class="pearl-speech">
                <div class="speech-bubble clam-speech">
                    <span class="speech-creature"><ClamSvg /></span>
                    <p>{message}</p>
                </div>
            </div>

            <Show when=move || !completed_words.get().is_empty()>
                <div class="pearl-gathered-words">
                    {move || completed_words.get().iter().map(|w| {
                        view! { <span class="pearl-gathered-word">{*w}</span> }
                    }).collect::<Vec<_>>()}
                </div>
            </Show>

            <div class="pearl-riddle-area">
                <div class="pearl-hint-svg" inner_html=current_hint_svg></div>
                <p class="pearl-riddle">{current_riddle}</p>
            </div>

            <div class="pearl-word-display">
                {move || {
                    let word = current_word();
                    let got = collected.get();
                    let mut available: Vec<char> = got.chars().collect();
                    word.chars().map(|ch| {
                        let filled = if let Some(pos) = available.iter().position(|&c| c == ch) {
                            available.remove(pos);
                            true
                        } else {
                            false
                        };
                        let class = if filled { "pearl-slot filled" } else { "pearl-slot" };
                        let display = if filled { ch.to_string() } else { "_".to_string() };
                        view! { <span class=class>{display}</span> }
                    }).collect::<Vec<_>>()
                }}
                <span class="pearl-progress">
                    {move || format!("{} / {}", word_index.get() + 1, WORDS.len())}
                </span>
            </div>

            <div class="pearl-field">
                {move || {
                    let caught_signals = pearl_caught.get();
                    pearls.get().into_iter().enumerate().map(|(idx, pearl)| {
                        let style = format!(
                            "left: {}%; animation-duration: {:.1}s; animation-delay: {:.1}s;",
                            pearl.left, pearl.duration, pearl.delay
                        );
                        let id = pearl.id;
                        let letter = pearl.letter;
                        let caught = caught_signals[idx];
                        view! {
                            <Show when=move || !caught.get()>
                                <button
                                    class="pearl-bubble"
                                    style=style.clone()
                                    on:click=move |_| on_pearl_click(id, letter)
                                >
                                    {letter.to_string()}
                                </button>
                            </Show>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[derive(Clone)]
struct PearlData {
    id: usize,
    letter: char,
    left: i32,
    duration: f64,
    delay: f64,
    caught: bool,
}
