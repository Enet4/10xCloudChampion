//! Module for hardware and overall computational power and load indicators

use yew::prelude::*;

use crate::{components::load_bar::LoadBar, Memory};

#[derive(Debug, PartialEq, Properties)]
pub struct PowerProps {
    /// the current CPU load between 0 and 1
    pub cpu_load: f32,
    /// the current memory load between 0 and 1
    pub mem_load: f32,
    /// the total memory available
    pub mem_total: Memory,
}

/// An indicator of the total CPU and Memory usage
#[function_component]
pub fn Power(props: &PowerProps) -> Html {
    let memory_used = props.mem_total * props.mem_load;

    html! {
        <div class="power">
            <div class="power-cpu">
                {"CPU: "} {(props.cpu_load * 100.).round()} {"%"} <LoadBar load={props.cpu_load}/>
            </div>
            <div class="power-mem">
                {"Memory: "} {memory_used} {"/"} {props.mem_total} <LoadBar load={props.mem_load}/>
            </div>
        </div>
    }
}
