# NetworkHandler singleton (autoloaded)
extends Node

signal received_level_data(data: Dictionary)
signal received_world_state(world_state: Dictionary)

var websocket := WebSocketPeer.new()
const WEBSOCKET_URL := "ws://localhost:1337"

var initialized := false
# Server's identifier for client, received with ACKNOWLEDGE message (see types/protocol.gd)
var client_id: int = -1

func _ready() -> void:
    self.connect_websocket()

func connect_websocket() -> void:
    var err := websocket.connect_to_url(WEBSOCKET_URL)
    if err != OK:
        set_physics_process(false)
        print("failed to connect to %s" % WEBSOCKET_URL)
    else:
        set_physics_process(true)
        print("connected to %s" % WEBSOCKET_URL)

func _physics_process(_delta: float) -> void:
    websocket.poll()
    match websocket.get_ready_state():
        WebSocketPeer.STATE_OPEN:
            while websocket.get_available_packet_count():
                self.handle_message(websocket.get_packet())
        WebSocketPeer.STATE_CLOSING:
            # Keep polling to achieve proper close.
            pass
        WebSocketPeer.STATE_CLOSED:
            var code = websocket.get_close_code()
            var reason = websocket.get_close_reason()
            print("WebSocket closed with code: %d, reason %s. Clean: %s" % [
                code,
                reason,
                code != -1,
            ])
            self.initialized = false
            set_physics_process(false) # Stop processing.

func handle_message(payload: PackedByteArray) -> void:
    #print("Message payload: ", payload)
    var message = Protocol.deserialize_message(payload)
    match message.type:
        Protocol.MessageType.ACKNOWLEDGE:
            self.client_id = message.data.client_id
            self.initialized = true
        Protocol.MessageType.WORLD_STATE:
            message.data.erase(client_id)
            emit_signal(&"received_world_state", message.data)
        Protocol.MessageType.LEVEL_DATA:
            emit_signal(&"received_level_data", message.data)
        _:
            print("Unhandled message: ", message)

func publish_player_state(state: Dictionary) -> void:
    if not self.initialized:
        return
    websocket.send(
        Protocol.serialize_message(Protocol.MessageType.PLAYER_STATE, state),
        WebSocketPeer.WRITE_MODE_BINARY,
    )
