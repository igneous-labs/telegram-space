# LevelDataManager singleton (autoloaded) manages the local cache of level scenes
extends Node

const Hash := preload("res://src/utils/hash.gd")
const SelfDestructiveTimer := preload("res://src/utils/self_destructive_timer.gd")
const REQUEST_DELAY_SEC := 5.0

# TODO: consider cache miss sending a singnal (e.g. received_level) on ready
# e.g. in caller code:
# while not LevelDataManager.has_or_request(some_level_id):
#     await LevelDataManager.received_level
# Note that since this relies on network, this loop can get stuck forever

# key: level_id
# value: data
# type level_id = int32
# type data = Dictionary {
#   hash: int32,
#   level_scene: PackedScene,
# }
var _data: Dictionary = {}

func insert_level(level_id: int, level_data: PackedByteArray) -> void:
    var level_data_hash = Hash.hash_packed_byte_array(level_data).decode_s32(0)
    if not self._data.has(level_id) or level_data_hash != self._data[level_id].hash:
        var level_scene = PackedScene.new()
        print(bytes_to_var_with_objects(level_data))
        level_scene._bundled = bytes_to_var_with_objects(level_data)
        self._data[level_id] = {
            "hash": level_data_hash,
            "level_scene": level_scene,
        }

func get_level(level_id: int) -> PackedScene:
    return self._data[level_id].level_scene if self.has(level_id) else null

# Check if the level exists, if not send REQUEST_LEVEL message to server
# returns:
#  - false if either request was sent or waiting for the request delay to finish
#  - true if level_id already exists
func has_or_request_level(level_id: int) -> bool:
    if self.has(level_id):
        return true
    if not self.has_node(NodePath(str(level_id))):
        var request_delay := SelfDestructiveTimer.new(str(level_id), REQUEST_DELAY_SEC)
        self.add_child(request_delay)
        NetworkHandler.send_message(Protocol.MessageType.REQUEST_LEVEL, { "level_id": level_id })
        request_delay.start()
    return false

func has(level_id: int) -> bool:
    return self._data.has(level_id)
