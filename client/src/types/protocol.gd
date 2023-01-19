# Defines types related to messaging protocol
class_name Protocol

enum MessageType {
    # NOTE: not sure how client_id should be securely
    #       generated and passed, so for now server assigns client_id
    #       and passes it back on connection as a ACKNOWLEDGE message
    # server -> client only
    # payload = [type (1), client_id(8)]
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
        _:
            # This should never happen
            push_error("given message type is not in the type of MessageType")
    #print("payload length: ", len(payload), "; payload: ", payload)
    return payload

static func deserialize_message(payload: PackedByteArray) -> Dictionary:
    var type = payload.decode_u8(0);
    match type:
        MessageType.PLAYER_STATE:
            return {
                "type": MessageType.PLAYER_STATE,
                "data": {
                    "position": payload.decode_var(1),
                    "direction": payload.decode_u8(13),
                    "status": payload.decode_u8(14),
                },
            }
        MessageType.ACKNOWLEDGE:
            return {
                "type": MessageType.ACKNOWLEDGE,
                "data": {
                    "client_id": payload.decode_u64(1),
                },
            }
        _:
            # This should never happen
            push_error("given payload does not contain a valid message")
            return {}
