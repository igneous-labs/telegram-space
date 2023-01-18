extends Node

var websocket = WebSocketPeer.new()
const WEBSOCKET_URL = "ws://localhost:1337"
func _ready():
    var err = websocket.connect_to_url(WEBSOCKET_URL)
    if err != OK:
        set_process(false)
        print("failed to connect to %s" % WEBSOCKET_URL)
    else:
        print("connected to %s" % WEBSOCKET_URL)

func _process(delta):
    self.process_ingress()
    if Input.is_action_just_pressed("test_trigger_send"):
        self.process_egress()

func process_ingress():
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
            print("WebSocket closed with code: %d, reason %s. Clean: %s" % [code, reason, code != -1])
            set_process(false) # Stop processing.

func process_egress():
    print("send some stuff")
    websocket.send_text("hello")
