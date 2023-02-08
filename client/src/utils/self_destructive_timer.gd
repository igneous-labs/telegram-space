extends Timer

func _init(name: String, wait_time: float) -> void:
    super()
    self.name = name
    self.wait_time = wait_time
    self.one_shot = true
    self.timeout.connect(self.queue_free)
