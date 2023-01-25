# Character defines common behavior of character class
# inherited by (local) player and remote_player
extends CharacterBody2D

@export var character_direction: Common.Direction
@export var character_status: Common.CharacterStatus

func update_animation() -> void:
	$AnimationPlayer.play(&"%s-%s" % [
		Common.char_status_to_strn(self.character_status),
		Common.dir_to_strn(self.character_direction),
	])

func update_character_state(input_vector: Vector2):
	if input_vector.length() != 0:
		self.character_status = Common.CharacterStatus.WALK
		var dir = Common.vec_to_dir(input_vector)
		if dir != Common.Direction.NONE:
			self.character_direction = dir
	else:
		self.character_status = Common.CharacterStatus.IDLE

func setup(initial_position: Vector2) -> void:
	self.character_direction = Common.Direction.LEFT
	self.character_status = Common.CharacterStatus.IDLE
	self.position = initial_position
	self.update_animation()
