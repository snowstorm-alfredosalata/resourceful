use thiserror::Error as ThisError;

use crate::resource::{ResourceState, ResourceTransition};

#[allow(dead_code)]
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("cannot perform transition {current_state:?} from state {transition:?}!")]
    TransitionError {
        current_state: ResourceState,
        transition: ResourceTransition,
    },

    #[error("cannot perform {transition:?} with human resource {human_resource_id:?}! Current state: {current_state:?}")]
    WrongTransitionError {
        current_state: ResourceState,
        transition: ResourceTransition,
        human_resource_id: usize,
    },

    #[error("unknown error")]
    Unknown,
}
