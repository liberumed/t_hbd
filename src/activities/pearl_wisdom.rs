use leptos::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;

const WORDS: [&str; 3] = ["DREAM", "BRAVE", "SHINE"];
const DECOYS: &str = "XZQWKJFGHPNM";

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
    let message = RwSignal::new(String::from("Catch the pearls to spell a word!"));

    let current_word = move || {
        let idx = word_index.get();
        if idx < WORDS.len() { WORDS[idx] } else { "" }
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
                let left = rng.gen_range(5..90);
                let delay = i as f64 * 0.8 + rng.gen_range(0.0..0.5_f64);
                PearlData {
                    id: i,
                    letter: ch,
                    left,
                    delay,
                    caught: false,
                }
            })
            .collect();

        pearls.set(new_pearls);
        collected.set(String::new());
    };

    spawn_pearls();

    let on_pearl_click = move |pearl_id: usize, letter: char| {
        let word = current_word();
        if word.is_empty() { return; }

        let current_collected = collected.get();
        let next_expected = word.chars().nth(current_collected.len());

        if next_expected == Some(letter) {
            collected.update(|c| c.push(letter));
            pearls.update(|ps| {
                if let Some(p) = ps.iter_mut().find(|p| p.id == pearl_id) {
                    p.caught = true;
                }
            });

            let new_collected = collected.get();
            if new_collected.len() == word.len() {
                let idx = word_index.get();
                if idx + 1 >= WORDS.len() {
                    message.set("All words found! You did it!".into());
                    complete.trigger();
                } else {
                    word_index.set(idx + 1);
                    message.set(format!("Great! Now spell: {}", WORDS[idx + 1]));
                    spawn_pearls();
                }
            }
        } else {
            message.set("Not quite... try another pearl!".into());
        }
    };

    view! {
        <div class="pearl-wisdom">
            <div class="pearl-speech">
                <div class="speech-bubble clam-speech">
                    <span class="speech-creature">"🦪"</span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="pearl-word-display">
                <span class="pearl-word-label">"Spell: "</span>
                {move || {
                    let word = current_word();
                    let got = collected.get();
                    word.chars().enumerate().map(|(i, ch)| {
                        let filled = i < got.len();
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
                    pearls.get().into_iter().map(|pearl| {
                        let style = format!(
                            "left: {}%; animation-delay: {:.1}s;",
                            pearl.left, pearl.delay
                        );
                        let visible = !pearl.caught;
                        let id = pearl.id;
                        let letter = pearl.letter;
                        view! {
                            <Show when=move || visible>
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
    delay: f64,
    caught: bool,
}
