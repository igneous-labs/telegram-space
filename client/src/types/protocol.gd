# Defines types related to messaging protocol
class_name Protocol

enum MessageType {
    # NOTE: not sure how client_id should be securely
    #       generated and passed, so for now server assigns client_id
    #       and passes it back on connection as a ACKNOWLEDGE message
    # server -> client only
    # payload = [type (1), client_id(2)]
    ACKNOWLEDGE = 0,
    # client -> server only
    # state = {
    #   position: Vector2,
    #   direction: Common.Direction,
    #   status: Common.CharacterStatus,
    # }
    # payload = [type (1), position (12), direction (1), status (1)]
    PLAYER_STATE = 1,
    WORLD_STATE = 2,
    
    # server -> client
    # data = {
    #   level: PackedScene
    # }
    # payload = [type (1), data ()]
    LEVEL_DATA = 3,
}

static func serialize_message(type: MessageType, data: Dictionary) -> PackedByteArray:
    var payload := PackedByteArray()
    match type:
        MessageType.PLAYER_STATE:
            payload.resize(15)
            payload.encode_u8(0, MessageType.PLAYER_STATE)  # 1 byte;   (1)
            payload.encode_var(1, data.position)            # 12 bytes; (13)
            payload.encode_u8(13, data.direction)           # 1 byte;   (14)
            payload.encode_u8(14, data.status)              # 1 byte;   (15)
            #print("player state: (%s, %s, %s)" % [data.position, data.direction, data.status])
        _:
            # These messages are not expected to be sent to server
            push_error("unexpected message type was serialized")
    return payload

static func deserialize_message(payload: PackedByteArray) -> Dictionary:
    var type := payload.decode_u8(0);
    match type:
        MessageType.ACKNOWLEDGE:
            return {
                "type": MessageType.ACKNOWLEDGE,
                "data": {
                    "client_id": payload.decode_u16(1),
                },
            }
        MessageType.WORLD_STATE:
            var world_state_data = bytes_to_var(payload.slice(1))
            var data = {}
            for player_state in world_state_data:
                data[player_state.decode_u16(0)] = {
                    "position": player_state.decode_var(2),
                    "direction": player_state.decode_u8(14),
                    "status":  player_state.decode_u8(15),
                }
            return {
                "type": MessageType.WORLD_STATE,
                "data": data,
            }
        MessageType.LEVEL_DATA:
            print("received level data")
            print("not yet implemented")
            return {
                "type": MessageType.LEVEL_DATA,
                "data": {}
            }
        _:
            # These messages are not expected to come into client
            push_error("unexpected message type was deserialized")
            return {}
