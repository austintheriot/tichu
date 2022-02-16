use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UpArrowProps {
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(UpArrow)]
pub fn up_arrow(props: &UpArrowProps) -> Html {
    html! {
      <svg width="90" height="90" viewBox="0 0 90 90" fill="none" version="1.1" class={props.classes.clone()}>
        <g>
          <path
             d="M 84.397521,45.741557 45.247185,6.5912214 6.0968501,45.741557"
             stroke-width="8.15631"
             stroke-linecap="round"
             stroke-linejoin="round"
             id="path833" />
          <path
             d="M 45.247185,84.891893 V 6.5912214"
             stroke-width="8.15631"
             stroke-linecap="round"
             stroke-linejoin="round"
             id="path835" />
        </g>
      </svg>
    }
}
