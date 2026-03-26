use leptos::prelude::*;
use crate::state::{ActivityId, AppState, Screen};
use crate::particles::UnderwaterScene;
use crate::creatures::{ClamIcon, ClownFishSvg, PonyoSvg, AnglerFishSvg, SebastianIcon};

#[component]
pub fn HubScreen() -> impl IntoView {
    let state = expect_context::<AppState>();

    let activities = ActivityId::all();

    let location_class = |id: ActivityId| {
        let base = match id {
            ActivityId::PearlWisdom => "location loc-pearl",
            ActivityId::CurrentRider => "location loc-current",
            ActivityId::CoralGarden => "location loc-coral",
            ActivityId::DeepSeaLights => "location loc-lights",
            ActivityId::TreasureHunt => "location loc-treasure",
        };
        move || {
            if state.is_completed(&id) {
                format!("{base} completed")
            } else {
                base.to_string()
            }
        }
    };

    let on_activity_click = move |id: ActivityId| {
        move |_| state.navigate(Screen::Activity(id))
    };

    let all_done = move || state.all_completed();

    let on_finale = move |_| {
        state.navigate(Screen::Finale);
    };

    let on_reset = move |_| {
        state.reset();
    };

    view! {
        <div class="screen hub-screen">
            <UnderwaterScene bubble_count=12 seaweed_count=6 fish_count=2 />

            <h2 class="hub-title">"Complete all 5 to unlock your surprise!"</h2>

            <div class="hub-locations">
                {activities
                    .into_iter()
                    .map(|id| {
                        view! {
                            <button
                                class=location_class(id)
                                on:click=on_activity_click(id)
                            >
                                <span class="loc-icon">{location_icon_view(id)}</span>
                                <span class="loc-label">{id.label()}</span>
                                <Show when=move || state.is_completed(&id)>
                                    <span class="loc-check">"✓"</span>
                                </Show>
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            <Show when=all_done>
                <button class="btn-primary finale-btn" on:click=on_finale>
                    "Something special is waiting..."
                </button>
            </Show>

            <button class="btn-secondary reset-btn" on:click=on_reset>
                "Start Over"
            </button>
        </div>
    }
}

fn location_icon_view(id: ActivityId) -> impl leptos::IntoView {
    use leptos::prelude::*;
    match id {
        ActivityId::PearlWisdom => view! { <ClamIcon /> }.into_any(),
        ActivityId::CurrentRider => view! { <ClownFishSvg size=36 /> }.into_any(),
        ActivityId::CoralGarden => view! { <PonyoSvg size=36 /> }.into_any(),
        ActivityId::DeepSeaLights => view! { <AnglerFishSvg size=36 /> }.into_any(),
        ActivityId::TreasureHunt => view! { <SebastianIcon /> }.into_any(),
    }
}
