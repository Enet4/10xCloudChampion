//! Module for the computational load bar
//!

use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct LoadBarProps {
    /// the current load level between 0 and 1
    pub load: f32,
}

#[function_component]
pub fn LoadBar(props: &LoadBarProps) -> Html {
    html! {
        <div class="load-bar">
            <div class="load-bar-inner" />
            <div class="load-bar-cover" style={format!("left:{}%", (props.load * 100.) as i32)}/>
        </div>
    }
}
