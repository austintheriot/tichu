#[cfg(test)]
mod test_get_next_user_turn_id {
    use common::{ImmutableTeam, PassWithUserId, PrivatePlay, UserIdWithTichuCallStatus};

    #[test]
    fn it_should_return_the_correct_next_user_id() {
        let user_1 = String::from("1");
        let user_2 = String::from("2");
        let user_3 = String::from("3");
        let user_4 = String::from("4");

        let tichu_call_statuses = [
            UserIdWithTichuCallStatus {
                user_id: user_1.clone(),
                tichu_call_status: common::TichuCallStatus::Declined,
            },
            UserIdWithTichuCallStatus {
                user_id: user_2.clone(),
                tichu_call_status: common::TichuCallStatus::Declined,
            },
            UserIdWithTichuCallStatus {
                user_id: user_3.clone(),
                tichu_call_status: common::TichuCallStatus::Declined,
            },
            UserIdWithTichuCallStatus {
                user_id: user_4.clone(),
                tichu_call_status: common::TichuCallStatus::Declined,
            },
        ];

        let teams = [
            ImmutableTeam {
                id: "a".into(),
                score: 0,
                team_name: "Example".into(),
                user_ids: [user_1.clone(), user_2.clone()],
            },
            ImmutableTeam {
                id: "b".into(),
                score: 0,
                team_name: "Example 2".into(),
                user_ids: [user_3.clone(), user_4.clone()],
            },
        ];

        let passes = [
            PassWithUserId {
                passed: false,
                user_id: user_1.clone(),
            },
            PassWithUserId {
                passed: false,
                user_id: user_2.clone(),
            },
            PassWithUserId {
                passed: false,
                user_id: user_3.clone(),
            },
            PassWithUserId {
                passed: false,
                user_id: user_4.clone(),
            },
        ];

        // ALL USERS IN ////////////////////////////////////////////////////////////////

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_1.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![
                user_1.clone(),
                user_2.clone(),
                user_3.clone(),
                user_4.clone(),
            ],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"3".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_2.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![
                user_1.clone(),
                user_2.clone(),
                user_3.clone(),
                user_4.clone(),
            ],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"4".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_3.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![
                user_1.clone(),
                user_2.clone(),
                user_3.clone(),
                user_4.clone(),
            ],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"2".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_4.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![user_1.clone(), user_2, user_3.clone(), user_4.clone()],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"1".into()));

        // ONE USER OUT ////////////////////////////////////////////////////////////////

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_1.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![user_1.clone(), user_3.clone(), user_4.clone()],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"3".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_3.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![user_1.clone(), user_3.clone(), user_4.clone()],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"4".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_4.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![user_1.clone(), user_3.clone(), user_4],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"1".into()));

        // TWO USERS OUT ////////////////////////////////////////////////////////////////

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses.clone(),
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_1.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes.clone(),
            users_in_play: vec![user_1.clone(), user_3.clone()],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"3".into()));

        let private_play = PrivatePlay {
            small_tichus: tichu_call_statuses.clone(),
            grand_tichus: tichu_call_statuses,
            teams: teams.clone(),
            table: vec![],
            turn_user_id: user_3.clone(),
            user_id_to_give_dragon_to: None,
            wished_for_card_value: None,
            passes: passes,
            users_in_play: vec![user_1, user_3],
            first_user_out: None,
        };
        assert_eq!(private_play.get_next_turn_user_id(), Some(&"1".into()));
    }
}
