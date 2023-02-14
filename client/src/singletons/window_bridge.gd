extends Node

const START_GAME_BRIDGE_MARKER = "StartGameBridgeBeeppack"

var _save_msg_port_ref = JavaScriptBridge.create_callback(Callable(self, "_save_msg_port"))
var _on_msg_ref = JavaScriptBridge.create_callback(Callable(self, "_on_msg"))
var _msg_port

func _ready():
	var window = JavaScriptBridge.get_interface("window")
	if window == null:
		return
	window.addEventListener("message", self._save_msg_port_ref) # Replace with function body.


func _save_msg_port(args) -> void:
	var js_event = args[0]
	var msg_port = js_event.ports[0]
	if msg_port == null or self._msg_port != null or js_event.data != START_GAME_BRIDGE_MARKER:
		return
	self._msg_port = msg_port
	self._msg_port.addEventListener("message", self._on_msg_ref)
	self._msg_port.start() # IMPORTANT
	# TODO: feedback to host that port was received


func _on_msg(args) -> void:
	var js_event = args[0]
	self._msg_port.postMessage(str("Game saw: ", js_event.data))
