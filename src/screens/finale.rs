use leptos::prelude::*;
use crate::state::{ActivityId, AppState};
use crate::wishes;
use crate::particles::UnderwaterScene;

#[component]
pub fn FinaleScreen() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_reset = move |_| {
        state.reset();
    };

    let creature_icons = ["🦪", "🐟", "🪸", "🐡", "🦀"];

    view! {
        <div class="screen finale-screen">
            <UnderwaterScene bubble_count=30 seaweed_count=4 fish_count=5
                show_light_rays=false />

            <div class="finale-creatures">
                {creature_icons
                    .into_iter()
                    .enumerate()
                    .map(|(i, icon)| {
                        let delay = 0.5 + i as f64 * 1.0;
                        let style = format!("animation-delay: {delay:.1}s;");
                        view! {
                            <span class="finale-creature" style=style>{icon}</span>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div class="finale-wishes">
                {ActivityId::all()
                    .into_iter()
                    .map(|id| {
                        view! {
                            <div class="finale-wish">
                                <h3>{id.label()}</h3>
                                <p>{wishes::wish_for(&id)}</p>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div class="finale-reveal">
                <div class="axolotl-surprise">
                    <svg class="axolotl-svg" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
                        <g class="axolotl-body-group">
                            // body
                            <ellipse cx="100" cy="110" rx="45" ry="35" fill="var(--axolotl-primary)"/>
                            // belly
                            <ellipse cx="100" cy="118" rx="30" ry="22" fill="var(--axolotl-secondary)"/>
                            // head
                            <ellipse cx="100" cy="78" rx="35" ry="28" fill="var(--axolotl-primary)"/>
                            // face
                            <ellipse cx="100" cy="82" rx="22" ry="16" fill="var(--axolotl-secondary)" opacity="0.5"/>
                            // eyes
                            <circle cx="87" cy="73" r="5" fill="#2D1B3D"/>
                            <circle cx="113" cy="73" r="5" fill="#2D1B3D"/>
                            <circle cx="88" cy="72" r="2" fill="white"/>
                            <circle cx="114" cy="72" r="2" fill="white"/>
                            // smile
                            <path d="M90 85 Q100 93 110 85" fill="none" stroke="#2D1B3D"
                                  stroke-width="2" stroke-linecap="round"/>
                            // left gill
                            <g class="gill-left">
                                <path d="M65 65 Q55 50 50 35" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="4" stroke-linecap="round"/>
                                <path d="M65 60 Q50 55 40 45" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="3" stroke-linecap="round"/>
                                <path d="M67 70 Q55 65 45 60" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="3" stroke-linecap="round"/>
                                <circle cx="50" cy="35" r="3" fill="var(--axolotl-glow)"/>
                                <circle cx="40" cy="45" r="2.5" fill="var(--axolotl-glow)"/>
                                <circle cx="45" cy="60" r="2.5" fill="var(--axolotl-glow)"/>
                            </g>
                            // right gill
                            <g class="gill-right">
                                <path d="M135 65 Q145 50 150 35" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="4" stroke-linecap="round"/>
                                <path d="M135 60 Q150 55 160 45" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="3" stroke-linecap="round"/>
                                <path d="M133 70 Q145 65 155 60" fill="none" stroke="var(--axolotl-glow)"
                                      stroke-width="3" stroke-linecap="round"/>
                                <circle cx="150" cy="35" r="3" fill="var(--axolotl-glow)"/>
                                <circle cx="160" cy="45" r="2.5" fill="var(--axolotl-glow)"/>
                                <circle cx="155" cy="60" r="2.5" fill="var(--axolotl-glow)"/>
                            </g>
                            // tail
                            <g class="axolotl-tail">
                                <path d="M100 145 Q120 155 130 170 Q125 160 110 155"
                                      fill="var(--axolotl-primary)" opacity="0.8"/>
                            </g>
                            // arms
                            <path d="M60 115 Q45 125 40 135" fill="none" stroke="var(--axolotl-primary)"
                                  stroke-width="6" stroke-linecap="round"/>
                            <path d="M140 115 Q155 125 160 135" fill="none" stroke="var(--axolotl-primary)"
                                  stroke-width="6" stroke-linecap="round"/>
                        </g>
                    </svg>
                </div>
                <h1 class="finale-title">{wishes::finale_message()}</h1>
                <p class="finale-personal">"The ocean celebrates YOU today!"</p>
            </div>

            <button class="btn-secondary reset-btn" on:click=on_reset>
                "Play Again"
            </button>
        </div>
    }
}
