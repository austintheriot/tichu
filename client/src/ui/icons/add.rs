use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AddProps {
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(Add)]
pub fn add(props: &AddProps) -> Html {
    html! {
      <svg width="38" height="38" viewBox="0 0 38 38" fill="none" xmlns="http://www.w3.org/2000/svg" class={props.classes.clone()}>
        <path d="M18.6035 12.2603V25.7396" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M12.2563 18.6035H25.744" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>

    }
}
