from typing import Tuple


def pattern_to_str(pattern: list[list[bool]]):
    """Print pattern back into the original format."""
    return "\n".join(
        ["".join(["#" if is_rock else "." for is_rock in row]) for row in pattern]
    )


def find_horizontal_mirror(
    pattern: list[list[bool]], disallowed: int | None
) -> int | None:
    """Find a horizontal mirror in a pattern and return the number of rows above the line."""

    def rec(remaining: list[list[bool]], stack: list[list[bool]]) -> bool:
        """Check whether the remaining pattern can finish the stack. The stack represents
        the currently unreflected rows, which need to be reflected to be successful."""
        if len(stack) == 0 or len(remaining) == 0:
            return True

        if remaining[0] != stack[-1]:
            return False

        return rec(remaining[1:], stack[:-1])

    # Try to start the mirror at all possible indices and test.
    for i in range(len(pattern) - 1):
        hypothetical_result = i + 1
        if hypothetical_result != disallowed:
            if rec(pattern[i + 1 :], pattern[: i + 1]):
                return hypothetical_result

    return None


def find_vertical_mirror(
    pattern: list[list[bool]], disallowed: int | None
) -> int | None:
    """Find a vertical mirror in a pattern and return the number of columns left to the line."""
    rotated_clockwise: list[list[bool]] = list(zip(*pattern[::-1]))  # type: ignore
    return find_horizontal_mirror(rotated_clockwise, disallowed)


def find_mirror(
    pattern: list[list[bool]], disallowed: Tuple[int, bool] | None
) -> Tuple[int, bool] | None:
    """Find a horizontal or vertical mirror and return the number of rows/cols
    that come before it. Also return whether the mirror is horizontal or not (vertical).
    """
    horizontal_disallowed = (
        disallowed[0] if disallowed is not None and disallowed[1] else None
    )
    vertical_disallowed = (
        disallowed[0] if disallowed is not None and not disallowed[1] else None
    )

    horizontal = find_horizontal_mirror(pattern, horizontal_disallowed)
    if horizontal is not None:
        return horizontal, True

    vertical = find_vertical_mirror(pattern, vertical_disallowed)
    if vertical is not None:
        return vertical, False

    return None


def find_mirror_with_flipped(pattern: list[list[bool]]) -> Tuple[int, bool] | None:
    """Find a different mirror with exactly one cell flipped."""
    original = find_mirror(pattern, None)
    assert original is not None
    for y in range(len(pattern)):
        for x in range(len(pattern[0])):
            flipped = find_mirror(
                [
                    [not val if y == i and x == j else val for j, val in enumerate(row)]
                    for i, row in enumerate(pattern)
                ],
                original,
            )
            if flipped is not None:
                return flipped
    return None


def main() -> None:
    """Day 13 part 2"""
    with open("input.txt", "r", encoding="utf-8") as file:
        lines = file.readlines()

        # True represents rock, false represents ash
        patterns: list[list[list[bool]]] = [[]]

        # Parse the input
        pattern_index = 0
        for line in lines:
            line = line.strip()
            if line == "":
                pattern_index += 1
                patterns.append([])
            else:
                patterns[pattern_index].append([s == "#" for s in line])

        # Find mirrors and add to result
        result = 0
        for pattern in patterns:
            mirror = find_mirror_with_flipped(pattern)
            assert mirror is not None
            if mirror[1]:
                result += mirror[0] * 100
            else:
                result += mirror[0]

        print(result)


if __name__ == "__main__":
    main()
