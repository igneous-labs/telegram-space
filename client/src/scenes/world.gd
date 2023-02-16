extends Node2D

const PlayerScene := preload("res://src/scenes/player.tscn")
const RemotePlayerScene := preload("res://src/scenes/remote_player.tscn")

var last_world_state: Dictionary = {}
var current_instance_id: int = -1 # NOTE: not used yet

func _ready() -> void:
    NetworkHandler.received_world_state.connect(self.handle_received_world_state)

func initialize(instance_id: int) -> void:
    self.modulate = Color.BLACK
    self.despawn_player()
    NetworkHandler.send_message(Protocol.MessageType.PLAYER_INSTANCE, { "instance_id": instance_id })
    var level_id = await NetworkHandler.received_player_instance_acknowledge
    await self.load_level(level_id)
    self.spawn_player({
        "position": Vector2(100, 100),
        "direction": Common.Direction.LEFT,
        "status": Common.CharacterStatus.IDLE,
    })
    self.current_instance_id = instance_id
    create_tween()\
        .tween_property(self, "modulate", Color.WHITE, 1)\
        .set_trans(Tween.TRANS_EXPO)


func load_level(level_id: int) -> void:
    # TODO: change this to how PLAYER_INSTANCE awaits for its acknowledge message
    while not LevelDataManager.has_or_request_level(level_id):
        await get_tree().create_timer(1).timeout
    if self.has_node(^"Level"):
        var current_level = self.get_node(^"Level")
        current_level.queue_free()
        await current_level.tree_exited
    var level = LevelDataManager.get_level(level_id).instantiate()
    level.name = &"Level"
    self.add_child(level)

func despawn_player() -> void:
    if self.has_node(^"Player"):
        var current_player = self.get_node(^"Player")
        current_player.queue_free()
        await current_player.tree_exited

func spawn_player(character_state: Dictionary) -> void:
    var p := PlayerScene.instantiate()
    p.setup(character_state)
    self.add_child(p)

func spawn_remote_player(client_id: int, character_state: Dictionary) -> void:
    var r := RemotePlayerScene.instantiate()
    r.setup(client_id, character_state)
    self.add_child(r)

func despawn_remote_players(client_ids: Array) -> void:
    for client_id in client_ids:
        var remote_player_path := get_remote_player_path(client_id)
        if self.has_node(remote_player_path):
            self.get_node(remote_player_path).queue_free()

# TODO: world_state timestamp, last_received_world_state_at, world_state_buffer
#       for now, just always update
func handle_received_world_state(data: Dictionary) -> void:
    var world_state = data.world_state_data
    # TODO: use this to spawn remote player
    var client_chat_user_ids = data.client_chat_user_ids
    var client_ids_to_despawn := last_world_state.keys()
    for client_id in world_state:
        var remote_player_path := get_remote_player_path(client_id)
        if not self.has_node(remote_player_path):
            # Spawn case
            # DELETEME
            print("spawning remote player for client %s (matrix id: %s)" % [client_id, client_chat_user_ids[client_id]])
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
    self.despawn_remote_players(client_ids_to_despawn)
    self.last_world_state = world_state

static func get_remote_player_path(client_id: int) -> NodePath:
    return NodePath("./%s" % RemotePlayer.get_remote_player_name(client_id))
