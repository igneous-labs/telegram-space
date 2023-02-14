# Defines common types shared across files
class_name Common

enum CharacterStatus {
    IDLE = 0,
    WALK = 1,
}

# Convert Common.CharacterStatus variant to stringname.
# Used for consistly mark character status (e.g. animation name in AnimationPlayer)
static func char_status_to_strn(status: CharacterStatus) -> StringName:
    match status:
        CharacterStatus.IDLE:
            return &"idle"
        CharacterStatus.WALK:
            return &"walk"
        _:
            # This should never happen
            push_error("given character status is not in the type of CharacterStatus")
            return &""

enum Direction {
    NONE = 0,  # used to indicate error
    LEFT = 1,
    RIGHT = 2,
    UP = 3,
    DOWN = 4,
}

# Convert Common.Direction variant to stringname.
# Used for consistly mark directions (e.g. animation name in AnimationPlayer)
static func dir_to_strn(dir: Direction) -> StringName:
    match dir:
        Direction.LEFT:
            return &"left"
        Direction.RIGHT:
            return &"right"
        Direction.UP:
            return &"up"
        Direction.DOWN:
            return &"down"
        _:
            # This should never happen
            push_error("given direction is not in the type of Direction")
            return &""

# Convert vector to Common.Direction variant.
# Used for consistly mark directions (e.g. animation name in AnimationPlayer)
# NOTE: if the direction does not match any cardinal directions then
#       Direction.NONE is returned
static func vec_to_dir(vec: Vector2) -> Direction:
    var abs = vec.normalized().abs()
    if abs.x > abs.y:
        return Direction.RIGHT if vec.x > 0 else Direction.LEFT
    else:
        return Direction.DOWN if vec.y > 0 else Direction.UP
