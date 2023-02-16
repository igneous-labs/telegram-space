# Character defines common behavior of character class
# inherited by (local) player and remote_player
extends CharacterBody2D

const SpeechBubbleManager := preload("res://src/scenes/speech_bubble_manager.gd")

signal speech_bubble_msg(msg: String)

@export var character_direction: Common.Direction
@export var character_status: Common.CharacterStatus
var _speech_bubble_manager: SpeechBubbleManager

# DELETEME: this just spawns a test speech bubble after 1s
func _ready() -> void:
    await self.get_tree().create_timer(1.0).timeout
    emit_signal(&"speech_bubble_msg", "hello")

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
    var speech_bubble_manager = SpeechBubbleManager.new()
    speech_bubble_msg.connect(speech_bubble_manager.on_speech_bubble_msg)
    self.add_child(speech_bubble_manager)
