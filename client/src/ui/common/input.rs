use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
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
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub classes: Vec<String>,
    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub maxlength: Option<usize>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let mut base_classes = props.classes.clone();
    base_classes.push("input".into());
    let input_ref = use_node_ref();
    let is_empty = use_state(|| props.value.is_empty());

    // when component is mounted, check whether this element is the document's active element
    let is_focused = use_state(|| {
        let active_element = document().active_element();
        let input_element = input_ref.cast::<HtmlInputElement>();
        if let Some(active_element) = active_element {
            if let Some(input_element) = input_element {
                return active_element == *input_element.as_ref();
            }
        }
        false
    });

    let handle_input = {
        let is_empty = is_empty.clone();
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            {
                // emit callback that was passed to component
                let e = e.clone();
                oninput.emit(e);
            }
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let value = input.map(|input| input.value()).unwrap();
            is_empty.set(value.is_empty());
        })
    };

    let handle_focus = {
        let is_focused = is_focused.clone();
        Callback::from(move |_: FocusEvent| {
            is_focused.set(true);
        })
    };

    let handle_blur = {
        let is_focused = is_focused.clone();
        let onblur = props.onblur.clone();
        Callback::from(move |e: FocusEvent| {
            {
                // emit callback that was passed to component
                let e = e.clone();
                onblur.emit(e);
            }
            is_focused.set(false);
        })
    };

    if *is_empty && props.value.is_empty() && !*is_focused {
        base_classes.push("input-label-down".into());
    }

    html! {
      <div class={base_classes}>
        <label for={props.id.clone()}>{&props.label}</label>
        <input
          id={props.id.clone()}
          type={props.input_type.clone()}
          value={props.value.clone()}
          oninput={handle_input}
          onfocus={handle_focus}
          onblur={handle_blur}
          ref={input_ref}
          disabled={props.disabled}
          maxlength={props.maxlength.map(|maxlength| maxlength.to_string())}
        />
        if let Some(error) = &props.error {
            <p class="error">{error}</p>
        }
      </div>
    }
}
