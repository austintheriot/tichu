use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| String::from("button"))]
    pub button_type: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(|| Callback::from(|_| {}))]
    onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut base_classes = props.classes.clone();
    base_classes.push("Button".into());

    html! {
      <button
        class={base_classes}
        type={props.button_type.clone()}
        onclick={&props.onclick}
      >
        {for props.children.iter()}
      </button>
    }
}
