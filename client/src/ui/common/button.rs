use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Square,
    Circle,
}

impl From<ButtonVariant> for String {
    fn from(variant: ButtonVariant) -> Self {
        match variant {
            ButtonVariant::Square => "square".to_string(),
            ButtonVariant::Circle => "circle".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(|| String::from("button"))]
    pub button_type: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(|| Callback::from(|_| {}))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub classes: Vec<String>,
    #[prop_or(ButtonVariant::Square)]
    pub variant: ButtonVariant,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut base_classes = props.classes.clone();
    base_classes.push("button".into());
    base_classes.push(props.variant.clone().into());

    html! {
      <button
        class={base_classes}
        type={props.button_type.clone()}
        onclick={&props.onclick}
        disabled={props.disabled}
      >
        {for props.children.iter()}
      </button>
    }
}
