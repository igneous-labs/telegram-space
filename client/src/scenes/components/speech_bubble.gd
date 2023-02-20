extends MarginContainer

const DEFAULT_DURATION := 5.0

func _ready():
    self.set_show_duration(DEFAULT_DURATION)
    $ClearTimer.timeout.connect(func (): self.clear())
    $ClearTimer.start()

# TODO: this is where to trigger freeing animation
#       for now just queue_free
func clear() -> void:
    self.queue_free()

# TODO: set show duration based on the length of text?
func set_show_duration(duration: float) -> void:
    $ClearTimer.wait_time = duration

func set_text(text: String) -> void:
    $M/Label.text = text
