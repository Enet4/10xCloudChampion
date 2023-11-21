use yew::prelude::*;

use crate::audio::play_zip_click;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct MenuProps {
    pub newgame_handler: Callback<()>,
    pub continuegame_handler: Callback<()>,
    pub has_save: bool,
}

#[function_component]
pub fn Menu(props: &MenuProps) -> Html {
    let newgame_handler = props.newgame_handler.clone();
    let continuegame_handler = props.continuegame_handler.clone();
    html! {
        <>
        <div class="main-menu-back" />
        <div class="main-menu">
            <h1><img src="assets/ico.svg" /> { " Cloud Champion" }</h1>
            <div class="main-menu-prompt">
                if props.has_save {
                    <button onclick={move |_| {
                        play_zip_click();
                        continuegame_handler.emit(())
                    }}>{"Continue Game"}</button>
                }
                <button onclick={move |_| {
                    play_zip_click();
                    newgame_handler.emit(())
                }}>{"New Game"}</button>
            </div>
            <footer><a href="https://github.com/Enet4/10xCloudChampion">{"On GitHub"}</a></footer>
        </div>
        </>
    }
}
