extends Node

const DEFAULT_INSTANCE_ID := 0

func _ready() -> void:
    $UI/InstanceSelector.selected = DEFAULT_INSTANCE_ID
    $UI/InstanceSelector.item_selected.connect($World.initialize)
    $World.initialize(DEFAULT_INSTANCE_ID)
