//! ReactiveMotionDiv Test
//!
//! A minimal test to see if ReactiveMotionDiv works

use leptos::prelude::*;
use leptos_motion_dom::ReactiveMotionDiv;

#[component]
pub fn ReactiveMotionDivTest() -> impl IntoView {
    view! {
        <div>
            <h1>"ReactiveMotionDiv Test"</h1>
            <p>"Testing if ReactiveMotionDiv renders children"</p>
            <ReactiveMotionDiv>
                <div>
                    <h2>"This should be inside ReactiveMotionDiv"</h2>
                    <p>"If you can see this, ReactiveMotionDiv is working!"</p>
                </div>
            </ReactiveMotionDiv>
        </div>
    }
}
