use super::call_small_tichu_button::CallSmallTichuButton;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CallSmallTichuContainerProps {
    #[prop_or_default]
    pub show_optional: bool,
}

/// Wrapper around CallSmallTichu Button with info about what the button does
#[function_component(CallSmallTichuContainer)]
pub fn call_small_tichu_container(props: &CallSmallTichuContainerProps) -> Html {
    html! {
      <div class="call-small-tichu-container">
          <CallSmallTichuButton show_optional={props.show_optional} />
          <p>{"± 100 points for going out first"}</p>
      </div>
    }
}
