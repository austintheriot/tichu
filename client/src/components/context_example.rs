use yew::{function_component, html, prelude::*, use_state};

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    foreground: String,
    background: String,
}

/// Main component
#[function_component(ContextExample)]
pub fn context_example() -> Html {
    let ctx = use_state(|| Theme {
        foreground: "#000000".to_owned(),
        background: "#ffffff".to_owned(),
    });

    html! {
        // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme`
        // so we deref it.
        // It derefs to `&Theme`, hence the clone
        <ContextProvider<UseStateHandle<Theme>> context={ctx.clone()}>
            // Every child here and their children will have access to this context.
            <Toolbar />
        </ContextProvider<UseStateHandle<Theme>>>
    }
}

/// The toolbar.
/// This component has access to the context
#[function_component(Toolbar)]
pub fn toolbar() -> Html {
    html! {
        <div>
            <ThemedButton />
        </div>
    }
}

/// Button placed in `Toolbar`.
/// As this component is a child of `ThemeContextProvider` in the component tree, it also has access to the context.
#[function_component(ThemedButton)]
pub fn themed_button() -> Html {
    let ctx = use_context::<UseStateHandle<Theme>>().expect("no ctx found");
    let ctx_clone = ctx.clone();
    let theme = &*ctx;

    let onclick = {
        Callback::from(move |_: MouseEvent| {
            ctx_clone.set(Theme {
                foreground: "#f00".into(),
                background: "#0f0".into(),
            })
        })
    };

    html! {
        <button
            {onclick}
            style={format!("background: {}; color: {};", theme.background, theme.foreground)}
            >
            { "Click me!" }
        </button>
    }
}
