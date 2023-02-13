# Waits until all singletons gets initialized and initial data is received from the server
# Transitions to main scene
extends Control

const MAIN_SCENE := "res://src/scenes/main.tscn"

func _ready() -> void:
    $C/AnimatedSprite2D.play()
    $C/LoadingStateLabel.text = "connecting to server"
    $CheckLoadingInterval.timeout.connect(self.check_loading)
    $CheckLoadingInterval.start()

func check_loading() -> void:
    if not NetworkHandler.initialized:
        return
    $CheckLoadingInterval.stop()
    $C/LoadingStateLabel.text = "initializing world"
    self.load_main_scene()

func load_main_scene() -> void:
    create_tween()\
        .tween_property(self, "modulate", Color.BLACK, 1)\
        .set_trans(Tween.TRANS_EXPO)\
        .finished.connect(get_tree().change_scene_to_file.bind(MAIN_SCENE))
