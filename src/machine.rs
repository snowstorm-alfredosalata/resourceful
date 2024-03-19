use chrono::Utc;
use rand::Rng;

use crate::error::*;
use crate::resource::*;

#[derive(Debug, Default)]
pub struct Machine(pub Resource);

impl Machine {
    pub fn transition(&mut self, event: Event) -> Result<(), Error> {
        match event.event_type {
            ResourceTransition::Start(ev)
                if ev == self.0.state || self.0.state == ResourceState::Inactive =>
            {
                self.open_event(event)
            }

            ResourceTransition::Stop(ev) if ev == self.0.state => self.close_event(event),

            _ => Err(Error::TransitionError {
                current_state: self.0.state,
                transition: event.event_type,
            }),
        }
    }

    fn open_event(&mut self, event: Event) -> Result<(), Error> {
        if self.0.state == ResourceState::Inactive {
            self.0.state = event.event_type.state();
        }

        if self.0.open_events.iter().any(|e| event.matches(e)) {
            Err(Error::WrongTransitionError {
                current_state: self.0.state,
                transition: event.event_type,
                human_resource_id: event.human_resource_id,
            })?
        }

        self.0.open_events.push(event.clone());
        self.0.last_event = Some(event);

        Ok(())
    }

    fn close_event(&mut self, event: Event) -> Result<(), Error> {
        let mut closed_events: Vec<Event> = self
            .0
            .open_events
            .iter()
            .cloned()
            .filter(|e| event.closes(e))
            .collect();

        match closed_events.len() {
            1 => {
                self.0.open_events.retain(|e| !event.closes(e));
                self.0.closed_events.append(&mut closed_events);
                self.0.closed_events.push(event);

                if self.0.open_events.len() == 0 {
                    self.0.state = ResourceState::Inactive;
                }
                Ok(())
            }
            0 => Err(Error::WrongTransitionError {
                current_state: self.0.state,
                transition: event.event_type,
                human_resource_id: event.human_resource_id,
            }),
            _ => panic!("A Stop() transition unexpectedly matched more than 1 event!"),
        }
    }

    fn generate_valid_event(&self, available_hr_res_ids: &Vec<usize>) -> Event {
        use ResourceState::*;
        use ResourceTransition::*;
        let mut rng = rand::thread_rng();

        let working_resource_ids: Vec<usize> = self
            .0
            .open_events
            .iter()
            .map(|x| x.human_resource_id)
            .collect();
        let mut free_hr_res_ids = available_hr_res_ids.clone();
        free_hr_res_ids.retain(|id| !working_resource_ids.contains(id));

        let next_transition;
        if self.0.state == Inactive {
            let n = rng.gen_range(1..=3);
            next_transition = match n {
                1 => Start(Production),
                2 => Start(Setup),
                _ => Start(Blocked),
            };
        } else if self.0.state == Production || self.0.state == Setup {
            if working_resource_ids.len() > 0 && free_hr_res_ids.len() > 0 {
                let n = rng.gen_range(1..=2);
                next_transition = match n {
                    1 => Start(self.0.state),
                    _ => Stop(self.0.state),
                };
            } else if working_resource_ids.len() > 0 {
                next_transition = Stop(self.0.state);
            } else {
                next_transition = Start(self.0.state);
            }
        } else {
            next_transition = Stop(Blocked);
        }

        match next_transition {
            Start(Blocked) | Stop(Blocked) => Event {
                event_type: next_transition,
                start: Utc::now().naive_utc(),
                workcenter_resource_id: 1,
                human_resource_id: 0,
            },
            Start(_) => {
                let random_index = rng.gen_range(0..free_hr_res_ids.len());
                let random_id = free_hr_res_ids[random_index];

                Event {
                    event_type: next_transition,
                    start: Utc::now().naive_utc(),
                    workcenter_resource_id: 1,
                    human_resource_id: random_id,
                }
            }
            Stop(_) => {
                let random_index = rng.gen_range(0..working_resource_ids.len());
                let random_id = working_resource_ids[random_index];

                Event {
                    event_type: next_transition,
                    start: Utc::now().naive_utc(),
                    workcenter_resource_id: 1,
                    human_resource_id: random_id,
                }
            }
        }
    }

    pub fn generate_random_events(&mut self, available_hr_res_ids: &Vec<usize>, length: usize) {
        for _ in 0..length {
            let event = self.generate_valid_event(&available_hr_res_ids);
            self.transition(event).unwrap();
        }
    }

    pub fn import_and_validate_history(&mut self, mut events: Vec<Event>) {
        events

        for event in events.drain(..) {
            self.transition(event).unwrap()
        }
    }
}
