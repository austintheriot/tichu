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
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let mut base_classes = props.classes.clone();
    base_classes.push("Button".into());

    html! {
      <div class={base_classes}>
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
