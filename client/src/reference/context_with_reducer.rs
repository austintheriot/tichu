use std::rc::Rc;
use yew::{function_component, html, prelude::*};

enum ParentReducerAction {
    Increment,
}

#[derive(Clone, Debug, PartialEq)]
struct ParentState {
    count: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct ParentContext {
    reducer_handle: UseReducerHandle<ParentState>,
}

impl Reducible for ParentState {
    /// Reducer Action Type
    type Action = ParentReducerAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_ctr = match action {
            ParentReducerAction::Increment => self.count + 1,
        };

        Self { count: next_ctr }.into()
    }
}

/// Main component
#[function_component(Parent)]
pub fn parent() -> Html {
    // The use_reducer hook takes an initialization function which will be called only once.
    let reducer_handle = use_reducer_eq(|| ParentState { count: 1 });
    let context = ParentContext {
        reducer_handle: reducer_handle.clone(),
    };

    html! {
        <ContextProvider<ParentContext> {context}>
            <p>{reducer_handle.count}</p>
            <Middle />
        </ContextProvider<ParentContext>>
    }
}

/// Middle ocomponent
#[function_component(Middle)]
pub fn middle() -> Html {
    html! {
        <Child />
    }
}

#[function_component(Child)]
pub fn child() -> Html {
    let parent_context = use_context::<ParentContext>().expect("no ctx found");
    let onclick = {
        let reducer_handle = parent_context.reducer_handle.clone();
        Callback::from(move |_: MouseEvent| reducer_handle.dispatch(ParentReducerAction::Increment))
    };

    html! {
        <button {onclick}>
            { "Increment Count" }
        </button>
    }
}
