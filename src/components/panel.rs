use yew::prelude::*;


#[derive(Debug, PartialEq, Properties)]
pub struct PanelProps {
    pub children: Html,
    #[prop_or_default]
    pub classes: Option<Classes>,
    pub title: AttrValue,
}

#[function_component]
pub fn Panel(props: &PanelProps) -> Html {
    let mut classes = props.classes.clone().unwrap_or_default();
    classes.extend(Classes::from("panel"));
    html! {
        <div class={classes}>
            <h3>{ &props.title }</h3>
            { props.children.clone() }
        </div>
    }
}
