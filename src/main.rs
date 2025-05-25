use leptos::prelude::*;
use leptos_styling::inline_style_sheet;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}

inline_style_sheet! {red_button, "red_button",
    .button {
        background-color: red;
        padding: 8px 16px;
        border-radius: 4px;
    }
}

#[component]
pub fn MyButtons() -> impl IntoView {
    view! {
        <div>
            <button class=red_button::BUTTON>"Red Button"</button>
        </div>
    }
}
