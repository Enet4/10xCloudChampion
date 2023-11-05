#[cfg(not(feature = "playground"))]
mod app;
#[cfg(feature = "playground")]
mod playground;

#[cfg(not(feature = "playground"))]
fn main() {
    yew::Renderer::<app::App>::new().render();
}

#[cfg(feature = "playground")]
fn main() {
    yew::Renderer::<playground::Playground>::new().render();
}
