use leptos::prelude::*;

#[component]
pub fn ClamSvg(#[prop(default = 48)] size: u32) -> impl IntoView {
    view! {
        <svg width=size height=size viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg"
             class="creature-svg clam-svg">
            // bottom shell
            <path d="M8 36 Q8 56 32 58 Q56 56 56 36 Z"
                  fill="var(--clam-primary)" stroke="var(--clam-secondary)" stroke-width="1"/>
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
                      fill="var(--clam-primary)" stroke="var(--clam-secondary)" stroke-width="1"/>
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
                      stroke="#2a1000" stroke-width="1"/>
                // stripe mid-body
                <rect x="36" y="6" width="9" height="52" fill="white"/>
                <rect x="36" y="6" width="9" height="52" fill="none"
                      stroke="#2a1000" stroke-width="1"/>
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
        <svg width=size height=size viewBox="0 0 70 108" xmlns="http://www.w3.org/2000/svg"
             class="creature-svg ponyo-svg">

            // === BODY ===
            // main body — bigger
            <ellipse cx="35" cy="84" rx="22" ry="26" fill="#E05070"/>
            // wiggly bottom edge
            <path d="M13 104 Q18 116 24 104 Q30 116 36 104 Q42 116 48 104 Q52 114 57 104 L13 104 Z"
                  fill="#E05070"/>
            // white belly — smaller
            <ellipse cx="35" cy="83" rx="10" ry="12" fill="white" opacity="0.88"/>

            // fins as hands — left and right
            <ellipse cx="11" cy="82" rx="5" ry="8" fill="#E05070" transform="rotate(50 11 82)"/>
            <ellipse cx="59" cy="82" rx="5" ry="8" fill="#E05070" transform="rotate(-50 59 82)"/>

            // === HAIR (drawn first, face circle covers center seam) ===
            // hair mass behind head — big orange-red blob
            <ellipse cx="35" cy="30" rx="30" ry="26" fill="#C83020"/>
            // top spiky clumps
            <ellipse cx="25" cy="14" rx="7" ry="9" fill="#D03828" transform="rotate(-20 25 14)"/>
            <ellipse cx="35" cy="10" rx="7" ry="10" fill="#C83020"/>
            <ellipse cx="45" cy="14" rx="7" ry="9" fill="#D03828" transform="rotate(20 45 14)"/>

            // === FACE (drawn on top — no stroke to avoid seam lines) ===
            // cheek ellipse — wider at bottom, takes bottom half of head
            <ellipse cx="35" cy="53" rx="30" ry="20" fill="#e5c193"/>
            // round head — covers hair center, sits on cheeks
            <ellipse cx="35" cy="38" rx="30" ry="26" fill="#e5c193"/>

            // fringe — bangs drawn over face, hangs onto forehead
            <path d="M9 16 Q15 24 22 18 Q28 26 35 19 Q42 26 48 18 Q55 24 61 16 Q50 10 35 8 Q20 10 9 16 Z"
                  fill="#C83020"/>

            // eyes
            <circle cx="12" cy="40" r="6.5" fill="white" stroke="#1a1020" stroke-width="1"/>
            <circle cx="58" cy="40" r="6.5" fill="white" stroke="#1a1020" stroke-width="1"/>
            <circle cx="14" cy="39" r="3" fill="#1a1020"/>
            <circle cx="56" cy="39" r="3" fill="#1a1020"/>

            // nose
            <path d="M33 44 Q35 41 37 44" fill="none" stroke="#a07040"
                  stroke-width="1.5" stroke-linecap="round" opacity="0.7"/>

            // mouth — small diamond
            <path d="M35 59 L38 62 L35 65 L32 62 Z" fill="#1a1020"/>
        </svg>
    }
}
