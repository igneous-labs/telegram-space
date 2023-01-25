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
    self.update_player_state(input_vector)
    self.update_animation()
    self.publish_player_state()
    self.move_and_slide()

func update_player_state(input_vector: Vector2):
    if input_vector.length() != 0:
        self.character_status = Common.CharacterStatus.WALK
        var dir = Common.vec_to_dir(input_vector)
        if dir != Common.Direction.NONE:
            self.character_direction = dir
    else:
        self.character_status = Common.CharacterStatus.IDLE

func publish_player_state() -> void:
    NetworkHandler.publish_player_state({
        "position": self.position,
        "direction": self.character_direction,
        "status": self.character_status,
    })

func setup(initial_state: Dictionary) -> void:
    super._setup(initial_state)
