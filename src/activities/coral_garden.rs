use leptos::prelude::*;
use rand::Rng;
use super::{ActivityWrapper, ActivityCompleteSignal};
use crate::state::ActivityId;
use crate::creatures::PonyoSvg;

const CORAL_TYPES: [(&str, &str); 6] = [
    ("Branch", "var(--crab-primary)"),
    ("Brain", "#8B5DAF"),
    ("Fan", "var(--ponyo-primary)"),
    ("Tube", "var(--gold)"),
    ("Mushroom", "var(--ocean-mid)"),
    ("Flower", "#5B8CDB"),
];
const NEEDED: usize = 15;

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
        let bottom = rng.gen_range(8..22);
        let rotation = rng.gen_range(-15..15);
        let scale = rng.gen_range(0.8..1.4_f64);

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
            3  => life_message.set("Small fish appear nearby!".into()),
            5  => life_message.set("A squid drifts in!".into()),
            7  => life_message.set("An octopus peeks from behind!".into()),
            9  => life_message.set("A jellyfish drifts by!".into()),
            11 => life_message.set("A dolphin leaps with joy!".into()),
            13 => life_message.set("A whale watches from the deep!".into()),
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
                    <span class="speech-creature"><PonyoSvg size=52 /></span>
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
                            <svg viewBox="0 0 40 50" width="40" height="50"
                                 xmlns="http://www.w3.org/2000/svg"
                                 inner_html=coral_svg(i)>
                            </svg>
                            <span class="coral-name">{name}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <Show when=move || !life_message.get().is_empty()>
                <div class="coral-life-msg">{life_message}</div>
            </Show>

            <div class="coral-reef">
                // Rocky formations — layered, transparent, higher floor feel
                <svg class="reef-rocks" viewBox="0 0 1200 240"
                     preserveAspectRatio="xMidYMax slice"
                     xmlns="http://www.w3.org/2000/svg">
                    // deep back layer
                    <path d="M0 190 Q80 162 170 174 Q260 158 360 172 Q460 160 560 176
                             Q660 162 760 174 Q860 160 960 175 Q1060 162 1200 172 L1200 240 L0 240 Z"
                          fill="#0D2B45" opacity="0.65"/>
                    // mid rocky ridge
                    <path d="M0 205 Q50 182 110 196 Q175 178 250 194 Q325 180 410 198
                             Q490 182 575 197 Q655 183 740 198 Q820 182 910 198
                             Q990 183 1080 197 Q1140 186 1200 194 L1200 240 L0 240 Z"
                          fill="#1A3A4A" opacity="0.7"/>
                    // individual rock bumps mid layer
                    <ellipse cx="140" cy="202" rx="55" ry="20" fill="#0D2B45" opacity="0.6"/>
                    <ellipse cx="370" cy="197" rx="70" ry="26" fill="#122030" opacity="0.55"/>
                    <ellipse cx="610" cy="204" rx="50" ry="19" fill="#0D2B45" opacity="0.6"/>
                    <ellipse cx="840" cy="198" rx="65" ry="24" fill="#122030" opacity="0.55"/>
                    <ellipse cx="1060" cy="203" rx="55" ry="20" fill="#0D2B45" opacity="0.6"/>
                    // front rocky surface
                    <path d="M0 218 Q35 200 78 214 Q120 200 168 216 Q215 202 270 218
                             Q325 203 385 218 Q445 204 510 220 Q570 206 635 220
                             Q700 206 765 219 Q830 205 895 220 Q960 206 1020 219
                             Q1080 207 1140 218 Q1170 212 1200 216 L1200 240 L0 240 Z"
                          fill="#1A3A4A" opacity="0.85"/>
                    // surface rock formations
                    <ellipse cx="60" cy="217" rx="36" ry="14" fill="#2A4A5A" opacity="0.7"/>
                    <ellipse cx="280" cy="213" rx="48" ry="18" fill="#1A3A4A" opacity="0.65"/>
                    <ellipse cx="500" cy="218" rx="38" ry="14" fill="#2A4A5A" opacity="0.7"/>
                    <ellipse cx="730" cy="214" rx="44" ry="16" fill="#1A3A4A" opacity="0.65"/>
                    <ellipse cx="960" cy="217" rx="40" ry="15" fill="#2A4A5A" opacity="0.7"/>
                    <ellipse cx="1150" cy="215" rx="34" ry="13" fill="#1A3A4A" opacity="0.65"/>
                </svg>

                <Show when=move || { count.get() >= 3 }>
                    <span class="reef-life" style="left: 20%; bottom: 40%;">"🐠"</span>
                </Show>
                <Show when=move || { count.get() >= 5 }>
                    <span class="reef-life" style="left: 75%; bottom: 55%;">"🦑"</span>
                </Show>
                <Show when=move || { count.get() >= 7 }>
                    <span class="reef-life" style="left: 50%; bottom: 48%;">"🐙"</span>
                </Show>
                <Show when=move || { count.get() >= 9 }>
                    <span class="reef-life" style="left: 10%; bottom: 60%;">"🪼"</span>
                </Show>
                <Show when=move || { count.get() >= 11 }>
                    <span class="reef-life" style="left: 60%; bottom: 65%;">"🐬"</span>
                </Show>
                <Show when=move || { count.get() >= 13 }>
                    <span class="reef-life" style="left: 35%; bottom: 72%;">"🐋"</span>
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
                            <svg viewBox="0 0 40 50" width="120" height="150"
                                 xmlns="http://www.w3.org/2000/svg"
                                 style=format!("--coral-color: {};", color)
                                 inner_html=coral_svg(coral.coral_type)>
                            </svg>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

        </div>
    }
}

