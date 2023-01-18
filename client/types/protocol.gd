# Defines types related to messaging protocol
class_name Protocol

enum MessageType {
    # state = {
    #   position: Vector2,
    #   direction: Direction,
    #   velocity: Vector2,
    # }
    # payload = [position.x, position.y, direction, velocity.x, velocity.y]
    PLAYER_STATE = 0,
}

static func serialize_message(type: MessageType, data: Dictionary) -> PackedByteArray:
    match type:
        MessageType.PLAYER_STATE:
            return PackedByteArray([
                MessageType.PLAYER_STATE,
                data.position.x,
                data.position.y,
                data.direction,
                data.velocity.x,
                data.velocity.y,
            ])
        _:
            # This should never happen
            return PackedByteArray()

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
            return {}
