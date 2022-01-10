use crate::utils::use_styles::use_combined_styles;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_else(|| "".into())]
    pub styles: String,
    #[prop_or_else(|| "text".into())]
    pub input_type: String,
    pub label: String,
    pub id: String,
    pub value: String,
    #[prop_or_else(|| Callback::from(|_| {}))]
    pub oninput: Callback<InputEvent>,
}

#[styled_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let class_name = use_combined_styles(
        stringify!(input),
        r#"& {
          height: 63px;
          width: 100%;
          max-width: 300px;
          margin: 0;
    }
    & label {
      display: block;
    }
    & input {
      display: block;
      height: 100%;
      width: 100%;
      padding: 10px;
      margin: 0;
      background-color: white;
      outline: 0;
      border: 1px solid #D3D3D4;
    }
    "#,
        &props.styles,
    );

    html! {
      <div class={&class_name}>
        <label for={props.id.clone()}>{&props.label}</label>
        <input
          id={props.id.clone()}
          type={props.input_type.clone()}
          value={props.value.clone()}
          oninput={&props.oninput}
        />
      </div>
    }
}
