//! Module for the total stats
//! (all operations done so far)
use yew::prelude::*;

use crate::central::state::ServiceCounts;

#[derive(Debug, Default, PartialEq, Properties)]
pub struct TotalStatsProps {
    /// the counts for the base service
    pub base_service: ServiceCounts,

    /// the counts for the super service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub super_service: Option<ServiceCounts>,

    /// the counts for the epic service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub epic_service: Option<ServiceCounts>,

    /// the counts for the awesome service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub awesome_service: Option<ServiceCounts>,
}

/// The stats component.

#[function_component]
pub fn TotalStats(props: &TotalStatsProps) -> Html {
    let available_ops_to_show: Html = [
        ("super", props.super_service),
        ("epic", props.epic_service),
        ("awesome", props.awesome_service),
    ]
    .iter()
    .map(|(name, maybe)| {
        if let Some(counts) = maybe {
            html! {
                <li><span>{"Total "} {name} {" ops:"}</span> {" "} {counts.total}</li>
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
            <li><span>{"Total base ops: "}</span> {props.base_service.total}</li>
            {available_ops_to_show}
        </ul>
    }
}
