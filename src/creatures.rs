use leptos::prelude::*;

#[component]
pub fn ClamSvg(#[prop(default = 48)] size: u32) -> impl IntoView {
    view! {
        <svg width=size height=size viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg"
             class="creature-svg clam-svg">
            // bottom shell
            <path d="M8 36 Q8 56 32 58 Q56 56 56 36 Z"
                  fill="var(--clam-primary)" stroke="var(--clam-secondary)" stroke-width="1.5"/>
            // bottom shell ridges
            <path d="M16 40 Q24 52 32 54" fill="none" stroke="var(--clam-secondary)"
                  stroke-width="1" opacity="0.5"/>
            <path d="M32 54 Q40 52 48 40" fill="none" stroke="var(--clam-secondary)"
                  stroke-width="1" opacity="0.5"/>
            <path d="M22 38 Q28 48 32 50" fill="none" stroke="var(--clam-secondary)"
                  stroke-width="1" opacity="0.4"/>
            // top shell
            <g class="clam-top-shell">
                <path d="M8 36 Q8 14 32 10 Q56 14 56 36 Z"
                      fill="var(--clam-primary)" stroke="var(--clam-secondary)" stroke-width="1.5"/>
                // top shell ridges
                <path d="M16 32 Q24 18 32 14" fill="none" stroke="var(--clam-secondary)"
                      stroke-width="1" opacity="0.5"/>
                <path d="M32 14 Q40 18 48 32" fill="none" stroke="var(--clam-secondary)"
                      stroke-width="1" opacity="0.5"/>
                <path d="M22 34 Q28 22 32 18" fill="none" stroke="var(--clam-secondary)"
                      stroke-width="1" opacity="0.4"/>
            </g>
            // inner dark gap (mouth area)
            <ellipse cx="32" cy="36" rx="20" ry="4" fill="var(--ocean-abyss)" opacity="0.4"/>
            // eyes peeking from gap
            <g class="clam-eyes">
                <circle cx="24" cy="34" r="4.5" fill="white"/>
                <circle cx="40" cy="34" r="4.5" fill="white"/>
                <circle cx="25" cy="33.5" r="2.5" fill="var(--ocean-abyss)"/>
                <circle cx="41" cy="33.5" r="2.5" fill="var(--ocean-abyss)"/>
                <circle cx="25.8" cy="32.8" r="1" fill="white"/>
                <circle cx="41.8" cy="32.8" r="1" fill="white"/>
            </g>
            // pearl shimmer on top shell
            <circle cx="32" cy="22" r="3" fill="var(--pearl)" opacity="0.5"/>
        </svg>
    }
}

#[component]
pub fn ClamIcon() -> impl IntoView {
    view! { <ClamSvg size=36 /> }
}

#[component]
pub fn ClownFishSvg(#[prop(default = 64)] size: u32) -> impl IntoView {
    view! {
        <svg width=size height=size viewBox="0 0 80 64" xmlns="http://www.w3.org/2000/svg"
             class="creature-svg clownfish-svg">
            <defs>
                <clipPath id="cf-body-clip">
                    <ellipse cx="41" cy="32" rx="27" ry="22"/>
                </clipPath>
            </defs>

            // forked tail — traces outer edge, notch, back; no self-intersect
            <path d="M16 27
                     C10 22 2 14 2 10
                     C4 16 10 26 12 32
                     C10 38 4 48 2 54
                     C2 50 10 42 16 37
                     Z"
                  fill="#E8650A" stroke="#2a1000" stroke-width="1.5"
                  stroke-linejoin="round"/>

            // body — orange
            <ellipse cx="41" cy="32" rx="27" ry="22"
                     fill="#F4821F" stroke="#2a1000" stroke-width="2"/>

            // white stripes clipped to body shape
            <g clip-path="url(#cf-body-clip)">
                // stripe near head
                <rect x="53" y="6" width="14" height="52" fill="white"/>
                <rect x="53" y="6" width="14" height="52" fill="none"
                      stroke="#2a1000" stroke-width="1.5"/>
                // stripe mid-body
                <rect x="36" y="6" width="9" height="52" fill="white"/>
                <rect x="36" y="6" width="9" height="52" fill="none"
                      stroke="#2a1000" stroke-width="1.5"/>
            </g>

            // body outline redrawn on top of stripes
            <ellipse cx="41" cy="32" rx="27" ry="22"
                     fill="none" stroke="#2a1000" stroke-width="2"/>

            // dorsal fin
            <path d="M34 10 Q40 1 51 3 Q55 10 53 13"
                  fill="#F4821F" stroke="#2a1000" stroke-width="1.4"/>

            // bottom fin
            <path d="M34 54 Q40 63 49 61 Q52 54 50 51"
                  fill="#F4821F" stroke="#2a1000" stroke-width="1.4"/>

            // pectoral fin
            <path d="M49 38 Q61 48 58 54 Q49 49 47 40 Z"
                  fill="#E8650A" stroke="#2a1000" stroke-width="1.2"/>

            // eye — large and expressive
            <circle cx="64" cy="25" r="7.5" fill="white" stroke="#2a1000" stroke-width="1.8"/>
            <circle cx="65" cy="25" r="5" fill="#1a4a7a"/>
            <circle cx="65.5" cy="25" r="2.8" fill="#050d1a"/>
            <circle cx="67" cy="22.5" r="1.8" fill="white"/>

            // smile
            <path d="M60 32 Q64 36 68 32"
                  fill="none" stroke="#2a1000" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
    }
}

