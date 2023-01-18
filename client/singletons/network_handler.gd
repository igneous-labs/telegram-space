extends Node

var websocket := WebSocketPeer.new()
const WEBSOCKET_URL := "ws://localhost:1337"

var initialized := false

func _ready() -> void:
    var err = websocket.connect_to_url(WEBSOCKET_URL)
    if err != OK:
        set_physics_process(false)
        print("failed to connect to %s" % WEBSOCKET_URL)
    else:
        self.initialized = true
        print("connected to %s" % WEBSOCKET_URL)

func _physics_process(_delta) -> void:
    websocket.poll()
    match websocket.get_ready_state():
        WebSocketPeer.STATE_OPEN:
            while websocket.get_available_packet_count():
                print("Packet: ", websocket.get_packet())
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
            set_physics_process(false) # Stop processing.

func publish_player_state(state: Dictionary) -> void:
    if not self.initialized:
        return
    websocket.send(
        Protocol.serialize_message(Protocol.MessageType.PLAYER_STATE, state),
        WebSocketPeer.WRITE_MODE_BINARY,
    )
