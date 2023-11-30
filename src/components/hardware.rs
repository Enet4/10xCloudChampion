//! Module for hardware and overall computational power and load indicators

use yew::prelude::*;

use crate::{
    audio::play_zip_click,
    central::engine::{BARE_NODE_COST, UPGRADED_NODE_COST, UPGRADED_RACK_COST},
    components::load_bar::LoadBar,
    Memory, Money, PlayerAction,
};

/// The number of nodes that fit in a rack
pub(crate) const RACK_CAPACITY: u32 = 4;
pub(crate) const DATACENTER_CAPACITY: u32 = 32;

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

/// Base properties of a node component.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NodeProps {
    /// the node's unique ID
    pub id: u32,
    /// number of CPU cores in the node
    pub num_cores: u32,
    /// the node's total RAM capacity
    pub ram_capacity: Memory,
    /// whether the node is in powersave mode
    pub powersave: bool,
    /// the cost for the next CPU upgrade
    /// (or None if no upgrade is available)
    pub cpu_upgrade_cost: Option<Money>,
    /// the cost for the next RAM upgrade
    /// (or None if no upgrade is available)
    pub ram_upgrade_cost: Option<Money>,
}

/// Props for a Cloud Node component
/// which can be upgraded individually.
#[derive(Debug, PartialEq, Properties)]
pub struct UpgradableNodeProps {
    /// the node's unique ID
    pub id: u32,
    /// number of CPU cores in the node
    pub num_cores: u32,
    /// the node's total RAM capacity
    pub ram_capacity: Memory,
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
pub fn UpgradableNode(props: &UpgradableNodeProps) -> Html {
    let cores = if props.num_cores == 1 {
        "core"
    } else {
        "cores"
    };

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
            <span class="specs">{props.num_cores} {" "} {cores} {", "} {props.ram_capacity} {" RAM"}</span>
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
    pub nodes: Vec<NodeProps>,
    pub powersave: bool,
    pub on_player_action: Callback<PlayerAction>,
}

/// A rack of nodes
#[function_component]
pub fn OpenRack(props: &RackProps) -> Html {
    let can_buy_more_nodes = props.can_buy_nodes && (props.nodes.len() as u32) < RACK_CAPACITY;
    let purchase_button = if can_buy_more_nodes {
        let on_player_action = props.on_player_action.clone();
        let (action, disabled) = if !props.can_buy_racks {
            (PlayerAction::AddNode, props.funds < BARE_NODE_COST)
        } else {
            (
                PlayerAction::AddUpgradedNode,
                props.funds < UPGRADED_NODE_COST,
            )
        };
        let onclick = move |_| on_player_action.emit(action.clone());
        html! {
            <>
                <button {onclick} disabled={disabled}>
                    {"Buy node"}
                </button>
                {" "}
                if !props.can_buy_racks {
                    <span class="small">{BARE_NODE_COST.to_string()}</span>
                } else {
                    <span class="small">{UPGRADED_NODE_COST.to_string()}</span>
                }
            </>
        }
    } else {
        html! {}
    };
    let powersave = props.powersave;

    let nodes: Html = props
        .nodes
        .iter()
        .map(|node| {
            let cpu_upgrade_cost = node.cpu_upgrade_cost;
            let ram_upgrade_cost = node.ram_upgrade_cost;
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
                <UpgradableNode
                    id={node.id}
                    num_cores={node.num_cores} ram_capacity={node.ram_capacity}
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

/// Properties for the Equipment component
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EquipmentProps {
    pub nodes: Vec<NodeProps>,
    pub can_buy_nodes: bool,
    pub can_buy_racks: bool,
    pub can_buy_datacenters: bool,
    pub funds: Money,
    pub powersave: bool,
    pub on_player_action: Callback<PlayerAction>,
}

/// UI component for the whole equipment panel
#[derive(Debug)]
pub struct Equipment;

impl Component for Equipment {
    type Message = ();
    type Properties = EquipmentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let powersave = ctx.props().powersave;
        let can_buy_racks = ctx.props().can_buy_racks;
        let can_buy_datacenters = ctx.props().can_buy_datacenters;

        match (can_buy_racks, can_buy_datacenters) {
            (false, false) => {
                let nodes = ctx.props().nodes.clone();
                html! {
                    <div class="equipment">
                        <OpenRack
                            nodes={nodes}
                            can_buy_nodes={ctx.props().can_buy_nodes}
                            can_buy_racks={false}
                            funds={ctx.props().funds}
                            powersave={powersave}
                            on_player_action={ctx.props().on_player_action.clone()}
                        />
                    </div>
                }
            }
            (true, false) => {
                // show closed racks instead
                let racks: Html = ctx
                    .props()
                    .nodes
                    .chunks(RACK_CAPACITY as usize)
                    .map(|nodes| {
                        html! {
                            <div class="closed-rack">
                                <div class="closed-rack-inner">
                                    {nodes.iter().map(|node| {
                                        html! {
                                            <CloudNodeIcon powersave={node.powersave} />
                                        }
                                    }).collect::<Html>()}
                                </div>
                            </div>
                        }
                    })
                    .collect();

                html! {
                    <div class="equipment">
                        {racks}
                        // show buy button if available
                        // (first office only has room for 10 racks)
                        if ctx.props().nodes.len() < (10 * RACK_CAPACITY) as usize {
                            <div class="buy">
                                <button onclick={ctx.props().on_player_action.reform(|_| {
                                    play_zip_click();
                                    PlayerAction::AddUpgradedNode
                                })}>
                                    {"Buy node"}
                                </button>
                                <span>
                                    {UPGRADED_NODE_COST.to_string()}
                                </span>
                            </div>
                        } else if ctx.props().can_buy_datacenters {
                            <div class="buy">
                                <button onclick={ctx.props().on_player_action.reform(|_| {
                                    play_zip_click();
                                    PlayerAction::AddRack
                                })}>
                                    {"Buy rack"}
                                </button>
                                <span>
                                    {UPGRADED_RACK_COST.to_string()}
                                </span>
                            </div>
                        }
                    </div>
                }
            }
            (_, true) => {
                // show closed datacenters instead,
                // and each node is actually a rack
                let datacenters: Html = ctx
                    .props()
                    .nodes
                    .chunks(DATACENTER_CAPACITY as usize)
                    .map(|nodes| {
                        let num_racks = nodes.len() as u32;
                        let num_nodes = num_racks * RACK_CAPACITY;
                        let rack_count: Html = if num_racks == 1 {
                            html! { <span>{num_nodes} {" nodes, 1 rack"}</span> }
                        } else {
                            html! { <span>{num_nodes} {" nodes, "} {num_racks} {" racks"}</span> }
                        };
                        let leds = if ctx.props().powersave {
                            classes!["datacenter-led", "led-powersave"]
                        } else {
                            classes!["datacenter-led", "led-ok"]
                        };
                        html! {
                            <div class="datacenter-container">
                                <div class="datacenter-icon">
                                    <div class="datacenter-back"/>
                                    <div class="datacenter-front"/>
                                    <div class="datacenter-door"/>
                                    <div class={leds}/>
                                </div>
                                <div class="rack-count">
                                    {rack_count}
                                </div>
                            </div>
                        }
                    })
                    .collect();

                html! {
                    <div class="equipment">
                        {datacenters}
                        <div class="buy">
                            <button onclick={ctx.props().on_player_action.reform(|_| {
                                play_zip_click();
                                PlayerAction::AddRack
                            })}>
                                {"Buy rack"}
                            </button>
                            <span>
                                {UPGRADED_RACK_COST.to_string()}
                            </span>
                        </div>
                    </div>
                }
            }
        }
    }
}
