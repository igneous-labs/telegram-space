extends Node2D

const Player := preload("res://src/scenes/player.tscn")
const RemotePlayer := preload("res://src/scenes/remote_player.tscn")

func _ready() -> void:
    self.spawn_player()
    # DELETEME: used for testing remote player scene
    self.spawn_remote_player()

func spawn_remote_player() -> void:
    var r := RemotePlayer.instantiate()
    r.setup(Vector2(200, 150))
    self.add_child(r)

func spawn_player() -> void:
    var p := Player.instantiate()
    p.setup(Vector2(100, 100))
    self.add_child(p)
