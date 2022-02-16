use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct XProps {
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(X)]
pub fn x(props: &XProps) -> Html {
    html! {
      <svg width="39" height="44" viewBox="0 0 39 44" fill="none" xmlns="http://www.w3.org/2000/svg" class={props.classes.clone()}>
        <path d="M28.9793 11.3889L10.021 32.875" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M10.021 11.3889L28.9793 32.875" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    }
}
