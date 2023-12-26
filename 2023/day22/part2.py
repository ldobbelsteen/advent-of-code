import copy
from typing import TypeAlias

Coord2D: TypeAlias = tuple[int, int]
Coord3D: TypeAlias = tuple[int, int, int]
Brick: TypeAlias = tuple[Coord3D, int, int]  # base, direction, length


def brick_from_str(s: str) -> Brick:
    start_raw, end_raw = s.split("~")
    start = [int(v) for v in start_raw.split(",")]
    end = [int(v) for v in end_raw.split(",")]
    assert all([start[i] <= end[i] for i in range(max(len(start), len(end)))])

    result: Brick | None = None
    for direction in range(0, 3):
        if start[direction] < end[direction]:
            assert result is None
            result = (
                (start[0], start[1], start[2]),
                direction,
                end[direction] - start[direction],
            )
    if result is None:
        return ((start[0], start[1], start[2]), 0, 0)
    else:
        return result


def brick_cubes(b: Brick) -> list[Coord3D]:
    return [move_coord(b[0], b[1], offset) for offset in range(b[2] + 1)]


def move_brick(b: Brick, direction: int, delta: int) -> Brick:
    return (move_coord(b[0], direction, delta), b[1], b[2])


def move_coord(c: Coord3D, direction: int, delta: int) -> Coord3D:
    coord_list = list(c)
    coord_list[direction] += delta
    return (coord_list[0], coord_list[1], coord_list[2])


def conflicting_tops(b: Brick, tops: dict[Coord2D, tuple[int, Brick]]) -> set[Brick]:
    result: set[Brick] = set()
    for cube in brick_cubes(b):
        coord = (cube[0], cube[1])
        if coord in tops:
            top_z, top_b = tops[(cube[0], cube[1])]
            if top_z == cube[2]:
                result.add(top_b)
    return result


def move_down(
    b: Brick, tops: dict[Coord2D, tuple[int, Brick]]
) -> tuple[Brick, set[Brick]]:
    """Move brick down until it hits any top. Returns the new brick along with
    the bricks it is now supported by."""

    conflicting = conflicting_tops(b, tops)
    assert len(conflicting) == 0
    while len(conflicting) == 0 and b[0][2] > 0:
        b = move_brick(b, 2, -1)  # move down by one
        conflicting = conflicting_tops(b, tops)

    support = conflicting
    result = move_brick(b, 2, 1)  # move one up to get conflictless position
    return result, support


def would_fall(
    b: Brick, supports: dict[Brick, set[Brick]], supported_by: dict[Brick, set[Brick]]
) -> int:
    """Compute the number of other bricks that would fall if a brick were
    to be disintegrated. Destroys the supports and supported by dicts,
    so pass copies."""
    if b not in supports or len(supports[b]) == 0:
        return 0

    result = 0
    while len(supports[b]) > 0:
        s = supports[b].pop()
        supported_by[s].remove(b)
        if len(supported_by[s]) == 0:
            result += would_fall(s, supports, supported_by)
            result += 1  # the brick itself also falls
    return result


def main():
    with open("input.txt", "r") as file:
        # Get unsettled brick sorted by z-coord decreasing.
        unsettled = sorted(
            [brick_from_str(line.strip()) for line in file.readlines()],
            key=lambda b: b[0][2],
            reverse=True,
        )

        tops: dict[Coord2D, tuple[int, Brick]] = {}
        supported_by: dict[Brick, set[Brick]] = {}

        # Settle all unsettled bricks.
        while len(unsettled) > 0:
            lowest = unsettled.pop()
            new_settled, support = move_down(lowest, tops)
            supported_by[new_settled] = support

            # Update tops with new settled brick
            for cube in brick_cubes(new_settled):
                coord: Coord2D = (cube[0], cube[1])
                if coord not in tops or cube[2] > tops[coord][0]:
                    tops[coord] = (cube[2], new_settled)

        # Invert supported by dictionary to the bricks a brick supports.
        supports: dict[Brick, set[Brick]] = {}
        for brick, support in supported_by.items():
            for sup in support:
                if sup not in supports:
                    supports[sup] = set()
                supports[sup].add(brick)

        result = sum(
            [
                would_fall(b, copy.deepcopy(supports), copy.deepcopy(supported_by))
                for b in supported_by.keys()
            ]
        )
        print(result)


if __name__ == "__main__":
    main()
