# Waits until all singletons gets initialized and initial data is received from the server
# Transitions to main scene
extends Control

const MAIN_SCENE := "res://src/scenes/main.tscn"
# TODO: current level to be waited should be described in state somewhere
#       for now it just waits for level_id 0
const LEVEL_ID := 0

func _ready() -> void:
    $C/AnimatedSprite2D.play()
    $C/LoadingStateLabel.text = "connecting to server"
    $CheckLoadingInterval.timeout.connect(self.check_loading)

func check_loading() -> void:
    if not NetworkHandler.initialized:
        return
    if not LevelDataManager.has_or_request_level(LEVEL_ID):
        $C/LoadingStateLabel.text = "fetching level data"
        return
    $CheckLoadingInterval.stop()
    $C/LoadingStateLabel.text = "initializing"
    self.load_main_scene()

func load_main_scene() -> void:
    create_tween()\
        .tween_property(self, "modulate", Color.BLACK, 1)\
        .set_trans(Tween.TRANS_EXPO)\
        .finished.connect(get_tree().change_scene_to_file.bind(MAIN_SCENE))
