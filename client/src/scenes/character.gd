# Character defines common behavior of character class
# inherited by (local) player and remote_player
extends CharacterBody2D

const SpeechBubble := preload("res://src/scenes/speech_bubble.tscn")

const SPEECH_BUBBLE_CLEAR_SECS := 5.0

@export var character_direction: Common.Direction
@export var character_status: Common.CharacterStatus
var _speech_bubble_clear_timer: SceneTreeTimer

# DELETEME: this just spawns a test speech bubble after 1s
func _ready() -> void:
    await self.get_tree().create_timer(1.0).timeout
    self._replace_speech_bubble("hello")

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
    
func _replace_speech_bubble(text: String) -> void:
    var sb = self.get_node("SpeechBubble")
    sb.set_text(text)
    sb.show()
    if self._speech_bubble_clear_timer == null:
        self._speech_bubble_clear_timer = self.get_tree().create_timer(SPEECH_BUBBLE_CLEAR_SECS)
        self._speech_bubble_clear_timer.timeout.connect(self._on_clear_timer_timeout)
    else:
        self._speech_bubble_clear_timer.time_left = SPEECH_BUBBLE_CLEAR_SECS

func _on_clear_timer_timeout() -> void:
    self._speech_bubble_clear_timer = null
    self.get_node("SpeechBubble").hide()    
