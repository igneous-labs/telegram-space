# NetworkHandler singleton (autoloaded)
extends Node

signal initialized
signal received_world_state(world_state: Dictionary)
signal received_player_instance_acknowledge(level_id: int)
signal received_player_chat_user_id_acknowledge

var websocket := WebSocketPeer.new()
const WEBSOCKET_URL := "ws://localhost:1337"

var is_initialized := false
# Server's identifier for client, received with ACKNOWLEDGE message (see types/protocol.gd)
var client_id: int = -1

func _ready() -> void:
    self.connect_websocket()

func connect_websocket() -> void:
    var err := self.websocket.connect_to_url(WEBSOCKET_URL)
    if err != OK:
        self.set_physics_process(false)
        print("failed to connect to %s" % WEBSOCKET_URL)
    else:
        self.set_physics_process(true)
        print("connected to %s" % WEBSOCKET_URL)

func _physics_process(_delta: float) -> void:
    self.websocket.poll()
    match self.websocket.get_ready_state():
        WebSocketPeer.STATE_OPEN:
            while self.websocket.get_available_packet_count():
                # TODO: consider making this concurrent
                #self.emit_signal(&"received_message", self.websocket.get_packet())
                self.handle_message(self.websocket.get_packet())
        WebSocketPeer.STATE_CLOSING:
            # Keep polling to achieve proper close.
            pass
        WebSocketPeer.STATE_CLOSED:
            var code = self.websocket.get_close_code()
            var reason = self.websocket.get_close_reason()
            print("WebSocket closed with code: %d, reason %s. Clean: %s" % [
                code,
                reason,
                code != -1,
            ])
            self.is_initialized = false
            self.set_physics_process(false) # Stop processing.

func handle_message(payload: PackedByteArray) -> void:
    var message = Protocol.deserialize_message(payload)
    match message.type:
        Protocol.MessageType.ACKNOWLEDGE:
            # TODO: change this to how PLAYER_INSTANCE awaits for its acknowledge message
            self.client_id = message.data.client_id
            self.is_initialized = true
            emit_signal(&"initialized")
        Protocol.MessageType.WORLD_STATE:
            message.data.world_state_data.erase(self.client_id)
            message.data.client_chat_user_ids.erase(self.client_id)
            emit_signal(&"received_world_state", message.data)
        Protocol.MessageType.LEVEL_DATA:
            # TODO: change this to how PLAYER_INSTANCE awaits for its acknowledge message
            LevelDataManager.insert_level(message.data.level_id, message.data.level_data)
        Protocol.MessageType.PLAYER_INSTANCE_ACKNOWLEDGE:
            emit_signal(&"received_player_instance_acknowledge", message.data.level_id)
        Protocol.MessageType.PLAYER_CHAT_USER_ID_ACKNOWLEDGE:
            emit_signal(&"received_player_chat_user_id_acknowledge")
        _:
            print("Unhandled message: ", message)

func send_message(message_type: Protocol.MessageType, message_data: Dictionary) -> void:
    if not self.is_initialized:
        return
    self.websocket.send(
        Protocol.serialize_message(message_type, message_data),
        WebSocketPeer.WRITE_MODE_BINARY,
    )

# TODO: consider adding
#  - send_message_and_await
#  - send_message_and_callback
