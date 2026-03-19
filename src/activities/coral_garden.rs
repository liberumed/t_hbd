use leptos::prelude::*;
use rand::Rng;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;

const CORAL_TYPES: [(&str, &str); 6] = [
    ("Branch", "var(--crab-primary)"),
    ("Brain", "#8B5DAF"),
    ("Fan", "var(--ponyo-primary)"),
    ("Tube", "var(--gold)"),
    ("Mushroom", "var(--ocean-mid)"),
    ("Flower", "#5B8CDB"),
];
const NEEDED: usize = 8;

#[component]
pub fn CoralGardenActivity() -> impl IntoView {
    view! {
        <ActivityWrapper id=ActivityId::CoralGarden>
            <CoralGame />
        </ActivityWrapper>
    }
}

#[component]
fn CoralGame() -> impl IntoView {
    let complete = expect_context::<ActivityCompleteSignal>();
    let placed_corals = RwSignal::new(Vec::<PlacedCoral>::new());
    let count = RwSignal::new(0_usize);
    let message = RwSignal::new(String::from("Click the corals below to plant them!"));
    let life_message = RwSignal::new(String::new());

    let place_coral = move |coral_type: usize| {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(5..90);
        let bottom = rng.gen_range(5..35);
        let rotation = rng.gen_range(-15..15);
        let scale = rng.gen_range(0.8..1.3_f64);

        placed_corals.update(|corals| {
            corals.push(PlacedCoral {
                coral_type,
                x,
                bottom,
                rotation,
                scale,
            });
        });

        count.update(|c| *c += 1);
        let n = count.get();

        match n {
            3 => life_message.set("Small fish appear nearby!".into()),
            5 => life_message.set("A seahorse joins the reef!".into()),
            7 => life_message.set("An octopus peeks from behind!".into()),
            _ if n >= NEEDED => {
                message.set("Beautiful! Your reef is alive!".into());
                life_message.set("The reef is teeming with life!".into());
                complete.trigger();
            }
            _ => {}
        }

        if n < NEEDED {
            message.set(format!("Corals: {} / {} - keep building!", n, NEEDED));
        }
    };

    view! {
        <div class="coral-garden">
            <div class="coral-speech">
                <div class="speech-bubble ponyo-speech">
                    <span class="speech-creature">"🫧"</span>
                    <p>{message}</p>
                </div>
            </div>

            <div class="coral-palette">
                {CORAL_TYPES.iter().enumerate().map(|(i, (name, color))| {
                    let color = *color;
                    let name = *name;
                    view! {
                        <button
                            class="coral-palette-item"
                            style=format!("--coral-color: {};", color)
                            on:click=move |_| place_coral(i)
                        >
                            <svg viewBox="0 0 40 50" width="40" height="50" xmlns="http://www.w3.org/2000/svg">
                                {coral_svg(i)}
                            </svg>
                            <span class="coral-name">{name}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="coral-reef">
                <Show when=move || { count.get() >= 3 }>
                    <span class="reef-life fish-life" style="left: 20%; bottom: 40%;">"🐠"</span>
                </Show>
                <Show when=move || { count.get() >= 5 }>
                    <span class="reef-life" style="left: 70%; bottom: 45%;">"🦈"</span>
                </Show>
                <Show when=move || { count.get() >= 7 }>
                    <span class="reef-life" style="left: 50%; bottom: 50%;">"🐙"</span>
                </Show>

                {move || placed_corals.get().iter().enumerate().map(|(i, coral)| {
                    let style = format!(
                        "left: {}%; bottom: {}%; transform: rotate({}deg) scale({:.1}); \
                         animation-delay: {}ms;",
                        coral.x, coral.bottom, coral.rotation, coral.scale, i * 50
                    );
                    let color = CORAL_TYPES[coral.coral_type].1;
                    view! {
                        <div class="placed-coral" style=style>
                            <svg viewBox="0 0 40 50" width="40" height="50" xmlns="http://www.w3.org/2000/svg"
                                 style=format!("--coral-color: {};", color)>
                                {coral_svg(coral.coral_type)}
                            </svg>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <Show when=move || !life_message.get().is_empty()>
                <div class="coral-life-msg">{life_message}</div>
            </Show>
        </div>
    }
}

fn coral_svg(coral_type: usize) -> &'static str {
    match coral_type {
        0 => r##"<path d="M20 50 L20 30 L10 15 M20 30 L30 10 M20 35 L12 25" fill="none" stroke="var(--coral-color, #E85D3A)" stroke-width="4" stroke-linecap="round"/>"##,
        1 => r##"<ellipse cx="20" cy="30" rx="15" ry="12" fill="var(--coral-color, #8B5DAF)" opacity="0.8"/><path d="M10 28 Q15 22 20 28 Q25 22 30 28" fill="none" stroke="var(--coral-color, #8B5DAF)" stroke-width="1.5" opacity="0.5"/>"##,
        2 => r##"<path d="M20 50 L20 35 Q5 25 10 10 Q20 20 30 10 Q35 25 20 35" fill="var(--coral-color, #FF6B8A)" opacity="0.7"/>"##,
        3 => r##"<rect x="15" y="10" width="10" height="40" rx="5" fill="var(--coral-color, #FFD93D)" opacity="0.7"/><ellipse cx="20" cy="10" rx="7" ry="4" fill="var(--coral-color, #FFD93D)" opacity="0.5"/>"##,
        4 => r##"<path d="M20 50 L20 30" stroke="var(--coral-color, #4AADBA)" stroke-width="4" stroke-linecap="round"/><ellipse cx="20" cy="22" rx="14" ry="10" fill="var(--coral-color, #4AADBA)" opacity="0.7"/>"##,
        _ => r##"<circle cx="20" cy="20" r="10" fill="var(--coral-color, #5B8CDB)" opacity="0.7"/><circle cx="14" cy="28" r="7" fill="var(--coral-color, #5B8CDB)" opacity="0.5"/><circle cx="28" cy="27" r="8" fill="var(--coral-color, #5B8CDB)" opacity="0.6"/><path d="M20 35 L20 50" stroke="var(--coral-color, #5B8CDB)" stroke-width="3"/>"##,
    }
}

#[derive(Clone)]
struct PlacedCoral {
    coral_type: usize,
    x: i32,
    bottom: i32,
    rotation: i32,
    scale: f64,
}
