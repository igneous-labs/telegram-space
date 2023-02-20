extends VBoxContainer

const SpeechBubble := preload("res://src/scenes/components/speech_bubble.tscn")

func spawn_speech_bubble(text: String) -> void:
    var bubble = SpeechBubble.instantiate()
    bubble.set_text(text)
    self.add_child(bubble)
