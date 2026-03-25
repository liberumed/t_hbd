use crate::state::ActivityId;

pub fn wish_for(activity: &ActivityId) -> &'static str {
    match activity {
        ActivityId::PearlWisdom => concat!(
            "May every year bring you pearls of wisdom — ",
            "the kind you earn through curiosity, courage, and laughter."
        ),
        ActivityId::CurrentRider => concat!(
            "Life will have currents that push and pull, ",
            "but you have the strength to find your own way. ",
            "Every detour is just a new adventure."
        ),
        ActivityId::CoralGarden => concat!(
            "You are like a coral reef — you make the world around you ",
            "more colorful and alive. Everything you build, everything you create, ",
            "brings joy to others."
        ),
        ActivityId::DeepSeaLights => concat!(
            "Even in the darkest depths, there is light — ",
            "and you carry that light inside you. ",
            "Your kindness, your smile, your energy lights up everyone around you."
        ),
        ActivityId::TreasureHunt => concat!(
            "The greatest treasure in life is not gold or gems — ",
            "it is the people who love you and the memories you make together. ",
            "Today, you are our greatest treasure."
        ),
    }
}

pub fn finale_message() -> &'static str {
    "Happy Birthday, Tymur!"
}
