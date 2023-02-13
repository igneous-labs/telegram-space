extends Node

const DEFAULT_INSTANCE_ID := 0

#func _ready() -> void:
#    $UI/InstanceSelector.selected = DEFAULT_INSTANCE_ID
#    $UI/InstanceSelector.item_selected.connect($World.initialize)
#    $World.initialize(DEFAULT_INSTANCE_ID)

func _ready() -> void:
    var f = FileAccess.open("./test.txt", FileAccess.WRITE)
    f.store_string("hello world")
    f.flush()
    f = null
    f = FileAccess.open("./test.txt", FileAccess.READ)
    print(f.get_as_text())
