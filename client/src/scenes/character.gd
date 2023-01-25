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

func update_character_state(character_state: Dictionary) -> void:
    self.character_direction = character_state.direction
    self.character_status = character_state.status
    self.position = character_state.position
    self.update_animation()

func _setup(initial_state: Dictionary) -> void:
    self.update_character_state(initial_state)
