# Character defines common behavior of character class
# inherited by (local) player and remote_player
extends CharacterBody2D

const SpeechBubble := preload("res://src/scenes/components/speech_bubble.tscn")

@export var character_direction: Common.Direction
@export var character_status: Common.CharacterStatus
var is_right: bool = true;
var chat_user_id: String = "":
    # DELETEME
    set(id):
        print("The chat_user_id of Character %s is %s" % [self, id])
        chat_user_id = id

func _ready() -> void:
    WindowBridge.received_window_message.connect(self.handle_window_message)

func update_animation() -> void:
    $CharTexture.scale.x = -1 if self.is_right else 1
    $CharTexture/AnimationPlayer.play(&"%s" % [
        Common.char_status_to_strn(self.character_status),
    ])
    
func update_character_direction(new_dir: Common.Direction) -> void:
    self.character_direction = new_dir
    if self.character_direction == Common.Direction.LEFT:
        self.is_right = false
    elif self.character_direction == Common.Direction.RIGHT:
        self.is_right = true

func update_character_state(character_state: Dictionary) -> void:
    self.update_character_direction(character_state.direction)
    self.character_status = character_state.status
    self.position = character_state.position
    self.update_animation()

func _setup(initial_state: Dictionary) -> void:
    self.update_character_state(initial_state)

func handle_window_message(message_type: WindowBridge.MessageType, data: Dictionary) -> void:
    match message_type:
        WindowBridge.MessageType.CHAT_MESSAGE:
            if data.sender_id == self.chat_user_id:
                print("ok: ", data.body)
                $SpeechBubbleContainer.spawn_speech_bubble(data.body)
