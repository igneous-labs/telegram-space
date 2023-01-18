# Defines types related to messaging protocol
class_name Protocol

enum MessageType {
    # state = {
    #   position: Vector2,
    #   direction: Direction,
    #   status: CharacterStatus,
    # }
    # payload = [position, direction, status]
    PLAYER_STATE = 0,
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
    # TODO
    var type = payload[0];
    match type:
        MessageType.PLAYER_STATE:
            return {
                "position": Vector2(),
                "direction": Common.Direction.RIGHT,
                "velocity": Vector2(),
            }
        _:
            # This should never happen
            push_error("given payload does not contain a valid message")
            return {}
