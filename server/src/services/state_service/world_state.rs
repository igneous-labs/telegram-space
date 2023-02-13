use std::{collections::HashMap, time::Instant};

use crate::{
    consts::WORLD_STATE_SYNC_INTERVAL,
    protocol::{ClientId, InstanceId, LevelId, PlayerStateData},
};

#[derive(Debug)]
struct Instance {
    level_id: LevelId,
    last_synced_at: Option<Instant>,
    last_updated_at: Option<Instant>,
    state: HashMap<ClientId, PlayerStateData>,
}

impl Instance {
    fn new(level_id: &LevelId) -> Self {
        Self {
            level_id: level_id.to_owned(),
            last_synced_at: None,
            last_updated_at: None,
            state: HashMap::new(),
        }
    }
}

pub struct WorldState {
    instances: HashMap<InstanceId, Instance>,
    client_instance: HashMap<ClientId, InstanceId>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            client_instance: HashMap::new(),
        }
    }

    pub fn has_instance(&self, instance_id: &InstanceId) -> bool {
        self.instances.contains_key(instance_id)
    }

    pub fn has_client(&self, client_id: &ClientId) -> bool {
        self.client_instance.contains_key(client_id)
    }

    /// precondition:
    ///  - `has_client(client_id) == true`
    pub fn has_player_state(&self, client_id: &ClientId) -> bool {
        self.get_instance_state(self.client_instance.get(client_id).unwrap())
            .contains_key(client_id)
    }

    /// precondition:
    ///  - `has_instance(instance_id) == true`
    pub fn get_instance_state(
        &self,
        instance_id: &InstanceId,
    ) -> &HashMap<ClientId, PlayerStateData> {
        &self.instances.get(instance_id).unwrap().state
    }

    /// precondition:
    ///  - `has_instance(instance_id) == true`
    pub fn get_instance_level_id(&self, instance_id: &InstanceId) -> &LevelId {
        &self.instances.get(instance_id).unwrap().level_id
    }

    /// precondition:
    ///  - `client_id` exists in world state: client_instance points to instance where client_id exists
    pub fn get_player_state_data(&self, client_id: &ClientId) -> &PlayerStateData {
        let instance_id = self.client_instance.get(client_id).unwrap();
        self.get_instance_state(instance_id).get(client_id).unwrap()
    }

    /// precondition:
    ///  - `has_instance(instance_id) == false`
    pub fn add_instance(&mut self, instance_id: &InstanceId, level_id: &LevelId) {
        self.instances
            .insert(instance_id.to_owned(), Instance::new(level_id));
    }

    pub fn add_player_to_instance(&mut self, client_id: &ClientId, instance_id: &InstanceId) {
        self.client_instance
            .insert(client_id.to_owned(), instance_id.to_owned());
    }

    /// precondition:
    ///  - either
    ///    - `has_client(client_id) == true` and `state.has_player_state(client_id) == false`
    ///    - `client_id` exists in world state: client_instance points to instance where client_id exists
    pub fn update_player_state(
        &mut self,
        client_id: &ClientId,
        player_state_data: PlayerStateData,
    ) {
        let instance_id = self.client_instance.get(client_id).unwrap();
        let instance = self.instances.get_mut(instance_id).unwrap();
        instance
            .state
            .insert(client_id.to_owned(), player_state_data);
        self.client_instance
            .insert(client_id.to_owned(), instance_id.to_owned());
        instance.last_updated_at = Some(Instant::now());
    }

    /// precondition:
    ///  - `next_instance_id` exists in world state
    ///  - `has_client(client_id) == true`
    pub fn move_player_to_instance(&mut self, next_instance_id: &InstanceId, client_id: &ClientId) {
        let prev_instance_id = self.client_instance.get(client_id).unwrap();
        let prev_instance = self.instances.get_mut(prev_instance_id).unwrap();
        prev_instance.state.remove(client_id);
        self.client_instance
            .insert(client_id.to_owned(), next_instance_id.to_owned());
        prev_instance.last_updated_at = Some(Instant::now());
    }

    /// precondition:
    ///  - `client_id` exists in world state
    pub fn remove_player_state(&mut self, client_id: &ClientId) {
        let instance = self
            .instances
            .get_mut(self.client_instance.get(&client_id).unwrap())
            .unwrap();
        instance.state.remove(client_id);
        instance.last_updated_at = Some(Instant::now());
    }

    /// return instances that needs to be synced
    pub fn get_instance_ids_to_sync(&self) -> Vec<InstanceId> {
        let now = Instant::now();
        self.instances
            .iter()
            .filter_map(
                |(instance_id, instance): (&InstanceId, &Instance)| -> Option<InstanceId> {
                    if instance.last_synced_at.is_none()
                        || (instance.last_updated_at.is_some()
                            && instance.last_updated_at.unwrap() > instance.last_synced_at.unwrap())
                            && now.duration_since(instance.last_synced_at.unwrap())
                                > WORLD_STATE_SYNC_INTERVAL
                    {
                        Some(instance_id.to_owned())
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    pub fn update_last_synced_at(&mut self, instance_ids: &[InstanceId]) {
        let now = Instant::now();
        for instance_id in instance_ids {
            self.instances.get_mut(instance_id).unwrap().last_synced_at = Some(now);
        }
    }
}
