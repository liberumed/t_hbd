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
                    <svg class="axolotl-svg" viewBox="0 0 220 250" xmlns="http://www.w3.org/2000/svg">
                        <defs>
                            <radialGradient id="body-glow" cx="50%" cy="50%" r="50%">
                                <stop offset="0%" stop-color="white" stop-opacity="0.22"/>
                                <stop offset="100%" stop-color="white" stop-opacity="0"/>
                            </radialGradient>
                        </defs>

                        <g class="axolotl-body-group">
                            <ellipse cx="110" cy="120" rx="95" ry="90" fill="url(#body-glow)"/>

                            // === TAIL FIN ===
                            <g class="axolotl-tail">
                                <path d="M88 215 Q55 242 28 238 Q14 222 42 208 Q62 198 84 205 Z"
                                      fill="#F4A0B8"/>
                                <path d="M86 212 Q60 234 40 231 Q30 220 52 211 Q64 205 82 208 Z"
                                      fill="#FAD0E0" opacity="0.65"/>
                            </g>

                            // === BODY ===
                            <ellipse cx="112" cy="178" rx="42" ry="50" fill="#F4A0B8"/>
                            <ellipse cx="112" cy="185" rx="26" ry="33" fill="#FAD0E0" opacity="0.6"/>
                            <ellipse cx="112" cy="182" rx="16" ry="22" fill="white" opacity="0.2"/>

                            // === BACK LEGS ===
                            <path d="M88 222 Q82 236 80 246" fill="none" stroke="#F4A0B8"
                                  stroke-width="11" stroke-linecap="round"/>
                            <ellipse cx="79" cy="249" rx="10" ry="6" fill="#F4A0B8"/>
                            <path d="M132 218 Q146 230 152 242" fill="none" stroke="#F4A0B8"
                                  stroke-width="11" stroke-linecap="round"/>
                            <ellipse cx="154" cy="245" rx="10" ry="6" fill="#F4A0B8"/>

                            // === HEAD ===
                            <ellipse cx="110" cy="94" rx="66" ry="62" fill="#F4A0B8"/>
                            <ellipse cx="94" cy="70" rx="34" ry="20" fill="white" opacity="0.17"/>

                            // === LEFT GILLS (5-branch fan) ===
                            <g class="gill-left">
                                <ellipse cx="50" cy="82" rx="11" ry="9" fill="#D84070"/>
                                // branch 1 — lowest, going left
                                <path d="M48 88 Q24 86 8 82" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                <ellipse cx="4" cy="81" rx="11" ry="7" fill="#D84070" transform="rotate(8 4 81)"/>
                                <path d="M48 88 Q24 86 8 82" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                // branch 2 — going upper-left
                                <path d="M48 79 Q26 68 10 56" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                <ellipse cx="5" cy="52" rx="11" ry="7" fill="#D84070" transform="rotate(-18 5 52)"/>
                                <path d="M48 79 Q26 68 10 56" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                // branch 3 — upper-left
                                <path d="M50 70 Q34 52 22 34" fill="none" stroke="#E05080" stroke-width="8" stroke-linecap="round"/>
                                <ellipse cx="18" cy="28" rx="11" ry="7" fill="#E05080" transform="rotate(-32 18 28)"/>
                                <path d="M50 70 Q34 52 22 34" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                // branch 4 — upward
                                <path d="M54 62 Q46 42 42 20" fill="none" stroke="#D84070" stroke-width="8" stroke-linecap="round"/>
                                <ellipse cx="40" cy="13" rx="11" ry="7" fill="#D84070" transform="rotate(-6 40 13)"/>
                                <path d="M54 62 Q46 42 42 20" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                // branch 5 — inner upward
                                <path d="M62 57 Q62 36 68 16" fill="none" stroke="#D84070" stroke-width="7" stroke-linecap="round"/>
                                <ellipse cx="69" cy="9" rx="10" ry="7" fill="#D84070"/>
                                <path d="M62 57 Q62 36 68 16" fill="none" stroke="#F4A0B8" stroke-width="3" stroke-linecap="round" opacity="0.4"/>
                            </g>

                            // === RIGHT GILLS (5-branch fan, mirrored) ===
                            <g class="gill-right">
                                <ellipse cx="170" cy="82" rx="11" ry="9" fill="#D84070"/>
                                // branch 1
                                <path d="M172 88 Q196 86 212 82" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                <ellipse cx="216" cy="81" rx="11" ry="7" fill="#D84070" transform="rotate(-8 216 81)"/>
                                <path d="M172 88 Q196 86 212 82" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                // branch 2
                                <path d="M172 79 Q194 68 210 56" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                <ellipse cx="215" cy="52" rx="11" ry="7" fill="#D84070" transform="rotate(18 215 52)"/>
                                <path d="M172 79 Q194 68 210 56" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                // branch 3
                                <path d="M170 70 Q186 52 198 34" fill="none" stroke="#E05080" stroke-width="8" stroke-linecap="round"/>
                                <ellipse cx="202" cy="28" rx="11" ry="7" fill="#E05080" transform="rotate(32 202 28)"/>
                                <path d="M170 70 Q186 52 198 34" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                // branch 4
                                <path d="M166 62 Q174 42 178 20" fill="none" stroke="#D84070" stroke-width="8" stroke-linecap="round"/>
                                <ellipse cx="180" cy="13" rx="11" ry="7" fill="#D84070" transform="rotate(6 180 13)"/>
                                <path d="M166 62 Q174 42 178 20" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                // branch 5
                                <path d="M158 57 Q158 36 152 16" fill="none" stroke="#D84070" stroke-width="7" stroke-linecap="round"/>
                                <ellipse cx="151" cy="9" rx="10" ry="7" fill="#D84070"/>
                                <path d="M158 57 Q158 36 152 16" fill="none" stroke="#F4A0B8" stroke-width="3" stroke-linecap="round" opacity="0.4"/>
                            </g>

                            // === FRONT LEGS ===
                            <path d="M76 154 Q58 166 52 182" fill="none" stroke="#F4A0B8"
                                  stroke-width="11" stroke-linecap="round"/>
                            <ellipse cx="49" cy="186" rx="10" ry="6" fill="#F4A0B8"/>
                            <path d="M146 154 Q164 166 170 182" fill="none" stroke="#F4A0B8"
                                  stroke-width="11" stroke-linecap="round"/>
                            <ellipse cx="173" cy="186" rx="10" ry="6" fill="#F4A0B8"/>

                            // === EYES ===
                            <circle cx="84" cy="98" r="19" fill="white"/>
                            <circle cx="136" cy="98" r="19" fill="white"/>
                            <circle cx="86" cy="100" r="14" fill="#6D2040"/>
                            <circle cx="138" cy="100" r="14" fill="#6D2040"/>
                            <circle cx="87" cy="101" r="8" fill="#1a0810"/>
                            <circle cx="139" cy="101" r="8" fill="#1a0810"/>
                            <circle cx="78" cy="92" r="6" fill="white"/>
                            <circle cx="130" cy="92" r="6" fill="white"/>
                            <circle cx="91" cy="108" r="2.5" fill="white" opacity="0.55"/>
                            <circle cx="143" cy="108" r="2.5" fill="white" opacity="0.55"/>

                            // === FRECKLES ===
                            <circle cx="96" cy="78" r="2" fill="#E080A0" opacity="0.35"/>
                            <circle cx="110" cy="72" r="2" fill="#E080A0" opacity="0.35"/>
                            <circle cx="124" cy="78" r="2" fill="#E080A0" opacity="0.35"/>
                            <circle cx="104" cy="82" r="1.5" fill="#E080A0" opacity="0.3"/>
                            <circle cx="116" cy="82" r="1.5" fill="#E080A0" opacity="0.3"/>

                            // === NOSE ===
                            <circle cx="106" cy="112" r="2.5" fill="#C07090" opacity="0.5"/>
                            <circle cx="114" cy="112" r="2.5" fill="#C07090" opacity="0.5"/>

                            // === MOUTH (subtle smirk) ===
                            <path d="M100 122 Q110 130 120 124" fill="none" stroke="#C06080"
                                  stroke-width="2.5" stroke-linecap="round"/>
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
