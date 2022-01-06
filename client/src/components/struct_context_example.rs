use log::*;
use yew::{context::ContextHandle, prelude::*};

// CONTEXT ////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub struct ParentContext {
    state: ParentState,
    msg: Callback<ParentMsg>,
}

// PARENT COMPONENT ////////////////////////////////////////////////////////////////

pub enum ParentMsg {
    IncCount,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParentState {
    count: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Parent {
    state: ParentState,
}

impl Component for Parent {
    type Message = ParentMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            state: ParentState { count: 0 },
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        info!("Running update() in Parent component");
        match msg {
            ParentMsg::IncCount => {
                self.state = ParentState {
                    count: self.state.count + 1,
                };
                info!("New state: {:?}", self.state);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg = ctx.link().callback(|msg: ParentMsg| msg);
        let state = self.state.clone();
        let parent_context = ParentContext { state, msg };
        html! {
            <ContextProvider<ParentContext> context={parent_context}>
                <p>{"Parent count: "}{&self.state.count}</p>

                <button onclick={ctx.link().callback(move |_| { ParentMsg::IncCount})}>
                  {"Increment count from parent"}
                </button>

                <Middle />
            </ContextProvider<ParentContext>>
        }
    }
}

// MIDDLE COMPONENT ////////////////////////////////////////////////////////////////
#[function_component(Middle)]
pub fn middle() -> Html {
    html! {
        <Child />
    }
}

// CHILD COMPONENT ////////////////////////////////////////////////////////////////
pub struct Child {
    parent_context: ParentContext,
    // when the context_handle is dropped, the listener is unsubscribed,
    // so save context handle in state
    #[allow(dead_code)]
    context_handle: ContextHandle<ParentContext>,
}

pub enum ChildMsg {
    SetParentContext(ParentContext),
}

impl Component for Child {
    type Message = ChildMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // subscribe to changes in context
        let link = ctx.link().clone();
        let (parent_context, context_handle) = ctx
            .link()
            // this callback is called when parent context changes
            .context::<ParentContext>(Callback::from(move |parent_context| {
                // save updated context in this component's state
                link.send_message(Self::Message::SetParentContext(parent_context));
            }))
            .expect("context to be set");

        Self {
            parent_context,
            context_handle,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::SetParentContext(parent_context) => {
                self.parent_context = parent_context;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let msg_clone = self.parent_context.msg.clone();
        html! {
          <>
            <p>{format!("Child count: {}", &self.parent_context.state.count)}</p>

            <button onclick={ctx.link().batch_callback(move |_| {
              // emit message to parent component
              msg_clone.emit(ParentMsg::IncCount);
              // do not send message to Child component
              None
            })}>
              {"Increment count from child"}
            </button>
          </>
        }
    }
}
