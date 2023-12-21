use std::any::TypeId;
use std::collections::HashSet;

use uuid::Uuid;

pub type EntityId = Uuid;
pub type ComponentId = TypeId;
pub type Signature = HashSet<ComponentId>;

pub struct Component<T> {
    pub entity_id: EntityId,
    pub data: T,
}
