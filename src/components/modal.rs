use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ModalProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub children: Html,
}

/// View component for a modal box
/// that sits over the screen and can be customized with buttons.
#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    html! {
        <>
        <div class="modal-background" />
        <div class="modal">
            <h2>{&props.title}</h2>

            <div class="modal-content">
                {props.children.clone()}
            </div>
        </div>
        </>
    }
}
