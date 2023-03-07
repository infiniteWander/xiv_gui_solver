use crate::action::{Action, ACTIONS};
use crate::craft::Craft;
use crate::specs::Success;
use std::collections::VecDeque;
use std::ops::Not;

macro_rules! action_vec {
    ($($tt:expr),*) => { vec![ $(Some(&$tt),)*]};
}

/// Find the next authorised action for this step of the craft
pub fn next_action_picker_1<'a>(craft: &Craft<'a>) -> Vec<Option<&'a Action>> {
    if craft.success != Success::Pending {
        return vec![None];
    }
    let mut available_actions = Vec::<Option<&'a Action>>::new();
    let mut forbidden_actions = Vec::<Option<&'a Action>>::new();

    // Optimize the first four steps for massive time save
    if craft.step_count == 0 {
        return action_vec![ACTIONS.muscle_memory];
    }
    if craft.step_count == 1 {
        return action_vec![ACTIONS.manipulation];
    }

    // Prune some actions if not requested by --long
    if craft.step_count == 2 {
        return action_vec![/* ACTIONS.waste_not, ACTIONS.waste_not_ii, */ ACTIONS.veneration];
    }
    if craft.step_count == 3 {
        available_actions.append(&mut action_vec![
            ACTIONS.waste_not_ii /* ,ACTIONS.waste_not */
        ])
    }

    // Groundwork mostly for wn / mm
    if craft.buffs.waste_not > 0 || craft.buffs.muscle_memory > 0 {
        available_actions.append(&mut action_vec![ACTIONS.groundwork])
    }

    // Forbidding actions depending on buffs
    if craft.buffs.muscle_memory > 0 {
        forbidden_actions.append(&mut action_vec![
            ACTIONS.basic_synthesis,
            ACTIONS.careful_synthesis,
            ACTIONS.prudent_synthesis,
            ACTIONS.delicate_synthesis
        ])
    }
    if craft.buffs.waste_not > 0 {
        forbidden_actions.append(&mut action_vec![ACTIONS.prudent_synthesis])
    }
    available_actions.append(&mut action_vec![
        ACTIONS.basic_synthesis,
        ACTIONS.careful_synthesis,
        ACTIONS.prudent_synthesis,
        ACTIONS.delicate_synthesis
    ]);

    // ACTIONS.groundwork for the --pls
    if craft.args.desperate {
        available_actions.append(&mut action_vec![ACTIONS.groundwork]);
        available_actions.append(&mut action_vec![ACTIONS.masters_mend]);
    }

    // For long first run
    if craft.step_count > 8 && craft.buffs.waste_not == 0 {
        available_actions.append(&mut action_vec![ACTIONS.waste_not_ii, ACTIONS.waste_not])
    };

    // Pruning the actions with the forbidden ones
    let mut result_actions = Vec::<Option<&'a Action>>::new();
    for action in available_actions {
        if !forbidden_actions.contains(&action)
            && action.unwrap().can_use(&craft)
            && result_actions
                .iter()
                .any(|x| x.unwrap() == action.unwrap())
                .not()
        {
            result_actions.push(action);
        }
    }

    result_actions
}

/// Find all routes that can finish in one more action
pub fn generate_routes_phase1<'a>(craft: Craft<'a>) -> Vec<Craft<'a>> {
    let mut queue = Vec::new();
    queue.push(craft);
    let mut routes = Vec::new();
    while !queue.is_empty() {
        let craft = queue.pop().unwrap();
        for action in next_action_picker_1(&craft) {
            let mut craft = craft.clone();

            match action {
                Some(a) => craft.run_action(a),
                None => break,
            };

            let remaining_prog = (craft.recipe.progress as f32 - craft.progression as f32)
                / craft.get_base_progression() as f32;
            if remaining_prog <= 2.0 {
                if remaining_prog <= 0.0 {
                    continue;
                } else if 0.0 < remaining_prog && remaining_prog <= 1.2 {
                    // pass         // Will finish with basicSynth2 (free)
                } else if 1.2 < remaining_prog && remaining_prog <= 1.2 {
                    craft.cp -= 7; // Will finish with carefulSynthesis (7cp)
                } else if 1.2 < remaining_prog && remaining_prog <= 2.0 {
                    craft.cp -= 12; // Will finish with observe+focusedSynthesis (12cp)
                }
                craft.durability -= 10; // Save the final step, since we always aim to end with -5dur, it doesn't matter
                routes.push(craft);
                continue;
            }

            // Keep adding initial routes with several pass, we must be able to finish in one step
            // 8 Seemed to be a good initial guess with enouth to use a good amount of WN / WNII to get close
            // A higher value will yield more initial guesses with minimal benefits
            // A lover value will yield less initial guesses and might spent too much CP to get to the required progression
            if craft.step_count < craft.args.depth {
                queue.push(craft);
            }
        }
    }
    routes
}

