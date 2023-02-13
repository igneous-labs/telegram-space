# Defines types related to messaging protocol
class_name Protocol

enum MessageType {
    # NOTE: not sure how client_id should be securely
    #       generated and passed, so for now server assigns client_id
    #       and passes it back on connection as a ACKNOWLEDGE message
    # Server acknowledges for client connection, assigns client_id
    # server -> client only
    # data = {
    #   client_id: u16,
    # }
    # payload = [type (1), client_id(2)]
    ACKNOWLEDGE = 0,
    # Client sends (local) player state to server
    # client -> server
    # data = {
    #   position: Vector2,
    #   direction: Common.Direction,
    #   status: Common.CharacterStatus,
    # }
    # payload = [type (1), position (12), direction (1), status (1)]
    PLAYER_STATE = 1,
    # TODO: write message spec
    # Server broadcasts world states (collection of player states) to clients
    # server -> client
    # data = {
    #   [client_id: u16]: {
    #     position: Vector2,
    #     direction: Common.Direction,
    #     status: Common.CharacterStatus,
    #   },
    #   ...
    # }
    # payload = [type (1), ]
    WORLD_STATE = 2,
    # Client requests for LEVEL_DATA
    # client -> server
    # data = {
    #   level_id: u64,
    # }
    # payload = [type (1), level_id(8)]
    REQUEST_LEVEL = 3,
    # Server responses to REQUEST_LEVEL
    # server -> client
    # data = {
    #   level_id: u64,
    #   decompressed_size: u32,
    #   compressed_level_data: PackedByteArray,
    # }
    # payload = [type (1), level_id(8), decompressed_size (4), compressed_level_data(decompressed_size)]
    LEVEL_DATA = 4,
    # Client registers player to an instance
    # NOTE: This includes both initial registeration and updating current instance
    # client -> server
    # data = {
    #   instance_id: u64,
    # }
    # payload = [type(1), instance_id(8)]
    PLAYER_INSTANCE = 5,
    # Server acknowledges for player instance registration
    # server -> client
    # data = {
    #   level_id: u64,
    # }
    # payload = [type(1), level_id(8)]
    PLAYER_INSTANCE_ACKNOWLEDGE = 6,
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
        MessageType.REQUEST_LEVEL:
            payload.resize(9)
            payload.encode_u8(0, MessageType.REQUEST_LEVEL) # 1 byte;   (1)
            payload.encode_u64(1, data.level_id)            # 8 byte;   (9)
        MessageType.PLAYER_INSTANCE:
            payload.resize(9)
            payload.encode_u8(0, MessageType.PLAYER_INSTANCE)  # 1 byte;   (1)
            payload.encode_u64(1, data.instance_id)            # 8 byte;   (9)
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
            var decompressed_size = payload.decode_u32(9)
            return {
                "type": MessageType.LEVEL_DATA,
                "data": {
                    "level_id": payload.decode_u64(1),
                    "level_data": payload.slice(13).decompress(decompressed_size, FileAccess.CompressionMode.COMPRESSION_ZSTD),
                }
            }
        MessageType.PLAYER_INSTANCE_ACKNOWLEDGE:
            var decompressed_size = payload.decode_u32(9)
            return {
                "type": MessageType.PLAYER_INSTANCE_ACKNOWLEDGE,
                "data": {
                    "level_id": payload.decode_u64(1),
                }
            }
        _:
            # These messages are not expected to come into client
            push_error("unexpected message type was deserialized")
            return {}
