# (Local) Player implements user controlled character
# Inherits character
extends "res://src/scenes/character.gd"

const SPEED: float = 200.

func _physics_process(_delta: float) -> void:
	var input_vector := Vector2(
		Input.get_axis("ui_left", "ui_right"),
		Input.get_axis("ui_up", "ui_down")
	).normalized()
	self.velocity = SPEED * input_vector
	self.update_character_state(input_vector)
	self.update_animation()
	self.publish_player_state()
	self.move_and_slide()

func publish_player_state() -> void:
	NetworkHandler.publish_player_state({
		"position": self.position,
		"direction": self.character_direction,
		"status": self.character_status,
	})
