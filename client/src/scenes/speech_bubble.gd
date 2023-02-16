extends Node2D

func set_text(text: String) -> void:
     self.get_node("VBoxContainer/Label").text = text
