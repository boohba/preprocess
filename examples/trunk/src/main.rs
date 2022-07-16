use yew::{function_component, html};

mod assets {
    include!(concat!(env!("OUT_DIR"), "/assets.rs"));
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <img alt="" src={assets::NEVER_GONNA_GIVE_YOU_UP_GIF}/>
    }
}

fn main() {
    yew::start_app::<App>();
}
