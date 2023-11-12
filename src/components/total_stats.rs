//! Module for the total stats
//! (all operations done so far)
use yew::prelude::*;

use crate::Ops;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct TotalStatsProps {
    /// the total op count for the base service
    pub base_ops_total: Ops,

    /// the total op count for the super service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub super_ops_total: Option<Ops>,

    /// the total op count for the epic service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub epic_ops_total: Option<Ops>,

    /// the total op count for the awesome service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub awesome_ops_total: Option<Ops>,
}

/// The stats component.

#[function_component]
pub fn TotalStats(props: &TotalStatsProps) -> Html {
    let available_ops_to_show: Html = [
        ("super", props.super_ops_total),
        ("epic", props.epic_ops_total),
        ("awesome", props.awesome_ops_total),
    ]
    .iter()
    .map(|(name, maybe)| {
        if let Some(counts) = maybe {
            html! {
                <li><span>{"Total "} {name} {" ops:"}</span> {" "} {counts}</li>
            }
        } else {
            html! {
                <li class="hidden"><span>{"Total "} {name} {" ops:"}</span> {" 0"}</li>
            }
        }
    })
    .collect();

    html! {
        <ul class="stats">
            <li><span>{"Total base ops: "}</span> {props.base_ops_total}</li>
            {available_ops_to_show}
        </ul>
    }
}
