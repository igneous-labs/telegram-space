# Character defines common behavior of character class
# inherited by (local) player and remote_player
extends CharacterBody2D

const SpeechBubble := preload("res://src/scenes/components/speech_bubble.tscn")

@export var character_direction: Common.Direction
@export var character_status: Common.CharacterStatus
var chat_user_id: String = ""

func _ready() -> void:
    WindowBridge.received_window_message.connect(self.handle_window_message)

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

func handle_window_message(message_type: WindowBridge.MessageType, data: Dictionary) -> void:
    match message_type:
        WindowBridge.MessageType.CHAT_MESSAGE:
            if data.sender_id == self.chat_user_id:
                print("ok: ", data.body)
                $SpeechBubbleContainer.spawn_speech_bubble(data.body)
