# Remote Player implements remote user controlled character
# Inherits character
class_name RemotePlayer

extends "res://src/scenes/character.gd"

var client_id: int:
    set(id):
        self.name = get_remote_player_name(id)
        client_id = id

func setup(client_id: int, initial_state: Dictionary) -> void:
    self.client_id = client_id
    super._setup(initial_state)

static func get_remote_player_name(client_id: int) -> String:
    return "RemotePlayer-%s" % str(client_id)
