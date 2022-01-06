use yew::prelude::*;

pub struct Button;

pub enum ButtonMsg {}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
}

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button type="button">
                {&ctx.props().text}
            </button>
        }
    }
}
