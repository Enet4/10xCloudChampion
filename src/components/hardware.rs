//! Module for hardware and overall computational power and load indicators

use yew::prelude::*;

use crate::{components::load_bar::LoadBar, Memory, Money};

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

#[derive(Debug, PartialEq, Properties)]
pub struct NodeProps {
    /// number of CPUs in the node
    pub cpus: u32,
    /// RAM available in the node
    pub ram: Memory,
}

/// A node in the Cloud network
#[function_component]
pub fn Node(props: &NodeProps) -> Html {
    let cores = if props.cpus == 1 { "core" } else { "cores" };

    // TODO pass
    let upgrade_cost = Money::dollars(500);

    html! {
        <div class="node-container">
            <div class="node">
                // decorative lines
                <div class="lines" />
                // blinking light
                <div class="led" />
            </div>
            <span class="specs">{props.cpus} {" "} {cores} {", "} {props.ram} {" RAM"}</span>
            <div class="upgrade">
                <span>{upgrade_cost.to_string()}</span>
                <button>{"Upgrade"}</button>
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct RackProps {
    pub children: Html,
}

/// A rack of nodes
#[function_component]
pub fn Rack(props: &RackProps) -> Html {
    html! {
        <div class="rack">
            {props.children.clone()}
        </div>
    }
}
