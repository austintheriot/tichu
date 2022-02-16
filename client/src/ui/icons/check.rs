use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CheckProps {
    #[prop_or_default]
    pub classes: Vec<String>,
    #[prop_or_default]
    pub style: String,
}

#[function_component(Check)]
pub fn check(props: &CheckProps) -> Html {
    html! {
      <svg width="18" height="13" viewBox="0 0 18 13" fill="none" xmlns="http://www.w3.org/2000/svg" class={props.classes.clone()} style={props.style.clone()}>
        <path d="M17 1L6 12L1 7" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    }
}
