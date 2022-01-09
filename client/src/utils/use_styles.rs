use gloo::utils::document;
use log::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlStyleElement;
use yew::{use_effect, use_mut_ref, use_ref};

const APP_STYLES_ID: &'static str = "tichu-app-styles";

fn get_or_make_style_element() -> HtmlStyleElement {
    document()
        .get_element_by_id(APP_STYLES_ID)
        // if it exists in the document, it is an HtmlStyleElement
        .and_then(|element| element.dyn_into::<HtmlStyleElement>().ok())
        // if it doesn't exist, create it
        .or_else(|| {
            // create element
            let element = document().create_element("style").unwrap();
            let style_element = element.dyn_into::<HtmlStyleElement>().unwrap();
            style_element.set_id(APP_STYLES_ID);

            // append style element to head
            document()
                .head()
                .unwrap()
                .append_child(style_element.as_ref())
                .unwrap();

            Some(style_element)
        })
        .unwrap()
}

pub fn use_styles(styles: &str) -> String {
    let uuid = use_ref(|| {
        let mut uuid = Uuid::new_v4().to_string();
        uuid.retain(|ch| ch.is_alphabetic());
        uuid
    });
    let style_element = use_ref(get_or_make_style_element);
    {
        let styles = styles.to_owned();
        let uuid = uuid.clone();
        use_effect(move || {
            let mut style_text_content = style_element.text_content().unwrap_or("".to_string());
            let edited_styles = styles.replace("&", &format!(".{}", uuid));
            style_text_content.push_str(&edited_styles);
            style_element.set_text_content(Some(&style_text_content));

            // remove rules on cleanup
            || {}
        });
    }

    (*uuid).clone()
}
