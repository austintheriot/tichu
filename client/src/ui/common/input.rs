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

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
      <div>
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
