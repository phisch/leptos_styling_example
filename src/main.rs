use leptos::prelude::*;
use leptos_styling::{StyleSheets, inline_style_sheet};

fn main() {
    console_error_panic_hook::set_once();
    leptos_styling::init();
    leptos::mount::mount_to_body(|| view! { <StyleSheets/><MyButtons/> })
}

inline_style_sheet! {blue_button, "blue_button",
    .button {
        background-color: red;
    }
}

#[component]
pub fn MyButtons() -> impl IntoView {
    view! {
        <div>
            <button class=blue_button::BUTTON>"Red Button"</button>
        </div>
    }
}
