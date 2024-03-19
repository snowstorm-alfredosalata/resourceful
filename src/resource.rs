use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceState {
    Inactive,
    Setup,
    Production,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceTransition {
    Start(ResourceState),
    Stop(ResourceState),
}

impl ResourceTransition {
    pub fn state(&self) -> ResourceState {
        match *self {
            ResourceTransition::Start(v) => v,
            ResourceTransition::Stop(v) => v,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Event {
    pub event_type: ResourceTransition,
    pub start: NaiveDateTime,
    pub workcenter_resource_id: usize,
    pub human_resource_id: usize,
}

impl Event {
    pub fn matches(&self, other: &Event) -> bool {
        if self.event_type == other.event_type && other.human_resource_id == self.human_resource_id
        {
            return true;
        }

        false
    }

    pub fn closes(&self, other: &Event) -> bool {
        if let ResourceTransition::Stop(state) = self.event_type {
            return state == other.event_type.state()
                && other.human_resource_id == self.human_resource_id;
        }

        false
    }
}

#[derive(Debug)]
pub struct Resource {
    pub state: ResourceState,
    pub last_event: Option<Event>,
    pub open_events: Vec<Event>,
    pub closed_events: Vec<Event>,
}

impl Default for Resource {
    fn default() -> Self {
        Self {
            state: ResourceState::Inactive,
            last_event: None,
            open_events: Vec::default(),
            closed_events: Vec::default(),
        }
    }
}
