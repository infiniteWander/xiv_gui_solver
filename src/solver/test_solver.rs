#[allow(unused_imports)]
use super::*;

#[test]
pub fn test_first_step() {
    let mut craft = crate::craft::Craft::two_star();
    // Try a failed craft
    craft.success = crate::specs::Success::Failure;
    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, vec![None]);
    craft.success = crate::specs::Success::Pending;

    // Try the first 4 steps
    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, action_vec![ACTIONS.muscle_memory]);
    craft.run_action(&ACTIONS.muscle_memory);

    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, action_vec![ACTIONS.manipulation]);
    craft.run_action(&ACTIONS.manipulation);

    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, action_vec![ACTIONS.veneration]);
    craft.run_action(&ACTIONS.veneration);

    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, action_vec![ACTIONS.waste_not_ii, ACTIONS.groundwork]);
    craft.run_action(&ACTIONS.waste_not_ii);

    // Try the steps 5-8, deterministically
    let vact = next_action_picker_1(&craft);
    assert_eq!(vact, action_vec![ACTIONS.groundwork]);
    craft.run_action(&ACTIONS.careful_synthesis);

    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.groundwork,
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.delicate_synthesis
        ]
    );
    craft.run_action(&ACTIONS.careful_synthesis);

    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.groundwork,
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.delicate_synthesis
        ]
    );
    craft.run_action(&ACTIONS.careful_synthesis);

    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.groundwork,
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.delicate_synthesis
        ]
    );
    craft.run_action(&ACTIONS.careful_synthesis);

    // Check odd parameters
    craft.buffs.waste_not = 0;
    craft.progression = 0;
    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.prudent_synthesis,
            ACTIONS.delicate_synthesis
        ]
    );
    craft.run_action(&ACTIONS.delicate_synthesis);

    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.prudent_synthesis,
            ACTIONS.delicate_synthesis,
            ACTIONS.waste_not_ii,
            ACTIONS.waste_not
        ]
    );

    craft.args.desperate = true;
    let vact = next_action_picker_1(&craft);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.prudent_synthesis,
            ACTIONS.delicate_synthesis,
            ACTIONS.groundwork,
            ACTIONS.masters_mend,
            ACTIONS.waste_not_ii,
            ACTIONS.waste_not
        ]
    );
    assert_eq!(craft.progression, 250);
    assert_eq!(craft.quality, 266);
    assert_eq!(craft.cp, 317);
}

#[test]
pub fn test_routes_p1() {
    let craft = crate::craft::Craft::two_star();
    let mut routes1 = generate_routes_phase1(craft);

    for route in routes1.iter() {
        println!("{:?}", route.actions);
    }
    // Literrally any change will break it
    // An eq should be implmented to just check a vector of vector versus a route
    assert_eq!(routes1.len(), 9);
    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.waste_not_ii,
            &ACTIONS.groundwork,
            &ACTIONS.delicate_synthesis,
            &ACTIONS.delicate_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.waste_not_ii,
            &ACTIONS.groundwork,
            &ACTIONS.delicate_synthesis,
            &ACTIONS.basic_synthesis,
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.waste_not_ii,
            &ACTIONS.groundwork,
            &ACTIONS.careful_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.waste_not_ii,
            &ACTIONS.groundwork,
            &ACTIONS.basic_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.groundwork,
            &ACTIONS.delicate_synthesis,
            &ACTIONS.delicate_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.groundwork,
            &ACTIONS.delicate_synthesis,
            &ACTIONS.basic_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.groundwork,
            &ACTIONS.prudent_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.groundwork,
            &ACTIONS.careful_synthesis
        ]
    );

    let route = routes1.pop().unwrap();
    assert_eq!(
        route.actions,
        vec![
            &ACTIONS.muscle_memory,
            &ACTIONS.manipulation,
            &ACTIONS.veneration,
            &ACTIONS.groundwork,
            &ACTIONS.basic_synthesis
        ]
    );

    let route = routes1.pop();
    assert!(route.is_none());
}

// Test Custom routes (None and basic synthesis)
#[test]
pub fn test_custom_p1_routes() {
    // None route, no more actions
    let mut craft = crate::craft::Craft::zero_star();
    craft.success = crate::specs::Success::Failure;
    let routes1 = generate_routes_phase1(craft.clone());
    assert!(routes1.is_empty());
}

// Test step 2 route-finding
#[test]
pub fn test_second_step() {
    let mut craft = crate::craft::Craft::two_star();
    craft.cp = 10000;
    let mut routes1 = generate_routes_phase1(craft);
    let mut craft2 = routes1.pop().unwrap();
    let vact = next_action_phase_2(&craft2);
    assert_eq!(vact, action_vec![ACTIONS.innovation]);
    craft2.run_action(&ACTIONS.innovation);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.basic_touch, ACTIONS.preparatory_touch]
    );
    craft2.run_action(&ACTIONS.basic_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.preparatory_touch, ACTIONS.standard_touch]
    );
    craft2.run_action(&ACTIONS.standard_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.preparatory_touch, ACTIONS.advanced_touch]
    );
    craft2.run_action(&ACTIONS.advanced_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_touch,
            ACTIONS.preparatory_touch,
            ACTIONS.manipulation
        ]
    );
    craft2.run_action(&ACTIONS.manipulation);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(vact, action_vec![ACTIONS.innovation]);
    craft2.run_action(&ACTIONS.innovation);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.basic_touch, ACTIONS.prudent_touch]
    );
    craft2.run_action(&ACTIONS.basic_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.prudent_touch, ACTIONS.standard_touch]
    );
    craft2.run_action(&ACTIONS.prudent_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.basic_touch, ACTIONS.prudent_touch]
    );
    craft2.run_action(&ACTIONS.prudent_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.basic_touch, ACTIONS.prudent_touch]
    );
    craft2.run_action(&ACTIONS.prudent_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(vact, action_vec![ACTIONS.innovation]);
    craft2.run_action(&ACTIONS.innovation);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![ACTIONS.basic_touch, ACTIONS.prudent_touch]
    );
    craft2.run_action(&ACTIONS.prudent_touch);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_touch,
            ACTIONS.prudent_touch,
            ACTIONS.trained_finesse,
            ACTIONS.great_strides
        ]
    );
    craft2.run_action(&ACTIONS.great_strides);

    let vact = next_action_phase_2(&craft2);
    assert_eq!(
        vact,
        action_vec![
            ACTIONS.basic_touch,
            ACTIONS.prudent_touch,
            ACTIONS.byregot_blessing
        ]
    );

    craft2.run_action(&ACTIONS.byregot_blessing);
    craft2.success = crate::specs::Success::Success;
    assert_eq!(next_action_phase_2(&craft2), vec![None]);
    assert_eq!(craft2.progression, 3750);
    assert_eq!(craft2.quality, 9899);
    assert_eq!(craft2.cp, 9322);
}

#[test]
pub fn test_generate_routes_p2() {
    let craft = crate::craft::Craft::two_star();
    let mut routes1 = generate_routes_phase1(craft);
    let craft1 = routes1.pop().unwrap();
    let routes2 = generate_routes_phase2(craft1);
    assert_eq!(routes2.unwrap().len(), 241);

    // Any modification on the solving will break this test, same as before,
    // maybe having a range of expected solution instead is the way to go.

    // Nonetheless the solver test will add qualitative testing so some
    // results will be thoughrouly checked

    // Maybe sampling a handful results and expect them to always exits could also work
}
