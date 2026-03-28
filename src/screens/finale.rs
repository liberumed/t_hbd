use leptos::prelude::*;
use crate::state::{ActivityId, AppState};
use crate::wishes;
use crate::particles::UnderwaterScene;
use crate::creatures::{ClamIcon, ClownFishSvg, PonyoSvg, AnglerFishSvg, SebastianIcon};

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

            <div class="finale-reveal">
                <div class="axolotl-surprise">
                    <svg class="axolotl-svg" viewBox="-5 -14 256 278" xmlns="http://www.w3.org/2000/svg">
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

                            // === BACK LEGS ===
                            <g class="leg-back-left">
                                <path d="M96 222 Q92 236 90 246" fill="none" stroke="#F4A0B8"
                                      stroke-width="11" stroke-linecap="round"/>
                                <ellipse cx="89" cy="249" rx="10" ry="6" fill="#F4A0B8"/>
                            </g>
                            <g class="leg-back-right">
                                <path d="M126 218 Q136 230 140 242" fill="none" stroke="#F4A0B8"
                                      stroke-width="11" stroke-linecap="round"/>
                                <ellipse cx="142" cy="245" rx="10" ry="6" fill="#F4A0B8"/>
                            </g>

                            // === BODY ===
                            <ellipse cx="112" cy="178" rx="34" ry="50" fill="#F4A0B8"/>
                            <ellipse cx="112" cy="185" rx="21" ry="33" fill="#FAD0E0" opacity="0.6"/>

                            // === HEAD ===
                            <g class="axolotl-head">
                                // === LEFT GILLS (3-branch sideways, behind head) ===
                                <g class="gill-left">
                                    <ellipse cx="52" cy="78" rx="10" ry="8" fill="#D84070"/>
                                    // branch 1 — lower-left
                                    <path d="M50 84 Q28 88 10 90" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                    <ellipse cx="5" cy="90" rx="11" ry="7" fill="#D84070" transform="rotate(5 5 90)"/>
                                    <path d="M50 84 Q28 88 10 90" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                    // branch 2 — straight left
                                    <path d="M50 76 Q28 74 10 72" fill="none" stroke="#E05080" stroke-width="8" stroke-linecap="round"/>
                                    <ellipse cx="5" cy="72" rx="11" ry="7" fill="#E05080"/>
                                    <path d="M50 76 Q28 74 10 72" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                    // branch 3 — upper-left
                                    <path d="M50 68 Q30 60 14 54" fill="none" stroke="#D84070" stroke-width="7" stroke-linecap="round"/>
                                    <ellipse cx="9" cy="52" rx="11" ry="7" fill="#D84070" transform="rotate(-15 9 52)"/>
                                    <path d="M50 68 Q30 60 14 54" fill="none" stroke="#F4A0B8" stroke-width="3" stroke-linecap="round" opacity="0.4"/>
                                </g>

                                // === RIGHT GILLS (3-branch sideways, behind head) ===
                                <g class="gill-right">
                                    <ellipse cx="168" cy="78" rx="10" ry="8" fill="#D84070"/>
                                    // branch 1 — lower-right
                                    <path d="M170 84 Q192 88 210 90" fill="none" stroke="#D84070" stroke-width="9" stroke-linecap="round"/>
                                    <ellipse cx="215" cy="90" rx="11" ry="7" fill="#D84070" transform="rotate(-5 215 90)"/>
                                    <path d="M170 84 Q192 88 210 90" fill="none" stroke="#F4A0B8" stroke-width="4" stroke-linecap="round" opacity="0.45"/>
                                    // branch 2 — straight right
                                    <path d="M170 76 Q192 74 210 72" fill="none" stroke="#E05080" stroke-width="8" stroke-linecap="round"/>
                                    <ellipse cx="215" cy="72" rx="11" ry="7" fill="#E05080"/>
                                    <path d="M170 76 Q192 74 210 72" fill="none" stroke="#F4A0B8" stroke-width="3.5" stroke-linecap="round" opacity="0.4"/>
                                    // branch 3 — upper-right
                                    <path d="M170 68 Q190 60 206 54" fill="none" stroke="#D84070" stroke-width="7" stroke-linecap="round"/>
                                    <ellipse cx="211" cy="52" rx="11" ry="7" fill="#D84070" transform="rotate(15 211 52)"/>
                                    <path d="M170 68 Q190 60 206 54" fill="none" stroke="#F4A0B8" stroke-width="3" stroke-linecap="round" opacity="0.4"/>
                                </g>

                                <ellipse cx="110" cy="94" rx="70" ry="48" fill="#F4A0B8"/>
                                <ellipse cx="110" cy="74" rx="34" ry="15" fill="white" opacity="0.17"/>

                                // === EYES ===
                                <g class="axolotl-eyes">
                                    <circle cx="79" cy="100" r="10" fill="#1a0810"/>
                                    <circle cx="147" cy="100" r="10" fill="#1a0810"/>
                                    <circle cx="75" cy="96" r="3.5" fill="white"/>
                                    <circle cx="143" cy="96" r="3.5" fill="white"/>
                                </g>

                                // === FRECKLES ===
                                <circle cx="96" cy="78" r="2" fill="#E080A0" opacity="0.35"/>
                                <circle cx="110" cy="72" r="2" fill="#E080A0" opacity="0.35"/>
                                <circle cx="124" cy="78" r="2" fill="#E080A0" opacity="0.35"/>
                                <circle cx="104" cy="82" r="1.5" fill="#E080A0" opacity="0.3"/>
                                <circle cx="116" cy="82" r="1.5" fill="#E080A0" opacity="0.3"/>

                                // === NOSE ===
                                <circle cx="106" cy="112" r="2.5" fill="#C07090" opacity="0.5"/>
                                <circle cx="114" cy="112" r="2.5" fill="#C07090" opacity="0.5"/>

                                // === MOUTH (smirk — asymmetric, right corner up) ===
                                <path d="M107 128 Q112 130 118 126" fill="none" stroke="#C06080"
                                      stroke-width="2.5" stroke-linecap="round"/>
                            </g>

                            // === FRONT LEGS (shoulders) ===
                            <g class="arm-left">
                                <path d="M86 154 Q66 164 60 174" fill="none" stroke="#F4A0B8"
                                      stroke-width="11" stroke-linecap="round"/>
                                <ellipse cx="58" cy="178" rx="10" ry="6" fill="#F4A0B8"/>
                            </g>
                            <g class="arm-right">
                                <path d="M136 154 Q156 164 162 174" fill="none" stroke="#F4A0B8"
                                      stroke-width="11" stroke-linecap="round"/>
                                <ellipse cx="164" cy="178" rx="10" ry="6" fill="#F4A0B8"/>
                            </g>
                        </g>
                    </svg>
                </div>
                <h1 class="finale-title">{wishes::finale_message()}</h1>
            </div>

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
        ActivityId::DeepSeaLights  => view! { <AnglerFishSvg size=36 /> }.into_any(),
        ActivityId::TreasureHunt   => view! { <SebastianIcon /> }.into_any(),
    }
}
