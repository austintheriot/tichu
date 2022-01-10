use crate::utils::use_styles::use_combined_styles;
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
    #[prop_or_else(|| "".into())]
    pub styles: String,
    #[prop_or_else(|| Callback::from(|_| {}))]
    onclick: Callback<MouseEvent>,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class_name = use_combined_styles(
        stringify!(button),
        r#"& {
      display: block;
      margin: 0rem;
      padding: 0.75rem 1.5rem;
      background-color: white;
      outline: 0;
      border: 1px solid var(--gray-75);
      border-radius: 5px;
      font-size: 16px;
      cursor: pointer;
    }"#,
        &props.styles,
    );

    html! {
      <button
        type={props.button_type.clone()}
        class={&class_name}
        onclick={&props.onclick}
      >
        {for props.children.iter()}
      </button>
    }
}
