//! Experiment in making CSS utility hooks for component-scoped styles
//!
//! Hacky and slow, but should suffice for my needs until a more stable library is in use

use gloo::utils::document;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlStyleElement;
use yew::{use_effect, use_ref};

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

/// Very quick and dirty tool for using component-scoped styles
/// The Tichu front-end shouldn't be re-rendering a lot, so running this
/// on initial-renders/unmounts should be fine.
///
/// Pass in regular CSS styles and use `&` to select the top-level element in the component
pub fn use_styles(component_name: &str, styles: String) -> String {
    // uuid for component instance
    let uuid = use_ref(|| format!("{}-{}", component_name, Uuid::new_v4().to_string()));
    let style_element = use_ref(get_or_make_style_element);

    {
        // add styles to <style> tag
        let uuid = uuid.clone();
        use_effect(move || {
            // replace `&` operator with instance's uuid
            let mut style_text_content = style_element.text_content().unwrap_or("".to_string());
            let edited_styles = styles.replace("&", &format!(".{}", uuid));
            let edited_styles = edited_styles
                // remove new lines
                .split("\n")
                // remove extra spaces
                .map(|string| string.trim())
                .collect::<String>();
            style_text_content.push_str(&edited_styles);
            style_element.set_text_content(Some(&style_text_content));

            // remove generated rules on cleanup
            move || {
                let style_text_content = style_element.text_content();
                // if None, then no need to try to remove any existing styles
                if let Some(style_text_content) = style_text_content {
                    let edited_text_content = style_text_content.replace(&edited_styles, "");
                    style_element.set_text_content(Some(&edited_text_content));
                }
            }
        });
    }

    (*uuid).clone()
}

/// More convenient wrapper for adding styles from props
pub fn use_combined_styles(
    component_name: &str,
    component_styles: &str,
    props_styles: &str,
) -> String {
    use_styles(
        component_name,
        format!("{} {}", component_styles, props_styles),
    )
}
