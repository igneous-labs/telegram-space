extends Node

const Player := preload("res://scenes/player.tscn")

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
    var p := Player.instantiate()
    p.setup(Vector2(100, 100))
    $World.add_child(p)
