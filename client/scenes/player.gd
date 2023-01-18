# (Local) Player implements user controlled character
# Inherits character
extends CharacterBody2D

const SPEED: float = 200.0
@export var animation_dir := Common.Direction.LEFT
@export var animation_state := &"idle"

func _physics_process(_delta: float) -> void:
    var input_vector := Vector2(
        Input.get_axis("ui_left", "ui_right"),
        Input.get_axis("ui_up", "ui_down")
    ).normalized()
    self.velocity = SPEED * input_vector
    if input_vector.length() != 0:
        self.animation_state = &"walk"
        self.animation_dir = Common.vec_to_dir(input_vector)
    else:
        self.animation_state = &"idle"
    self.update_animation()
    self.publish_player_state()
    self.move_and_slide()

func publish_player_state() -> void:
    NetworkHandler.publish_player_state({
        "position": self.position,
        "direction": self.animation_dir,
        "velocity": self.velocity,
    })

func update_animation() -> void:
    $AnimationPlayer.play(&"%s-%s" % [
        self.animation_state,
        Common.dir_to_strn(self.animation_dir),
    ])

func setup(initial_position: Vector2) -> void:
    self.position = initial_position
