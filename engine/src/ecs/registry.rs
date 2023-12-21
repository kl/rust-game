use crate::ecs::ecs::{Component, ComponentId, EntityId, Signature};
use std::any;
use std::any::{Any, TypeId};
use std::collections::HashMap;

use uuid::Uuid;

pub struct Registry {
    components: HashMap<ComponentId, Box<dyn Any>>,
    signatures: HashMap<EntityId, Signature>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            signatures: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.components.clear();
        self.signatures.clear();
    }

    pub fn add_entity(&mut self) -> EntityId {
        let id = Uuid::new_v4();
        self.signatures.insert(id, Signature::new());
        id
    }

    pub fn is_entity_alive(&self, entity: EntityId) -> bool {
        self.signatures.contains_key(&entity)
    }

    pub fn entities<C: 'static>(&self) -> Vec<EntityId> {
        let type_id = TypeId::of::<C>();
        self.signatures
            .iter()
            .filter(|(_, sig)| sig.contains(&type_id))
            .map(|(id, _)| *id)
            .collect()
    }

    #[inline]
    pub fn assert_entity_exists(&self, entity: EntityId) {
        if !self.signatures.contains_key(&entity) {
            panic!("entity {} does not exist in registry", entity);
        }
    }

    pub fn component_for_entity<C: 'static>(&self, entity: EntityId) -> &C {
        self.assert_entity_exists(entity);

        let components = self.get_components::<C>().unwrap_or_else(|| {
            panic!(
                "no Component<{}> exists in the registry",
                any::type_name::<C>()
            )
        });

        &components
            .iter()
            .find(|c| c.entity_id == entity)
            .unwrap_or_else(|| {
                panic!(
                    "Entity[{}] did not hava a Component<{}>",
                    entity,
                    any::type_name::<C>()
                )
            })
            .data
    }

    pub fn component_for_entity_mut<C: 'static>(&mut self, entity: EntityId) -> &mut C {
        self.assert_entity_exists(entity);

        let components = self.get_components_mut::<C>().unwrap_or_else(|| {
            panic!(
                "no Component<{}> exists in the registry",
                any::type_name::<C>()
            )
        });

        &mut components
            .iter_mut()
            .find(|c| c.entity_id == entity)
            .unwrap_or_else(|| {
                panic!(
                    "Entity[{}] did not hava a Component<{}>",
                    entity,
                    any::type_name::<C>()
                )
            })
            .data
    }

    pub fn add_component<C: 'static>(&mut self, entity: EntityId, component_type: C) {
        match self.signatures.get_mut(&entity) {
            Some(sig) => {
                let type_id = TypeId::of::<C>();
                sig.insert(type_id);
                self.get_or_insert_empty_components::<C>().push(Component {
                    entity_id: entity,
                    data: component_type,
                });
            }
            None => panic!("entity {} does not exist in registry", entity),
        }
    }

    pub fn remove_component<C: 'static>(&mut self, entity: EntityId) {
        if let Some(sig) = self.signatures.get_mut(&entity) {
            sig.remove(&TypeId::of::<C>());
        }
        if let Some(components) = self.get_components_mut::<C>() {
            if let Some(index) = components.iter().position(|c| c.entity_id == entity) {
                components.swap_remove(index);
            }
        }
    }

    pub fn has_component<C: 'static>(&self, entity: EntityId) -> bool {
        if let Some(sig) = self.signatures.get(&entity) {
            sig.contains(&TypeId::of::<C>())
        } else {
            false
        }
    }

    pub fn get_or_insert_empty_components<C: 'static>(&mut self) -> &mut Vec<Component<C>> {
        self.components
            .entry(TypeId::of::<C>())
            .or_insert_with(|| Box::<Vec<Component<C>>>::default())
            .downcast_mut()
            .expect("TypeId mapped to wrong type")
    }

    pub fn get_components<C: 'static>(&self) -> Option<&Vec<Component<C>>> {
        self.components
            .get(&TypeId::of::<C>())
            .map(|e| e.downcast_ref().expect("TypeId mapped to wrong type"))
    }

    pub fn get_components_mut<C: 'static>(&mut self) -> Option<&mut Vec<Component<C>>> {
        self.components
            .get_mut(&TypeId::of::<C>())
            .map(|e| e.downcast_mut().expect("TypeId mapped to wrong type"))
    }
}
