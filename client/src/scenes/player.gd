# (Local) Player implements user controlled character
# Inherits character
extends "res://src/scenes/character.gd"

const SPEED: float = 200.

# NOTE: a nullable variant that when set to a Vector2 gets processed by _physics_process
var move_target = null

func _ready() -> void:
    super()
    # TODO: find a better way to initialize chat_user_id field
    self.initialize_chat_user_id()

func _unhandled_input(event: InputEvent) -> void:
    if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT and not event.pressed:
        var player_screen_position = self.get_global_transform_with_canvas().get_origin()
        # NOTE: use to_local to adjust for the scale (inherited from World)
        self.move_target = self.position + self.to_local(event.position) - self.to_local(player_screen_position)

func _physics_process(_delta: float) -> void:
    var input_vector := self.get_input_vector()
    self.velocity = SPEED * input_vector
    self.update_player_state(input_vector)
    self.update_animation()
    self.publish_player_state()
    self.move_and_slide()

func get_input_vector() -> Vector2:
    var input_vector := Vector2(
        Input.get_axis("ui_left", "ui_right"),
        Input.get_axis("ui_up", "ui_down")
    ).normalized()
    if input_vector != Vector2.ZERO:
        self.move_target = null
        return input_vector
    if self.move_target != null:
        if self.position.distance_to(self.move_target) < 1.0:
            self.move_target = null
        else:
            return self.position.direction_to(self.move_target)
    return Vector2.ZERO

func update_player_state(input_vector: Vector2):
    if input_vector.length() != 0:
        self.character_status = Common.CharacterStatus.WALK
        var dir = Common.vec_to_dir(input_vector)
        if dir != Common.Direction.NONE:
            self.update_character_direction(dir)
    else:
        self.character_status = Common.CharacterStatus.IDLE

func publish_player_state() -> void:
    NetworkHandler.send_message(Protocol.MessageType.PLAYER_STATE, {
        "position": self.position,
        "direction": self.character_direction,
        "status": self.character_status,
    })

# TODO: find a better way to initialize chat_user_id field
# NOTE: meant to be used as a corutine
func initialize_chat_user_id() -> void:
    while self.chat_user_id.is_empty():
        if not WindowBridge.chat_user_id.is_empty():
            self.chat_user_id = WindowBridge.chat_user_id
        else:
            await self.get_tree().create_timer(1.0).timeout

func setup(initial_state: Dictionary) -> void:
    super._setup(initial_state)
