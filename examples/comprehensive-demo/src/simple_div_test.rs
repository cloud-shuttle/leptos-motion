//! Simple Div Test
//!
//! A minimal test to see if regular divs work

use leptos::prelude::*;

#[component]
pub fn SimpleDivTest() -> impl IntoView {
    view! {
        <div>
            <h1>"Simple Div Test"</h1>
            <p>"This is a simple test to see if regular divs work"</p>
        </div>
    }
}
