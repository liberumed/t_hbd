use leptos::prelude::*;
use rand::Rng;

#[component]
pub fn Bubbles(#[prop(default = 20)] count: usize) -> impl IntoView {
    let bubbles: Vec<_> = (0..count)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let size = rng.gen_range(4..20);
            let left = rng.gen_range(0..100);
            let delay = rng.gen_range(0.0..10.0_f64);
            let duration = rng.gen_range(6.0..14.0_f64);

            let style = format!(
                "width: {size}px; height: {size}px; left: {left}%; \
                 animation-delay: {delay:.1}s; animation-duration: {duration:.1}s;"
            );

            view! { <div class="bubble" style=style></div> }
        })
        .collect();

    view! { <div class="bubbles-container">{bubbles}</div> }
}

#[component]
pub fn Seaweed(#[prop(default = 6)] count: usize) -> impl IntoView {
    let plants: Vec<_> = (0..count)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let left = rng.gen_range(2..98);
            let height = rng.gen_range(60..160);
            let delay = rng.gen_range(0.0..3.0_f64);
            let duration = rng.gen_range(3.0..6.0_f64);
            let hue_shift = rng.gen_range(-20..20);
            let width = rng.gen_range(20..40);

            let style = format!(
                "left: {left}%; height: {height}px; width: {width}px; \
                 animation-delay: {delay:.1}s; animation-duration: {duration:.1}s; \
                 filter: hue-rotate({hue_shift}deg);"
            );

            view! {
                <svg class="seaweed" style=style viewBox="0 0 40 160" xmlns="http://www.w3.org/2000/svg">
                    <path d="M20 160 Q10 120 20 100 Q30 80 20 60 Q10 40 20 20 Q25 10 20 0"
                          fill="none" stroke="var(--seaweed-dark)" stroke-width="6"
                          stroke-linecap="round" opacity="0.8"/>
                    <path d="M20 130 Q5 110 8 90" fill="none" stroke="var(--seaweed-light)"
                          stroke-width="4" stroke-linecap="round" opacity="0.6"/>
                    <path d="M20 90 Q35 70 32 50" fill="none" stroke="var(--seaweed-light)"
                          stroke-width="3" stroke-linecap="round" opacity="0.5"/>
                </svg>
            }
        })
        .collect();

    view! { <div class="seaweed-container">{plants}</div> }
}

#[component]
pub fn LightRays() -> impl IntoView {
    let rays: Vec<_> = (0..5)
        .map(|i| {
            let mut rng = rand::thread_rng();
            let left = 10 + i * 20 + rng.gen_range(-5..5_i32);
            let width = rng.gen_range(30..80);
            let delay = rng.gen_range(0.0..4.0_f64);
            let duration = rng.gen_range(4.0..8.0_f64);
            let opacity = rng.gen_range(0.03..0.08_f64);

            let style = format!(
                "left: {left}%; width: {width}px; \
                 animation-delay: {delay:.1}s; animation-duration: {duration:.1}s; \
                 opacity: {opacity:.2};"
            );

            view! { <div class="light-ray" style=style></div> }
        })
        .collect();

    view! { <div class="light-rays-container">{rays}</div> }
}

#[component]
pub fn SwimmingFish(#[prop(default = 3)] count: usize) -> impl IntoView {
    let fish: Vec<_> = (0..count)
        .map(|_| {
            let mut rng = rand::thread_rng();
            let top = rng.gen_range(10..70);
            let delay = rng.gen_range(0.0..15.0_f64);
            let duration = rng.gen_range(12.0..25.0_f64);
            let size = rng.gen_range(16..30);
            let style = format!(
                "top: {top}%; animation-delay: {delay:.1}s; \
                 animation-duration: {duration:.1}s; \
                 font-size: {size}px;"
            );

            view! {
                <div class="swimming-fish" style=style>
                    <svg viewBox="0 0 40 24" width="40" height="24" xmlns="http://www.w3.org/2000/svg">
                        <ellipse cx="22" cy="12" rx="14" ry="8" fill="var(--fish-primary)" opacity="0.7"/>
                        <polygon points="8,12 0,6 0,18" fill="var(--fish-secondary)" opacity="0.6"/>
                        <circle cx="30" cy="10" r="2" fill="var(--ocean-abyss)" opacity="0.8"/>
                    </svg>
                </div>
            }
        })
        .collect();

    view! { <div class="fish-container">{fish}</div> }
}

