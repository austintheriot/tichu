use crate::utils::use_styles::use_styles;
use gloo::utils::document;
use log::info;
use stylist::yew::styled_component;
use uuid::{uuid, Uuid};
use wasm_bindgen::JsCast;
use web_sys::{CssStyleSheet, HtmlStyleElement};
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
    let class_name = use_styles(
        r#"
      & {
        color: red;
      }
    "#,
    );
    let button_styles = css!(
        r#"
        display: block;
        margin: 1rem;
        padding: 0.75rem 1.5rem;
        background-color: white;
        outline: 0;
        border: 1px solid var(--gray-75);
        border-radius: 5px;
        font-size: 16px;
      "#
    );

    html! {
      <button type={props.button_type.clone()} class={&class_name}>
        {for props.children.iter()}
      </button>
    }
}
