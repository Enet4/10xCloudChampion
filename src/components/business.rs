//! Module for the business component,
//! which shows some metrics about how the cloud management business is going.
use yew::prelude::*;

use crate::{Money, Ops};

#[derive(Debug, Default, PartialEq, Properties)]
pub struct BusinessProps {
    /// the available funds
    #[prop_or_default]
    pub funds: Money,

    /// ops available for the base service
    pub base_ops_available: Ops,

    /// ops available for the super service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub super_ops_available: Option<Ops>,

    /// ops available for the epic service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub epic_ops_available: Option<Ops>,

    /// ops available for the awesome service
    /// (or `None` if this service is not available yet)
    #[prop_or_default]
    pub awesome_ops_available: Option<Ops>,

    /// estimate for the total number of clients using our services
    /// (or `None` if this has not been unlocked yet)
    #[prop_or_default]
    pub client_count: Option<u64>,
}

/// The business component.

#[function_component]
pub fn Business(props: &BusinessProps) -> Html {
    let available_ops_to_show: Html = [
        ("super", props.super_ops_available),
        ("epic", props.epic_ops_available),
        ("awesome", props.awesome_ops_available),
    ]
    .iter()
    .filter_map(|(name, maybe)| maybe.map(|ops| (name, ops)))
    .map(|(name, counts)| {
        html! {
            <><span>{"Available "} {name} {" ops:"}</span> {" "} {counts}<br/></>
        }
    })
    .collect();

    html! {
        <div class="business">
            <p>
                <span>{"Funds: "}</span> {props.funds.into_cent().to_string()} <br/>
                <span>{"Available base ops: "}</span> {props.base_ops_available} <br/>
                {available_ops_to_show}
            </p>
            <p>
                if let Some(count) = props.client_count {
                    <><span>{"Clients: "}</span> {count} <br/></>
                }
            </p>
        </div>
    }
}
