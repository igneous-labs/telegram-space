extends Node2D

const PlayerScene := preload("res://src/scenes/player.tscn")
const RemotePlayerScene := preload("res://src/scenes/remote_player.tscn")

var last_world_state: Dictionary = {}

func _ready() -> void:
    NetworkHandler.received_world_state.connect(self.handle_received_world_state)

func load_level(level: PackedScene) -> void:
    self.add_child(level.instantiate())

func spawn_player(character_state: Dictionary) -> void:
    var p := PlayerScene.instantiate()
    p.setup(character_state)
    self.add_child(p)

func spawn_remote_player(client_id: int, character_state: Dictionary) -> void:
    var r := RemotePlayerScene.instantiate()
    r.setup(client_id, character_state)
    self.add_child(r)

# TODO: world_state timestamp, last_received_world_state_at, world_state_buffer
#       for now, just always update
func handle_received_world_state(world_state: Dictionary) -> void:
    var client_ids_to_despawn := last_world_state.keys()
    for client_id in world_state:
        var remote_player_path := get_remote_player_path(client_id)
        if not self.has_node(remote_player_path):
            # Spawn case
            self.spawn_remote_player(client_id, world_state[client_id])
        else:
            # Update case
            # TODO: rubber banding
            # NOTE: For now, we just use update_character_state.
            # NOTE: If we restrict the movement to 4 direction then character_direction,
            #       then this might be perfectly fine
            self.get_node(remote_player_path).update_character_state(world_state[client_id])
        client_ids_to_despawn.erase(client_id)
    # Despawn disconnected remote players
    for client_id in client_ids_to_despawn:
        var remote_player_path := get_remote_player_path(client_id)
        if self.has_node(remote_player_path):
            self.get_node(remote_player_path).queue_free()
    self.last_world_state = world_state

static func get_remote_player_path(client_id: int) -> NodePath:
    return NodePath("./%s" % RemotePlayer.get_remote_player_name(client_id))
