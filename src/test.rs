use chrono::Utc;

use super::*;
use crate::resource::{Event, ResourceState::*, ResourceTransition::*};

macro_rules! check_transitions {
    (
        check_valid;
        $( $hr_id:literal : $state:expr);*;
    ) => {
        {
            let mut machine = Machine::default();
            let mut results = Vec::default();
            $(
                results.push(machine.transition(Event { event_type: $state, start: Utc::now().naive_utc(), workcenter_resource_id: 1, human_resource_id: $hr_id }));
            )*
            assert!(results.iter().all(|result| result.is_ok()));
        }
    };
    (
        check_invalid;
        $( $hr_id:literal : $state:expr);*;
    ) => {
        {
            let mut machine = Machine::default();
            let mut results = Vec::default();
            $(
                results.push(machine.transition(Event { event_type: $state, start: Utc::now().naive_utc(), workcenter_resource_id: 1, human_resource_id: $hr_id }));
            )*
            assert!(results.iter().any(|result| result.is_err()));
        }
    };
}

#[test]
fn test_transitions() {
    check_transitions! {
        check_valid;
        1: Start(Setup);
        1: Stop(Setup);
        2: Start(Setup);
        2: Stop(Setup);
        1: Start(Production);
        2: Start(Production);
        1: Stop(Production);
        2: Stop(Production);
    }
}

#[test]
fn test_generate_random_transitions() {
    let mut machine = Machine::default();
    let available_employees: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    machine.generate_random_events(&available_employees, 500)
}
