from typing import TypeAlias

Coord: TypeAlias = tuple[int, int]
FloatCoord: TypeAlias = tuple[float, float]
Hailstone: TypeAlias = tuple[Coord, tuple[int, int]]  # start, (x_velocity, y_velocity)
Line: TypeAlias = tuple[float, float]  # (a, b) where y = ax + b


def intersection(first: Line, second: Line) -> FloatCoord | None:
    # https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    a, c = first
    b, d = second
    if a == b:  # lines are parallel
        return None
    x = (d - c) / (a - b)
    y = a * x + c
    return (x, y)


def hailstone_to_line(h: Hailstone) -> Line:
    a = h[1][1] / h[1][0]
    b = h[0][1] - a * h[0][0]
    return (a, b)


def in_bounds(c: FloatCoord, min: int, max: int) -> bool:
    return c[0] >= min and c[0] <= max and c[1] >= min and c[1] <= max


def in_future(c: FloatCoord, h: Hailstone) -> bool:
    if h[1][0] > 0:
        if c[0] < h[0][0]:
            return False  # positive x velocity but x is smaller
    else:
        if c[0] > h[0][0]:
            return False  # negative x velocity but x is bigger
    if h[1][1] > 0:
        if c[1] < h[0][1]:
            return False  # positive y velocity but y is smaller
    else:
        if c[1] > h[0][1]:
            return False  # negative y velocity but y is bigger
    return True


def main():
    min = 200000000000000
    max = 400000000000000
    with open("input.txt", "r") as file:
        hailstones: list[Hailstone] = []
        for line in file.readlines():
            left, right = line.strip().split(" @ ")
            left = left.replace(",", " ").split()
            right = right.replace(",", " ").split()
            hailstones.append(
                ((int(left[0]), int(left[1])), (int(right[0]), int(right[1])))
            )

        hailstone_lines = [hailstone_to_line(h) for h in hailstones]

        result = 0
        for i in range(len(hailstones)):
            for j in range(i + 1, len(hailstones)):
                first = hailstones[i]
                second = hailstones[j]
                first_line = hailstone_lines[i]
                second_line = hailstone_lines[j]
                c = intersection(first_line, second_line)
                if (
                    c is not None
                    and in_bounds(c, min, max)
                    and in_future(c, first)
                    and in_future(c, second)
                ):
                    result += 1

        print(result)


if __name__ == "__main__":
    main()
