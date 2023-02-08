extends Node

func _ready() -> void:
    $World.modulate = Color.BLACK
    $World.load_level(LevelDataManager.get_level(0))
    $World.spawn_player({
        "position": Vector2(100, 100),
        "direction": Common.Direction.LEFT,
        "status": Common.CharacterStatus.IDLE,
    })
    create_tween()\
        .tween_property($World, "modulate", Color.WHITE, 1)\
        .set_trans(Tween.TRANS_EXPO)
