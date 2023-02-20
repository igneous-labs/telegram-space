extends Node2D

const DEFAULT_DURATION := 5.0

func _ready():
    self.set_show_duration(DEFAULT_DURATION)
    self.clear()
    $ClearTimer.timeout.connect(func (): self.clear())

func clear():
    self.hide()
    $VBoxContainer/Label.text = ""

func set_show_duration(duration: float) -> void:
    $ClearTimer.wait_time = duration

func show_message(msg: String) -> void:
    $VBoxContainer/Label.text = msg
    $ClearTimer.start()
    self.show()
