//! Module for a small text component that pops up and fades out quickly

use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct PopProps<T>
where
    T: PartialEq,
{
    /// the content to display
    pub text: T,
}

/// The pop component.
#[function_component]
pub fn Pop<T>(props: &PopProps<T>) -> Html
where
    T: PartialEq,
    T: ToHtml,
{
    html! {
        <span class="pop">{&props.text}</span>
    }
}
