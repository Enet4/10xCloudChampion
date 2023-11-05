//! Module for the business component,
//! which shows some metrics about how the cloud management business is going.
use yew::prelude::*;

use crate::{central::state::ServiceCounts, Money};

#[derive(Debug, Default, PartialEq, Properties)]
pub struct BusinessProps {
    /// the available funds
    #[prop_or_default]
    pub funds: Money,

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

    /// the total number of clients using our services
    /// (or `None` if this has not been unlocked yet)
    #[prop_or_default]
    pub client_count: Option<u64>,

    /// the total number of internal researchers using our services
    /// for personal gain
    /// (or `None` if this has not been unlocked yet)
    #[prop_or_default]
    pub researcher_count: Option<u64>,
}

/// The business component.

#[function_component]
pub fn Business(props: &BusinessProps) -> Html {
    let available_ops_to_show: Html = [
        ("super", props.super_service),
        ("epic", props.epic_service),
        ("awesome", props.awesome_service),
    ]
    .iter()
    .filter_map(|(name, maybe)| maybe.map(|ops| (name, ops)))
    .map(|(name, counts)| {
        html! {
            <><span>{"Available "} {name} {" ops:"}</span> {" "} {counts.available}<br/></>
        }
    })
    .collect();

    html! {
        <div class="business">
            <p>
                <span>{"Funds: "}</span> {props.funds.into_cent().to_string()} <br/>
                <span>{"Available base ops: "}</span> {props.base_service.available} <br/>
                {available_ops_to_show}
            </p>
            <p>
                if let Some(count) = props.client_count {
                    <><span>{"Clients: "}</span> {count} <br/></>
                }
                if let Some(count) = props.researcher_count {
                    <><span>{"Researchers: "}</span> {count} <br/></>
                }
            </p>
        </div>
    }
}
