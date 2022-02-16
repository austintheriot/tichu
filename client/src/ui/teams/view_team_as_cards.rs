use super::view_user_as_card::ViewUserAsCard;
use common::MutableTeam;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewTeamAsCardsProps {
    pub team: MutableTeam,
}

#[function_component(ViewTeamAsCards)]
pub fn view_team_as_cards(props: &ViewTeamAsCardsProps) -> Html {
    html! {
        <div class="view-team-as-cards-container">
            {props
                .team
                .user_ids
                .iter()
                .map(|user_id| {
                    html! {
                        <ViewUserAsCard user_id={user_id.clone()} />
                    }
                })
                .collect::<Html>()
            }
        </div>
    }
}
