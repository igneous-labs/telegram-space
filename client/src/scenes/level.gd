extends Node2D

@onready var user_modifiable_tilemap = $Custom

func _global_to_map(global: Vector2) -> Vector2i:
    return user_modifiable_tilemap.local_to_map(user_modifiable_tilemap.to_local(global))

func place_custom_tile(atlas_coord: Vector2i, global: Vector2):
    $Custom.set_cell(0, self._global_to_map(global), 0, atlas_coord)
