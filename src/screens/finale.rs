use leptos::prelude::*;
use crate::state::{ActivityId, AppState};
use crate::wishes;
use crate::particles::UnderwaterScene;
use crate::creatures::{ClamIcon, ClownFishSvg, PonyoSvg};

#[component]
pub fn FinaleScreen() -> impl IntoView {
    let state = expect_context::<AppState>();

    let on_reset = move |_| {
        state.reset();
    };

    view! {
        <div class="screen finale-screen">
            <UnderwaterScene bubble_count=30 seaweed_count=4 fish_count=5
                show_light_rays=false />

            <div class="finale-wishes">
                {ActivityId::all()
                    .into_iter()
                    .map(|id| {
                        view! {
                            <div class="finale-wish">
                                <span class="finale-wish-icon">{activity_icon_view(id)}</span>
                                <div class="finale-wish-text">
                                    <h3>{id.label()}</h3>
                                    <p>{wishes::wish_for(&id)}</p>
                                </div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <div class="finale-reveal">
                <div class="axolotl-surprise">
                    <svg class="axolotl-svg" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
                        <defs>
                            <radialGradient id="body-glow" cx="50%" cy="50%" r="50%">
                                <stop offset="0%" stop-color="white" stop-opacity="0.25"/>
                                <stop offset="100%" stop-color="white" stop-opacity="0"/>
                            </radialGradient>
                            <radialGradient id="gill-glow-l" cx="50%" cy="50%" r="50%">
                                <stop offset="0%" stop-color="#FFA040" stop-opacity="0.5"/>
                                <stop offset="100%" stop-color="#FFA040" stop-opacity="0"/>
                            </radialGradient>
                            <radialGradient id="gill-glow-r" cx="50%" cy="50%" r="50%">
                                <stop offset="0%" stop-color="#FFA040" stop-opacity="0.5"/>
                                <stop offset="100%" stop-color="#FFA040" stop-opacity="0"/>
                            </radialGradient>
                        </defs>

                        <g class="axolotl-body-group">
                            // soft white body glow behind
                            <ellipse cx="100" cy="100" rx="70" ry="60" fill="url(#body-glow)"/>

                            // body
                            <ellipse cx="100" cy="110" rx="45" ry="35" fill="var(--axolotl-primary)"/>
                            // belly — white glowing
                            <ellipse cx="100" cy="118" rx="30" ry="22" fill="white" opacity="0.35"/>
                            <ellipse cx="100" cy="118" rx="22" ry="15" fill="white" opacity="0.3"/>
                            // head
                            <ellipse cx="100" cy="78" rx="35" ry="28" fill="var(--axolotl-primary)"/>
                            // face highlight
                            <ellipse cx="100" cy="76" rx="20" ry="14" fill="white" opacity="0.18"/>
                            // eyes
                            <circle cx="87" cy="73" r="5" fill="#2D1B3D"/>
                            <circle cx="113" cy="73" r="5" fill="#2D1B3D"/>
                            <circle cx="88" cy="72" r="2" fill="white"/>
                            <circle cx="114" cy="72" r="2" fill="white"/>
                            // smile
                            <path d="M90 85 Q100 93 110 85" fill="none" stroke="#2D1B3D"
                                  stroke-width="2" stroke-linecap="round"/>

                            // orange gill glow halos
                            <ellipse cx="52" cy="48" rx="22" ry="22" fill="url(#gill-glow-l)"/>
                            <ellipse cx="148" cy="48" rx="22" ry="22" fill="url(#gill-glow-r)"/>

                            // left gill — orange tips
                            <g class="gill-left">
                                <path d="M65 65 Q55 50 50 35" fill="none" stroke="#FFC070"
                                      stroke-width="4" stroke-linecap="round"/>
                                <path d="M65 60 Q50 55 40 45" fill="none" stroke="#FFC070"
                                      stroke-width="3" stroke-linecap="round"/>
                                <path d="M67 70 Q55 65 45 60" fill="none" stroke="#FFC070"
                                      stroke-width="3" stroke-linecap="round"/>
                                <circle cx="50" cy="35" r="4" fill="#FFA040"/>
                                <circle cx="40" cy="45" r="3.5" fill="#FFA040"/>
                                <circle cx="45" cy="60" r="3.5" fill="#FFA040"/>
                                // tip glow dots
                                <circle cx="50" cy="35" r="2" fill="white" opacity="0.7"/>
                                <circle cx="40" cy="45" r="1.5" fill="white" opacity="0.7"/>
                                <circle cx="45" cy="60" r="1.5" fill="white" opacity="0.7"/>
                            </g>
                            // right gill — orange tips
                            <g class="gill-right">
                                <path d="M135 65 Q145 50 150 35" fill="none" stroke="#FFC070"
                                      stroke-width="4" stroke-linecap="round"/>
                                <path d="M135 60 Q150 55 160 45" fill="none" stroke="#FFC070"
                                      stroke-width="3" stroke-linecap="round"/>
                                <path d="M133 70 Q145 65 155 60" fill="none" stroke="#FFC070"
                                      stroke-width="3" stroke-linecap="round"/>
                                <circle cx="150" cy="35" r="4" fill="#FFA040"/>
                                <circle cx="160" cy="45" r="3.5" fill="#FFA040"/>
                                <circle cx="155" cy="60" r="3.5" fill="#FFA040"/>
                                // tip glow dots
                                <circle cx="150" cy="35" r="2" fill="white" opacity="0.7"/>
                                <circle cx="160" cy="45" r="1.5" fill="white" opacity="0.7"/>
                                <circle cx="155" cy="60" r="1.5" fill="white" opacity="0.7"/>
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

fn activity_icon_view(id: ActivityId) -> impl leptos::IntoView {
    use leptos::prelude::*;
    match id {
        ActivityId::PearlWisdom    => view! { <ClamIcon /> }.into_any(),
        ActivityId::CurrentRider   => view! { <ClownFishSvg size=36 /> }.into_any(),
        ActivityId::CoralGarden    => view! { <PonyoSvg size=36 /> }.into_any(),
        ActivityId::DeepSeaLights  => view! { "🐡" }.into_any(),
        ActivityId::TreasureHunt   => view! { "🦀" }.into_any(),
    }
}
