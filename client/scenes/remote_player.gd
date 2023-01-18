# Remote Player implements remote user controlled character
# Inherits character
extends CharacterBody2D

const SPEED: float = 200.0
@export var animation_dir := Common.Direction.LEFT
@export var animation_state := &"idle"

func _ready():
    self.update_animation()

func _handle_remote_input():
    # TODO
    pass

func update_animation() -> void:
    $AnimationPlayer.play(&"%s-%s" % [
        self.animation_state,
        Common.dir_to_strn(self.animation_dir),
    ])

func setup(initial_position: Vector2) -> void:
    self.position = initial_position
