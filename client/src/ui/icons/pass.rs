use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PassProps {
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(Pass)]
pub fn pass(props: &PassProps) -> Html {
    html! {
        <svg
        width="50"
        height="50"
        viewBox="0 0 50 50"
        fill="none"
        version="1.1"
        class={props.classes.clone()}
      >
      <g
          filter="url(#filter0_d)"
          id="g839"
          transform="translate(-20,-20)">
        <path
            d="m 32.699623,55.909927 22.83109,-22.83109"
            stroke="#b5a8f4"
            stroke-width="3.36334"
            stroke-linecap="round"
            stroke-linejoin="round"
            id="path837"
            style="stroke-width:2;stroke-miterlimit:4;stroke-dasharray:none" />
        <rect
            style="fill:none;stroke:#b5a8f4;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1"
            id="rect864"
            width="13.426971"
            height="24.171354"
            x="37.844101"
            y="33.293537" />
      </g>
    </svg>
      }
}
