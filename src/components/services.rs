//! Module for all cloud services,
//! which generate ops.

use core::fmt;
use std::collections::VecDeque;

use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::{components::pop::Pop, Money, ServiceKind};

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
            let link = ctx.link().clone();
            let onclick = Callback::from(move |_e: MouseEvent| {
                on_click.emit(());
                link.send_message(CloudServiceMessage::New(CountPop { count: 1 }));
            });
            onclick
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
                    <div class="price">
                        <span>{"Price: "}</span><span class="money">{ctx.props().price.to_string()}</span>
                    </div>
                    <div class="change">
                        <button>{"lower"}</button>
                        <button>{"raise"}</button>
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
