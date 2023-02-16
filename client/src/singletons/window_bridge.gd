extends Node

const MATRIX_ID_KEY = "matrix_id"
const BODY_KEY = "body"

var _on_init_msg_ref = JavaScriptBridge.create_callback(Callable(self, "_on_init_msg"))
var _on_msg_ref = JavaScriptBridge.create_callback(Callable(self, "_on_msg"))

var _msg_port

func _ready():
    var window = JavaScriptBridge.get_interface("window")
    if window == null:
        return
    window.addEventListener("message", self._on_init_msg_ref)


# Init msg schema
# { matrix_id: string }
func _on_init_msg(args) -> void:
    var js_event = args[0]
    var msg_port = js_event.ports[0]
    var parsed = JSON.parse_string(js_event.data)
    if msg_port == null \
        or self._msg_port != null \
        or parsed == null \
        or typeof(parsed) != TYPE_DICTIONARY \
        or not parsed.has(MATRIX_ID_KEY) \
        or typeof(parsed[MATRIX_ID_KEY]) != TYPE_STRING:
            return
    
    # TODO: send this to server
    var _matrix_id = parsed[MATRIX_ID_KEY]
    
    self._msg_port = msg_port
    self._msg_port.addEventListener("message", self._on_msg_ref)
    self._msg_port.start() # IMPORTANT
    
    # Feedback to host that port was received by echoing
    self._msg_port.postMessage(js_event.data)


# Message msg schema
# { matrix_id: string, body: string }
func _on_msg(args) -> void:
    var js_event = args[0]
    var parsed = JSON.parse_string(js_event.data)
    if parsed == null \
        or typeof(parsed) != TYPE_DICTIONARY \
        or not parsed.has_all([MATRIX_ID_KEY, BODY_KEY]) \
        or typeof(parsed[MATRIX_ID_KEY]) != TYPE_STRING \
        or typeof(parsed[BODY_KEY]) != TYPE_STRING:
            return
    # TODO: render message instead of echoing back to source
    self._msg_port.postMessage(str(parsed[MATRIX_ID_KEY], ": ", parsed[BODY_KEY]))