fn coral_svg(coral_type: usize) -> &'static str {
    match coral_type {
        // Branch: multi-level branching tree
        0 => r##"<path d="M20 50 L20 36 M20 36 L11 22 M11 22 L6 11 M11 22 L16 10 M20 36 L29 22 M29 22 L24 10 M29 22 L34 11 M20 43 L13 33 M20 43 L27 33" stroke="var(--coral-color, #E85D3A)" stroke-width="3.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>"##,
        // Brain: round dome with wavy maze grooves
        1 => r##"<circle cx="20" cy="26" r="16" fill="var(--coral-color, #8B5DAF)" opacity="0.9"/>
<path d="M7 21 Q10 17 13 21 Q16 17 19 21 Q22 17 25 21 Q28 17 31 21 Q33 17 34 21" fill="none" stroke="white" stroke-width="1.8" opacity="0.45" stroke-linecap="round"/>
<path d="M5 27 Q8 23 11 27 Q14 23 17 27 Q20 23 23 27 Q26 23 29 27 Q32 23 35 27" fill="none" stroke="white" stroke-width="1.8" opacity="0.45" stroke-linecap="round"/>
<path d="M7 33 Q10 29 13 33 Q16 29 19 33 Q22 29 25 33 Q28 29 31 33 Q33 29 34 33" fill="none" stroke="white" stroke-width="1.8" opacity="0.45" stroke-linecap="round"/>"##,
        // Fan: wide spreading fan with vein lines
        2 => r##"<line x1="20" y1="50" x2="20" y2="38" stroke="var(--coral-color, #FF6B8A)" stroke-width="3.5" stroke-linecap="round"/>
<path d="M20 38 Q4 28 3 12 Q10 5 20 22 Q30 5 37 12 Q36 28 20 38 Z" fill="var(--coral-color, #FF6B8A)" opacity="0.75"/>
<path d="M20 38 L20 16 M20 32 L10 20 M20 32 L30 20 M20 35 L13 25 M20 35 L27 25" fill="none" stroke="white" stroke-width="0.9" opacity="0.4"/>"##,
        // Tube: three tubes of different heights
        3 => r##"<rect x="8" y="22" width="8" height="28" rx="4" fill="var(--coral-color, #FFD93D)" opacity="0.9"/>
<rect x="16" y="12" width="8" height="38" rx="4" fill="var(--coral-color, #FFD93D)" opacity="0.9"/>
<rect x="24" y="18" width="8" height="32" rx="4" fill="var(--coral-color, #FFD93D)" opacity="0.9"/>
<ellipse cx="12" cy="22" rx="4" ry="2.5" fill="white" opacity="0.35"/>
<ellipse cx="20" cy="12" rx="4" ry="2.5" fill="white" opacity="0.35"/>
<ellipse cx="28" cy="18" rx="4" ry="2.5" fill="white" opacity="0.35"/>"##,
        // Mushroom: flat wide cap on short stalk with radial texture
        4 => r##"<rect x="17" y="38" width="6" height="12" rx="3" fill="var(--coral-color, #4AADBA)" opacity="0.85"/>
<ellipse cx="20" cy="36" rx="17" ry="9" fill="var(--coral-color, #4AADBA)" opacity="0.95"/>
<ellipse cx="20" cy="33" rx="14" ry="7" fill="var(--coral-color, #4AADBA)" opacity="0.6"/>
<path d="M5 38 Q20 44 35 38" fill="none" stroke="white" stroke-width="1" opacity="0.3"/>
<path d="M9 40 Q20 45 31 40" fill="none" stroke="white" stroke-width="1" opacity="0.25"/>"##,
        // Flower: anemone with petal-tip tentacles around a center
        _ => r##"<line x1="20" y1="44" x2="20" y2="50" stroke="var(--coral-color, #5B8CDB)" stroke-width="4" stroke-linecap="round"/>
<line x1="12" y1="46" x2="14" y2="50" stroke="var(--coral-color, #5B8CDB)" stroke-width="3" stroke-linecap="round"/>
<line x1="28" y1="46" x2="26" y2="50" stroke="var(--coral-color, #5B8CDB)" stroke-width="3" stroke-linecap="round"/>
<circle cx="20" cy="35" r="8" fill="var(--coral-color, #5B8CDB)" opacity="0.9"/>
<circle cx="20" cy="24" r="5" fill="var(--coral-color, #5B8CDB)" opacity="0.85"/>
<circle cx="29" cy="28" r="5" fill="var(--coral-color, #5B8CDB)" opacity="0.85"/>
<circle cx="32" cy="38" r="5" fill="var(--coral-color, #5B8CDB)" opacity="0.85"/>
<circle cx="11" cy="28" r="5" fill="var(--coral-color, #5B8CDB)" opacity="0.85"/>
<circle cx="8" cy="38" r="5" fill="var(--coral-color, #5B8CDB)" opacity="0.85"/>
<circle cx="20" cy="35" r="4" fill="white" opacity="0.4"/>
<circle cx="20" cy="24" r="2" fill="white" opacity="0.5"/>
<circle cx="29" cy="28" r="2" fill="white" opacity="0.5"/>
<circle cx="32" cy="38" r="2" fill="white" opacity="0.5"/>
<circle cx="11" cy="28" r="2" fill="white" opacity="0.5"/>
<circle cx="8" cy="38" r="2" fill="white" opacity="0.5"/>"##,
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
