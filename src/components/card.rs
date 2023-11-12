//! Module for the bonus/purchase card component.

use yew::prelude::*;

use crate::Cost;

#[derive(PartialEq, Properties)]
pub struct CardProps {
    pub id: &'static str,
    pub title: AttrValue,
    pub description: AttrValue,
    pub cost: Cost,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub effect: (),
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
}

/// The bonus/purchase project card component.
#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let class = if props.disabled {
        classes!["card", "disabled"]
    } else {
        classes!["card"]
    };
    let disabled = props.disabled;

    let on_card_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(on_click) = &on_click {
                on_click.emit(());
            }
            // TODO apply effect?
        })
    };

    let cost = if props.cost.is_nothing() {
        html! {}
    } else {
        html! {
            <span class="cost">
                {"("}
                {props.cost.to_string()}
                {")"}
            </span>
        }
    };

    html! {
        <button key={props.id} class={class} disabled={disabled} onclick={on_card_click}>
            <div>
                <b>{ &props.title }</b>
                <span class="cost">{cost}</span>
            </div>
            <p>{ &props.description }</p>
        </button>
    }
}