#[component]
pub fn PonyoSvg(#[prop(default = 64)] size: u32) -> impl IntoView {
    view! {
        <svg width=size height=size viewBox="0 0 70 82" xmlns="http://www.w3.org/2000/svg"
             class="creature-svg ponyo-svg">

            // === BODY ===
            <ellipse cx="35" cy="74" rx="14" ry="7" fill="#E05070"/>
            <path d="M24 78 Q35 84 46 78 Q40 72 35 73 Q30 72 24 78 Z" fill="#C84060"/>
            <path d="M23 70 Q35 76 47 70 Q43 64 35 65 Q27 64 23 70 Z" fill="white" opacity="0.9"/>

            // === HAIR (drawn first, face circle covers center seam) ===
            // hair mass behind head — big orange-red blob
            <ellipse cx="35" cy="30" rx="24" ry="22" fill="#C83020"/>
            // side hangs — hair coming down past face
            <ellipse cx="13" cy="44" rx="8" ry="14" fill="#C83020"/>
            <ellipse cx="57" cy="44" rx="8" ry="14" fill="#C83020"/>
            // top spiky clumps
            <ellipse cx="25" cy="14" rx="7" ry="9" fill="#D03828" transform="rotate(-20 25 14)"/>
            <ellipse cx="35" cy="10" rx="7" ry="10" fill="#C83020"/>
            <ellipse cx="45" cy="14" rx="7" ry="9" fill="#D03828" transform="rotate(20 45 14)"/>

            // === FACE (drawn on top — no stroke to avoid seam lines) ===
            // cheek ellipse — wider at bottom
            <ellipse cx="35" cy="54" rx="22" ry="13" fill="#e5c193"/>
            // round head — covers hair center, sits on cheeks
            <circle cx="35" cy="38" r="20" fill="#e5c193"/>

            // rosy cheeks
            <ellipse cx="16" cy="52" rx="6" ry="4" fill="#e8907a" opacity="0.45"/>
            <ellipse cx="54" cy="52" rx="6" ry="4" fill="#e8907a" opacity="0.45"/>

            // eyebrows
            <path d="M21 31 Q27 27 32 30" fill="none" stroke="#7a2010"
                  stroke-width="2" stroke-linecap="round"/>
            <path d="M38 30 Q43 27 49 31" fill="none" stroke="#7a2010"
                  stroke-width="2" stroke-linecap="round"/>

            // eyes
            <circle cx="26" cy="40" r="7" fill="white"/>
            <circle cx="44" cy="40" r="7" fill="white"/>
            <circle cx="26" cy="41" r="4.5" fill="#1a1020"/>
            <circle cx="44" cy="41" r="4.5" fill="#1a1020"/>
            <circle cx="27.5" cy="39" r="1.8" fill="white"/>
            <circle cx="45.5" cy="39" r="1.8" fill="white"/>

            // nose
            <circle cx="33" cy="49" r="1.3" fill="#a07040" opacity="0.6"/>
            <circle cx="37" cy="49" r="1.3" fill="#a07040" opacity="0.6"/>

            // mouth
            <path d="M29 56 Q35 61 41 56"
                  fill="none" stroke="#a07040" stroke-width="1.8" stroke-linecap="round"/>
        </svg>
    }
}