/// Find the list of authorised actions for this step
pub fn next_action_phase_2<'a>(craft: &Craft<'a>) -> Vec<Option<&'a Action>> {
    let mut available_actions = vec![
        &ACTIONS.basic_touch,
        &ACTIONS.prudent_touch,
        &ACTIONS.preparatory_touch,
    ];
    let mut forbidden_actions = Vec::new();
    if craft.success != Success::Pending {
        return vec![None];
    }
    if craft.buffs.innovation > 0 {
        forbidden_actions.push(&ACTIONS.innovation);
    } else {
        if craft.buffs.inner_quiet >= 2 {
            forbidden_actions.append(&mut vec![
                &ACTIONS.basic_touch,
                &ACTIONS.standard_touch,
                &ACTIONS.advanced_touch,
                &ACTIONS.trained_finesse,
                &ACTIONS.prudent_touch,
                &ACTIONS.preparatory_touch,
                &ACTIONS.byregot_blessing,
            ]);
        }
        available_actions.push(&ACTIONS.innovation);
    }
    if craft.buffs.manipulation == 0
        && craft.buffs.basic_touch == 0
        && craft.buffs.standard_touch == 0
        && craft.buffs.inner_quiet < 8
    {
        available_actions.push(&ACTIONS.manipulation);
    }
    if craft.buffs.waste_not > 0 {
        available_actions.push(&ACTIONS.preparatory_touch);
        forbidden_actions.push(&ACTIONS.prudent_touch);
    } else {
        available_actions.push(&ACTIONS.prudent_touch);
        forbidden_actions.push(&ACTIONS.preparatory_touch);
    }
    if craft.buffs.basic_touch > 0 {
        available_actions.push(&ACTIONS.standard_touch);
        forbidden_actions.push(&ACTIONS.basic_touch);
    }
    if craft.buffs.standard_touch > 0 {
        available_actions.push(&ACTIONS.advanced_touch);
        forbidden_actions.push(&ACTIONS.basic_touch);
    }

    if craft.buffs.inner_quiet >= craft.args.byregot_step {
        // 10
        available_actions.push(&ACTIONS.trained_finesse);
        available_actions.push(&ACTIONS.great_strides);
    }
    if craft.buffs.great_strides > 0 {
        forbidden_actions.push(&ACTIONS.trained_finesse);
        forbidden_actions.push(&ACTIONS.great_strides);
        if craft.buffs.innovation > 0 {
            available_actions.push(&ACTIONS.byregot_blessing);
        }
    }
    // if craft.durability + 30 < craft.recipe.durability as i32{
    //     available_actions.push(&ACTIONS.masters_mend);
    // }
    let mut final_actions: Vec<Option<&Action>> = Vec::new();

    for action in available_actions {
        if !forbidden_actions.contains(&action)
            && !final_actions.iter().any(|a| a.unwrap() == action)
        {
            final_actions.push(Some(action));
        }
    }
    final_actions
}

/// Apply all actions to the current route
pub fn generate_routes_phase2<'a>(craft: Craft<'a>) -> Option<Vec<Craft<'a>>> {
    let mut queue = VecDeque::new();

    let mut top_route: Craft<'a> = craft.clone(); // Default route, no hq on that one
    let mut top_routes: Vec<Craft<'a>> = vec![];

    queue.push_back(craft);

    while !queue.is_empty() {
        let _craft = queue.pop_front().unwrap();
        for action in next_action_phase_2(&_craft) {
            if _craft.success != Success::Pending
                || action.is_none()
                || !action.unwrap().can_use(&_craft)
            {
                continue;
            }
            let mut craft = _craft.clone();
            craft.run_action(action.unwrap());
            if action.unwrap() == &ACTIONS.byregot_blessing {
                #[cfg(not(feature = "fast"))]
                if top_route.quality >= craft.recipe.quality {
                    top_routes.push(craft.clone()); // Me memory
                }
                if top_route.quality < craft.quality {
                    #[cfg(feature = "fast")]
                    top_routes.push(craft.clone());
                    top_route = craft;
                }
            } else {
                queue.push_back(craft);
            }
        }
    }
    // Let's not forget the best result
    top_routes.push(top_route);
    Some(top_routes)
}

#[cfg(test)]
mod test {
    use crate::solver::{generate_routes_phase1, next_action_picker_1, ACTIONS};

    #[test]
    pub fn test_first_step() {
        let mut craft = crate::craft::Craft::three_star();
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
    }

    #[test]
    pub fn test_routes_p1() {
        let craft = crate::craft::Craft::three_star();
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
        match route {
            None => (),
            Some(..) => panic!(),
        }
    }
}
