extends CharacterBody2D

const SPEED: float = 200.0
@export var animation_dir := &"right"
@export var animation_state := &"idle"

func _physics_process(_delta):
    var dir = Vector2(Input.get_axis("ui_left", "ui_right"), Input.get_axis("ui_up", "ui_down")).normalized()
    self.velocity = SPEED * dir
    if dir.length() != 0:
        self.animation_state = &"walk"
        match Vector2i(dir):
            Vector2i.LEFT:
                self.animation_dir = &"left"
            Vector2i.RIGHT:
                self.animation_dir = &"right"
            Vector2i.UP:
                self.animation_dir = &"up"
            Vector2i.DOWN:
                self.animation_dir = &"down"
    else:
        self.animation_state = &"idle"
    update_animation()
    move_and_slide()

func update_animation():
    $AnimationPlayer.play(&"%s-%s" % [self.animation_state, self.animation_dir])

func setup(initial_position: Vector2):
    self.position = initial_position
