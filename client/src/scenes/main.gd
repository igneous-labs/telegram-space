extends Node

func _ready() -> void:
    pass

func _unhandled_input(event):
    if event is InputEventKey and event.is_action_pressed("place_tile_exp"):
        print("place a tile on user position: ", $World/Player.position)
        $World/Level.place_custom_tile(Vector2i(12, 14), $World/Player.position)
