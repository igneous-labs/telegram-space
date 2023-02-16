extends Node2D

var _label_text: String

func _ready():
    self.get_node("VBoxContainer/Label").text = self._label_text

func setup(label_text: String) -> void:
    self._label_text = label_text
