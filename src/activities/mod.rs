mod pearl_wisdom;
mod current_rider;
mod coral_garden;
mod deep_sea_lights;
mod treasure_hunt;

pub use pearl_wisdom::PearlWisdomActivity;
pub use current_rider::CurrentRiderActivity;
pub use coral_garden::CoralGardenActivity;
pub use deep_sea_lights::DeepSeaLightsActivity;
pub use treasure_hunt::TreasureHuntActivity;

use leptos::prelude::*;
use crate::state::{ActivityId, AppState, Screen};
use crate::wishes;
use crate::particles::{OceanFloor, Seaweed};

#[component]
pub fn ActivityWrapper(id: ActivityId, children: Children) -> impl IntoView {
    let state = expect_context::<AppState>();
    let show_wish = RwSignal::new(false);

    provide_context(ActivityCompleteSignal(show_wish));

    let on_back = move |_| {
        state.navigate(Screen::Hub);
    };

    Effect::new(move |_| {
        if show_wish.get() {
            state.complete_activity(id);
        }
    });

    view! {
        <div class="screen activity-screen">
            <OceanFloor />
            <Seaweed count=5 />
            <div class="activity-header">
                <button class="btn-back" on:click=on_back>"< Back"</button>
                <h2 class="activity-title">{id.label()}</h2>
            </div>

            <div class="activity-content">
                {children()}
            </div>

            <Show when=move || show_wish.get()>
                <div class="wish-overlay">
                    <div class="wish-card">
                        <p class="wish-text">{wishes::wish_for(&id)}</p>
                        <button class="btn-primary" on:click=on_back>
                            "Back to Ocean"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[derive(Clone, Copy)]
pub struct ActivityCompleteSignal(RwSignal<bool>);

impl ActivityCompleteSignal {
    pub fn trigger(&self) {
        self.0.set(true);
    }
}
