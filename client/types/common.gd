# Defines common types shared across files
class_name Common

enum Direction {
    NONE = -1,  # used to indicate error
    LEFT = 0,
    RIGHT = 1,
    UP = 2,
    DOWN = 3,
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
            return &""

# Convert vector to Common.Direction variant.
# Used for consistly mark directions (e.g. animation name in AnimationPlayer)
# NOTE: if the direction does not match any cardinal directions Direction.NONE is returned
static func vec_to_dir(vec: Vector2) -> Direction:
    match vec.normalized():
        Vector2.LEFT:
            return Direction.LEFT
        Vector2.RIGHT:
            return Direction.RIGHT
        Vector2.UP:
            return Direction.UP
        Vector2.DOWN:
            return Direction.DOWN
        _:
            return Direction.NONE
