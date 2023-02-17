# Waits until all singletons gets initialized and initial data is received from the server
# Transitions to main scene
extends Control

const MAIN_SCENE := "res://src/scenes/main.tscn"

func _ready() -> void:
    $C/AnimatedSprite2D.play()
    $C/LoadingStateLabel.text = "connecting to server"
    self.check_loading()

func check_loading() -> void:
    # wait until NetworkHandler is initialized
    # TODO: consider changing this to await (see how PlayerInstance awaits PlayerInstanceAcknowledge)
    if not NetworkHandler.is_initialized:
        await NetworkHandler.initialized
    # TODO: find a better way to sync with chatapp on chat_user_id message
    #       for now just wait until you see a value
    # NOTE: we are doing this to insure that we'll always send PLAYER_CHAT_USER_ID message
    #       before sending PLAYER_INSTANCE message
    while WindowBridge.chat_user_id.is_empty():
        $C/LoadingStateLabel.text = "initializing chat connection"
        await self.get_tree().create_timer(0.2).timeout
    NetworkHandler.send_message(Protocol.MessageType.PLAYER_CHAT_USER_ID, { "chat_user_id": WindowBridge.chat_user_id })
    await NetworkHandler.received_player_chat_user_id_acknowledge
    $C/LoadingStateLabel.text = "initializing world"
    self.load_main_scene()

func load_main_scene() -> void:
    create_tween()\
        .tween_property(self, "modulate", Color.BLACK, 1)\
        .set_trans(Tween.TRANS_EXPO)\
        .finished.connect(get_tree().change_scene_to_file.bind(MAIN_SCENE))
