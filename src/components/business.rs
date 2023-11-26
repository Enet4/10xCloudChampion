//! Module for the business component,
//! which shows some metrics about how the cloud management business is going.
use yew::prelude::*;

use crate::{audio::play_zip_click, Money, Ops};

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

    /// the amount of money to be paid for electricity
    /// (bill should not appear if the money is below 1 cent)
    pub electricity_bill: Money,

    /// whether the player can afford to pay the electricity bill
    pub can_pay_bill: bool,

    /// callback for when the player clicks the "Pay" button
    pub on_pay_bills: Callback<()>,

    /// estimate for the service demand
    /// (or `None` if this has not been unlocked yet)
    pub demand: Option<f32>,
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

    let electricity = if props.electricity_bill >= Money::cents(1) {
        let onclick = props.on_pay_bills.clone();
        let onclick = move |_| {
            play_zip_click();
            onclick.emit(())
        };
        let enabled = if props.can_pay_bill { "true" } else { "false" };
        html! {
            <p>
                <span>{"Electricity bill: "}</span> {props.electricity_bill.into_cent_precision().to_string()}
                <button {enabled} {onclick}>{"Pay"}</button>
            </p>
        }
    } else {
        html! {}
    };

    html! {
        <div class="business">
            <p>
                <span>{"Funds: "}</span> {props.funds.into_cent_precision().to_string()} <br/>
                <span>{"Available base ops: "}</span> {props.base_ops_available} <br/>
                {available_ops_to_show}
            </p>
            <p>
                if let Some(demand) = props.demand {
                    <><span>{"Visibility: "}</span> {format!("{:.2}%", demand / 100.)} <br/></>
                }
            </p>
            {electricity}
        </div>
    }
}
