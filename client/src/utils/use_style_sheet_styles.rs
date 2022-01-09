use gloo::utils::document;
use log::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleSheet, HtmlStyleElement};
use yew::{use_effect, use_mut_ref, use_ref};

const APP_STYLES_ID: &'static str = "tichu-app-styles";

fn get_or_make_style_sheet() -> CssStyleSheet {
    document()
        .get_element_by_id(APP_STYLES_ID)
        // if it exists in the document, it is an HtmlStyleElement
        .and_then(|element| element.dyn_into::<HtmlStyleElement>().ok())
        // get its style sheet
        .and_then(|style_element| {
            style_element
                .sheet()
                .and_then(|sheet| sheet.dyn_into::<CssStyleSheet>().ok())
        })
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

            // get style_element sheet
            style_element
                .sheet()
                .and_then(|sheet| sheet.dyn_into::<CssStyleSheet>().ok())
        })
        .unwrap()
}

pub fn use_style_sheet_styles(selector: &str, rules: &str) -> String {
    let uuid = use_ref(|| {
        let mut uuid = Uuid::new_v4().to_string();
        uuid.retain(|ch| ch.is_alphabetic());
        uuid
    });
    let style_sheet = use_ref(get_or_make_style_sheet);
    let indexes = use_mut_ref(|| Vec::new());

    {
        let uuid = uuid.clone();
        let selector = selector.to_owned();
        let rules = rules.to_owned();
        use_effect(move || {
            let rules: Vec<&str> = rules.split("\n").collect();
            let mut indexes = (*indexes).borrow_mut();
            for rule in rules {
                let i = style_sheet
                    .insert_rule(&format!(".{}{} {{ {} }}", uuid, selector, rule))
                    .unwrap();
                info!("{}", i);
                indexes.push(i);
            }

            // remove rules on cleanup
            || {}
        });
    }

    (*uuid).clone()
}
