# Spawns expiring speech bubbles above its parent node

extends Node

const SpeechBubble := preload("res://src/scenes/speech_bubble.tscn")

const SPEECH_BUBBLE_CLEAR_SECS := 5.0

var _clear_timer: SceneTreeTimer

func on_speech_bubble_msg(msg: String) -> void:
    var bubble = SpeechBubble.instantiate()
    bubble.setup(msg)
    self._replace_curr_bubble(bubble)
    
func _replace_curr_bubble(new_bubble: Node2D) -> void:
    if self._clear_timer != null:
        self._clear_timer.time_left = 0.
        self._clear_timer = null
    self._clear_timer = self.get_tree().create_timer(SPEECH_BUBBLE_CLEAR_SECS)
    var parent = self.get_parent()
    parent.add_child(new_bubble)
    await self._clear_timer.timeout
    parent.remove_child(new_bubble)
