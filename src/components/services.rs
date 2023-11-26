//! Module for all cloud services,
//! which generate ops.

use core::fmt;
use std::collections::VecDeque;

use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::{
    audio::{play_op_click, play_zip_click},
    components::pop::Pop,
    Money, ServiceKind,
};

#[derive(Debug, PartialEq, Properties)]
pub struct CloudServiceProps {
    pub kind: ServiceKind,
    pub on_click: Callback<()>,
    pub on_price_change: Callback<Money>,
    pub price: Money,
    #[prop_or_default]
    pub new: bool,
    #[prop_or_default]
    pub private: bool,
}

/// the information to be shown in a cloud service op pop-up
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CountPop {
    count: i32,
}

impl fmt::Display for CountPop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:+}", self.count)
    }
}

impl ToHtml for CountPop {
    fn to_html(&self) -> Html {
        html! {
            {self}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CloudServiceMessage {
    /// add a new pop-up
    New(CountPop),
    /// make the oldest one disappear
    Disappear,
}

/// The cloud service component.
#[derive(Debug)]
pub struct CloudService {
    k: u32,
    popups: VecDeque<(u32, CountPop)>,
}

impl Component for CloudService {
    type Message = CloudServiceMessage;
    type Properties = CloudServiceProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            k: 0,
            popups: VecDeque::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CloudServiceMessage::New(c) => {
                self.popups.push_back((self.k, c));
                self.k = self.k.wrapping_add(1);
                // create a timeout to emit a message to make the pop-up disappear
                let link = _ctx.link().clone();
                let timeout = Timeout::new(800, move || {
                    link.send_message(CloudServiceMessage::Disappear);
                });
                timeout.forget();
            }
            CloudServiceMessage::Disappear => {
                self.popups.pop_front();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().kind.to_string();

        let color = match ctx.props().kind {
            ServiceKind::Base => "#ccc",
            ServiceKind::Super => "#bbf",
            ServiceKind::Epic => "#efc",
            ServiceKind::Awesome => "#ecf",
        };

        let on_click = ctx.props().on_click.clone();

        let onclick = {
            let onclick = Callback::from(move |_e: MouseEvent| {
                play_op_click();
                on_click.emit(());
            });
            onclick
        };

        let on_lower_price = {
            let on_price_change = ctx.props().on_price_change.clone();
            let price = ctx.props().price;
            let on_lower_price = Callback::from(move |_e: MouseEvent| {
                play_zip_click();
                let new_price = lower_price(price);
                on_price_change.emit(new_price);
            });
            on_lower_price
        };

        let on_raise_price = {
            let on_price_change = ctx.props().on_price_change.clone();
            let price = ctx.props().price;
            let on_raise_price = Callback::from(move |_e: MouseEvent| {
                play_zip_click();
                let new_price = raise_price(price);
                on_price_change.emit(new_price);
            });
            on_raise_price
        };

        let style = format!("background-color: {color}");

        let button_classes: Classes = if ctx.props().new {
            classes!("op", "new")
        } else {
            classes!("op")
        };

        html! {
            <div class="service" style={style}>
                <h4>{ name }</h4>
                <button class={button_classes} onclick={onclick}>{"Op"}</button>
                // price and buttons to lower/raise
                if ctx.props().private {
                    <div class="private">
                        <span>{"TESTING"}</span>
                    </div>
                } else {
                    <div class="price-container">
                        <div class="price">
                            <span>{"Price: "}</span><span class="money">{ctx.props().price.to_string()}</span>
                        </div>
                        <div class="change">
                            <button onclick={on_lower_price}>{"lower"}</button>
                            <button onclick={on_raise_price}>{"raise"}</button>
                        </div>
                    </div>
                }
                // pop-ups
                {
                    self.popups.iter().map(|(k, c)|
                        html! {
                            <Pop<CountPop> key={*k} text={*c} />
                        })
                        .collect::<Html>()
                }
            </div>
        }
    }
}

/// based on current price, decide how to lower it
fn lower_price(price: Money) -> Money {
    if price <= Money::millicents(1) {
        Money::millicents(1)
    } else if price <= Money::millicents(20) {
        price - Money::millicents(1)
    } else if price <= Money::millicents(100) {
        price - Money::millicents(5)
    } else if price <= Money::millicents(200) {
        price - Money::millicents(10)
    } else if price <= Money::cents(1) {
        price - Money::millicents(50)
    } else if price <= Money::cents(2) {
        price - Money::millicents(100)
    } else if price <= Money::cents(10) {
        price - Money::millicents(500)
    } else if price <= Money::cents(20) {
        price - Money::cents(1)
    } else if price <= Money::dollars(1) {
        price - Money::cents(5)
    } else if price <= Money::dollars(2) {
        price - Money::cents(10)
    } else {
        price - Money::cents(50)
    }
}

/// based on current price, decide how to raise it
fn raise_price(price: Money) -> Money {
    if price >= Money::dollars(25) {
        Money::dollars(25)
    } else if price >= Money::dollars(2) {
        price + Money::cents(50)
    } else if price >= Money::dollars(1) {
        price + Money::cents(10)
    } else if price >= Money::cents(20) {
        price + Money::cents(5)
    } else if price >= Money::cents(10) {
        price + Money::cents(1)
    } else if price >= Money::cents(2) {
        price + Money::dec_cents(5)
    } else if price >= Money::cents(1) {
        price + Money::dec_cents(1)
    } else if price >= Money::millicents(200) {
        price + Money::millicents(50)
    } else if price >= Money::millicents(100) {
        price + Money::millicents(10)
    } else if price >= Money::millicents(20) {
        price + Money::millicents(5)
    } else {
        price + Money::millicents(1)
    }
}

#[cfg(test)]
mod tests {
    use super::{lower_price, raise_price};
    use crate::Money;

    #[test]
    fn test_price_changes() {
        assert_eq!(raise_price(Money::millicents(1)), Money::millicents(2));
        assert_eq!(raise_price(Money::millicents(10)), Money::millicents(11));
        assert_eq!(raise_price(Money::millicents(11)), Money::millicents(12));
        assert_eq!(raise_price(Money::millicents(20)), Money::millicents(25));
        assert_eq!(raise_price(Money::millicents(45)), Money::millicents(50));
        assert_eq!(raise_price(Money::millicents(50)), Money::millicents(55));
        assert_eq!(raise_price(Money::millicents(100)), Money::millicents(110));
        assert_eq!(raise_price(Money::dec_cents(10)), Money::dec_cents(11));
        assert_eq!(raise_price(Money::dec_cents(11)), Money::dec_cents(12));
        assert_eq!(raise_price(Money::dec_cents(20)), Money::dec_cents(25));
        assert_eq!(raise_price(Money::dec_cents(45)), Money::dec_cents(50));
        assert_eq!(raise_price(Money::cents(10)), Money::cents(11));
        assert_eq!(raise_price(Money::cents(11)), Money::cents(12));
        assert_eq!(raise_price(Money::cents(20)), Money::cents(25));
        assert_eq!(raise_price(Money::cents(45)), Money::cents(50));
        assert_eq!(raise_price(Money::cents(50)), Money::cents(55));
        assert_eq!(raise_price(Money::cents(100)), Money::cents(110));

        assert_eq!(lower_price(Money::millicents(10)), Money::millicents(9));
        assert_eq!(lower_price(Money::millicents(11)), Money::millicents(10));
        assert_eq!(lower_price(Money::millicents(20)), Money::millicents(19));
        assert_eq!(lower_price(Money::millicents(25)), Money::millicents(20));
        assert_eq!(lower_price(Money::millicents(50)), Money::millicents(45));
        assert_eq!(lower_price(Money::millicents(55)), Money::millicents(50));
        assert_eq!(lower_price(Money::millicents(100)), Money::millicents(95));
        assert_eq!(lower_price(Money::millicents(110)), Money::millicents(100));
        assert_eq!(lower_price(Money::dec_cents(10)), Money::millicents(950));
        assert_eq!(lower_price(Money::dec_cents(11)), Money::dec_cents(10));
        assert_eq!(lower_price(Money::dec_cents(20)), Money::dec_cents(19));
        assert_eq!(lower_price(Money::dec_cents(25)), Money::dec_cents(20));
        assert_eq!(lower_price(Money::cents(10)), Money::dec_cents(95));
        assert_eq!(lower_price(Money::cents(11)), Money::cents(10));
        assert_eq!(lower_price(Money::cents(20)), Money::cents(19));
        assert_eq!(lower_price(Money::cents(25)), Money::cents(20));
        assert_eq!(lower_price(Money::cents(50)), Money::cents(45));
        assert_eq!(lower_price(Money::cents(55)), Money::cents(50));
        assert_eq!(lower_price(Money::cents(100)), Money::cents(95));
        assert_eq!(lower_price(Money::cents(110)), Money::cents(100));
        assert_eq!(lower_price(Money::cents(200)), Money::cents(190));
        assert_eq!(lower_price(Money::cents(250)), Money::cents(200));
    }
}
