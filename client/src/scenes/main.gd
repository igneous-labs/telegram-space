extends Node

@onready var world = $World

func _ready() -> void:
    self.world.load_level(load("res://test_level.res"))
    self.world.spawn_player({
        "position": Vector2(100, 100),
        "direction": Common.Direction.LEFT,
        "status": Common.CharacterStatus.IDLE,
    })
