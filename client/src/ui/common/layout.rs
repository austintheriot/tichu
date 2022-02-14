use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Vec<String>,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let mut base_classes = props.classes.clone();
    base_classes.push("layout".into());

    html! {
      <div class={base_classes}>
        {for props.children.iter()}
      </div>
    }
}
