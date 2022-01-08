use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| String::from("button"))]
    pub button_type: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let button_styles = css!(
        r#"
        display: block;
        margin: 1rem;
        padding: 0.25rem;
        background-color: white;

      "#
    );

    html! {
      <button type={props.button_type.clone()} class={button_styles}>
        {for props.children.iter()}
      </button>
    }
}
