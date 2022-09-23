pub struct Entities(Vec<Entity>);
impl Entities {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.0.iter_mut()
    }

    pub fn entities_mut(&mut self) -> &mut [Entity] {
        &mut self.0
    }
}

pub struct Tags;
impl Tags {
    pub fn copy_to(&self, other: &mut Self) {
        todo!()
    }
}

pub struct Trigger;
impl Trigger {
    pub fn event_result(&self, event: &Event) -> TriggerResult {
        todo!()
    }
}
pub struct TriggerResult {
    pub modifier: Option<Modifier>,
    pub trigger: Option<Trigger>,
    pub event: Option<Event>,
}

pub struct Modifier {
    pub name: String,
    pub ops: ModifierOps,
}
pub enum ModifierOps {}

pub struct EntityId(u64);

pub struct Entity {
    id: EntityId,
    original_tags: Tags,
    current_tags: Tags,
    triggers: List<Trigger>,
    modifiers: List<Modifier>,
}
impl Entity {
    pub fn handle_event(&mut self, event: &Option<Event>, triggers: &mut Stack<Event>) {
        // Handle modifiers
        let mut added_modifiers = List::new();
        {
            // Reset applied modifiers
            self.original_tags.copy_to(&mut self.current_tags);

            // Now apply all modifiers
            for modifier in self.modifiers.iter() {
                todo!("apply modifier")
            }
        }

        // Handle triggers
        let mut added_triggers = List::new();
        if let Some(event) = event {
            for trigger in self.triggers.iter() {
                let result = trigger.event_result(event);
                if let Some(modifier) = result.modifier {
                    added_modifiers.push(modifier);
                }
                if let Some(trigger) = result.trigger {
                    added_triggers.push(trigger);
                }
                if let Some(event) = result.event {
                    triggers.push(event);
                }
            }
        }

        // Finally store all new data
        self.modifiers.append(&mut added_modifiers);
        self.triggers.append(&mut added_triggers);
    }
}

pub struct List<T>(Vec<T>);
impl<T> List<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
    }
    fn new() -> Self {
        Self(vec![])
    }

    fn append(&mut self, other: &mut List<T>) {
        self.0.append(&mut other.0);
    }
}

pub struct Stack<T>(Vec<T>);
impl<T> Stack<T> {
    pub fn peek(&self) -> Option<&T> {
        if self.0.is_empty() {
            None
        } else {
            Some(&self.0[self.0.len() - 1])
        }
    }
    fn push(&mut self, t: T) {
        self.0.push(t);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}

pub struct Event;

#[repr(C)]
pub struct GameState {
    entities: Entities,
    events: Stack<Event>,
}
impl GameState {
    fn update_state(self) -> Self {
        let mut entities = self.entities;
        let mut events = self.events;

        let event = events.pop();
        for entity in entities.iter_mut() {
            entity.handle_event(&event, &mut events);
        }

        Self { entities, events }
    }
}

pub fn update(state: &mut GameState) {
    update_state(state);
    calculate_player_actions();
}

fn update_state(state: &mut GameState) {}

fn apply_player_actions() {}

fn calculate_player_actions() {}
