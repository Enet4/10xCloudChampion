use yew::prelude::*;


#[derive(Debug, PartialEq, Properties)]
pub struct PanelProps {
    pub children: Html,
    pub title: AttrValue,
}

#[function_component]
pub fn Panel(props: &PanelProps) -> Html {
    html! {
        <div class="panel">
            <h3>{ &props.title }</h3>
            { props.children.clone() }
        </div>
    }
}