#[component]
pub fn OceanFloor() -> impl IntoView {
    let mut rng = rand::thread_rng();

    // Rocks: (cx, cy, rx, ry, opacity)
    let rocks: Vec<_> = (0..10).map(|_| {
        let cx = rng.gen_range(30_u32..1170);
        let cy = rng.gen_range(108_u32..118);
        let rx = rng.gen_range(10_u32..28);
        let ry = rng.gen_range(5_u32..12);
        let op = rng.gen_range(40_u32..75);
        (cx, cy, rx, ry, op)
    }).collect();

    // Seaweed tufts: (x, blade_height, lean)
    let tufts: Vec<_> = (0..14).map(|_| {
        let x = rng.gen_range(20_u32..1180);
        let h = rng.gen_range(18_u32..40);
        let lean: i32 = rng.gen_range(-6..6);
        (x, h, lean)
    }).collect();

    view! {
        <div class="ocean-floor">
            <svg class="floor-svg" viewBox="0 0 1200 120" preserveAspectRatio="none"
                 xmlns="http://www.w3.org/2000/svg">

                // sand layers
                // sand layers
                <rect x="0" y="85" width="1200" height="35" fill="var(--sand-dark)"/>
                <path d="M0 80 Q100 60 200 70 Q300 80 400 65 Q500 50 600 70 Q700 90 800 65 Q900 50 1000 75 Q1100 85 1200 70 L1200 120 L0 120 Z"
                      fill="var(--sand-mid)"/>
                <path d="M0 90 Q150 75 300 85 Q450 95 600 80 Q750 70 900 85 Q1050 95 1200 80 L1200 120 L0 120 Z"
                      fill="var(--sand-dark)"/>

                // rocks
                {rocks.into_iter().map(|(cx, cy, rx, ry, op)| {
                    let style = format!("opacity: 0.{op}");
                    view! {
                        <ellipse cx=cx cy=cy rx=rx ry=ry
                                 fill="var(--ocean-floor)" style=style />
                    }
                }).collect::<Vec<_>>()}

                // seaweed tufts
                {tufts.into_iter().map(|(x, h, lean)| {
                    let tip_x = x as i32 + lean;
                    let tip_y = 120 - h as i32;
                    let mid_x = x as i32 + lean / 2;
                    let mid_y = 120 - h as i32 / 2;
                    let l_x = x as i32 - 5 + lean;
                    let r_x = x as i32 + 5 + lean;
                    view! {
                        <path d=format!("M{x} 120 Q{mid_x} {mid_y} {tip_x} {tip_y}")
                              fill="none" stroke="var(--seaweed-dark)"
                              stroke-width="2.5" stroke-linecap="round" opacity="0.7"/>
                        <path d=format!("M{x} 118 Q{l_x} {mid_y} {} {}", l_x - 2, tip_y + 6)
                              fill="none" stroke="var(--seaweed-light)"
                              stroke-width="2" stroke-linecap="round" opacity="0.5"/>
                        <path d=format!("M{x} 118 Q{r_x} {mid_y} {} {}", r_x + 2, tip_y + 6)
                              fill="none" stroke="var(--seaweed-light)"
                              stroke-width="2" stroke-linecap="round" opacity="0.5"/>
                    }
                }).collect::<Vec<_>>()}
            </svg>
        </div>
    }
}

#[component]
pub fn UnderwaterScene(
    #[prop(default = 15)] bubble_count: usize,
    #[prop(default = 5)] seaweed_count: usize,
    #[prop(default = 3)] fish_count: usize,
    #[prop(default = true)] show_light_rays: bool,
    #[prop(default = true)] show_floor: bool,
) -> impl IntoView {
    view! {
        <Bubbles count=bubble_count />
        {show_light_rays.then(|| view! { <LightRays /> })}
        <SwimmingFish count=fish_count />
        {show_floor.then(|| view! { <OceanFloor /> })}
        <Seaweed count=seaweed_count />
    }
}
