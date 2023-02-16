extends Node

const DEFAULT_INSTANCE_ID := 0

func _ready() -> void:
    # DELETEME: test sending player's matrix id to the server
    NetworkHandler.send_message(Protocol.MessageType.PLAYER_CHAT_USER_ID, { "chat_user_id": "helloworld" })
    $UI/InstanceSelector.selected = DEFAULT_INSTANCE_ID
    $UI/InstanceSelector.item_selected.connect($World.initialize)
    $World.initialize(DEFAULT_INSTANCE_ID)
