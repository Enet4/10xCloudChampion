//! Module for hardware and overall computational power and load indicators

use yew::prelude::*;

use crate::{
    audio::play_zip_click,
    central::engine::{CloudNode, BARE_NODE_COST, UPGRADED_NODE_COST},
    components::load_bar::LoadBar,
    Memory, Money, PlayerAction,
};

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
    /// whether the node is in powersave mode
    pub powersave: bool,
    /// the cost for the next CPU upgrade
    /// (or None if no upgrade is available)
    pub cpu_upgrade_cost: Option<Money>,
    /// the cost for the next RAM upgrade
    /// (or None if no upgrade is available)
    pub ram_upgrade_cost: Option<Money>,
    /// whether the CPU upgrade can be afforded
    pub cpu_upgrade_disabled: bool,
    /// whether the RAM upgrade can be afforded
    pub ram_upgrade_disabled: bool,
    /// callback for when the CPU upgrade button is clicked
    pub on_cpu_upgrade: Callback<()>,
    /// callback for when the RAM upgrade button is clicked
    pub on_ram_upgrade: Callback<()>,
}

/// A node in the Cloud network
#[function_component]
pub fn Node(props: &NodeProps) -> Html {
    let cores = if props.cpus == 1 { "core" } else { "cores" };

    let on_cpu_upgrade = {
        let cb = props.on_cpu_upgrade.clone();
        move |_ev| {
            play_zip_click();
            cb.emit(())
        }
    };
    let on_ram_upgrade = {
        let cb = props.on_ram_upgrade.clone();
        move |_ev| {
            play_zip_click();
            cb.emit(())
        }
    };

    let cpu_enabled = if !props.cpu_upgrade_disabled {
        "true"
    } else {
        "false"
    };
    let ram_enabled = if !props.ram_upgrade_disabled {
        "true"
    } else {
        "false"
    };

    html! {
        <div class="node-container">
            <CloudNodeIcon powersave={props.powersave} />
            <span class="specs">{props.cpus} {" "} {cores} {", "} {props.ram} {" RAM"}</span>
            <div class="upgrade-container">
            if let Some(cost) = props.cpu_upgrade_cost {
                <div class="upgrade">
                    <span>{cost.to_string()}</span>
                    <button enabled={cpu_enabled} onclick={on_cpu_upgrade}>{"Upgrade CPU"}</button>
                </div>
            }
            if let Some(cost) = props.ram_upgrade_cost {
                <div class="upgrade">
                    <span>{cost.to_string()}</span>
                    <button enabled={ram_enabled} onclick={on_ram_upgrade}>{"Upgrade RAM"}</button>
                </div>
            }
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct CloudNodeIconProps {
    pub powersave: bool,
}

#[function_component]
pub fn CloudNodeIcon(props: &CloudNodeIconProps) -> Html {
    let node_classes = if props.powersave {
        classes!["led", "led-powersave"]
    } else {
        classes!["led", "led-ok"]
    };

    html! {
        <div class="node">
            // decorative lines
            <div class="lines" />
            // blinking light
            <div class={node_classes} />
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct RackProps {
    /// whether the ability to purchase more nodes is unlocked
    pub can_buy_nodes: bool,
    /// whether the ability to purchase more racks is unlocked
    /// (means all node purchases are for fully upgraded nodes)
    pub can_buy_racks: bool,
    pub funds: Money,
    pub nodes: Vec<CloudNode>,
    pub powersave: bool,
    pub on_player_action: Callback<PlayerAction>,
}

const RACK_CAPACITY: u32 = 4;

/// A rack of nodes
#[function_component]
pub fn Rack(props: &RackProps) -> Html {
    let can_buy_more_nodes = props.can_buy_nodes && (props.nodes.len() as u32) < RACK_CAPACITY;
    let purchase_button = if can_buy_more_nodes {
        let on_player_action = props.on_player_action.clone();
        let (action, disabled) = if props.can_buy_racks {
            (PlayerAction::AddNode, props.funds < BARE_NODE_COST)
        } else {
            (
                PlayerAction::AddUpgradedNode,
                props.funds < UPGRADED_NODE_COST,
            )
        };
        let onclick = move |_| on_player_action.emit(action.clone());
        html! {
            <button {onclick} {disabled}>
                {"Buy node"}
            </button>
        }
    } else {
        html! {}
    };
    let powersave = props.powersave;

    let nodes: Html = props
        .nodes
        .iter()
        .map(|node| {
            let cpu_upgrade_cost = node.next_cpu_upgrade_cost();
            let ram_upgrade_cost = node.next_ram_upgrade_cost();
            let cpu_upgrade_disabled = cpu_upgrade_cost
                .map(|cost| props.funds < cost)
                .unwrap_or_default();
            let ram_upgrade_disabled = ram_upgrade_cost
                .map(|cost| props.funds < cost)
                .unwrap_or_default();
            let on_cpu_upgrade = {
                let on_player_action = props.on_player_action.clone();
                let node = node.id;
                move |_| on_player_action.emit(PlayerAction::UpgradeCpu { node })
            };
            let on_ram_upgrade = {
                let on_player_action = props.on_player_action.clone();
                let node = node.id;
                move |_| on_player_action.emit(PlayerAction::UpgradeRam { node })
            };
            html! {
                <Node
                    cpus={node.num_cores} ram={node.ram_capacity}
                    {powersave}
                    {cpu_upgrade_cost}
                    {ram_upgrade_cost}
                    {cpu_upgrade_disabled}
                    {ram_upgrade_disabled}
                    {on_cpu_upgrade}
                    {on_ram_upgrade}
                    />
            }
        })
        .collect();

    html! {
        <div class="rack">
            {nodes}
            {purchase_button}
        </div>
    }
}
