def pattern_to_str(pattern: list[list[bool]]):
    """Print pattern back into the original format."""
    return "\n".join(
        ["".join(["#" if is_rock else "." for is_rock in row]) for row in pattern]
    )


def find_horizontal_mirror(pattern: list[list[bool]]) -> int | None:
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
        if rec(pattern[i + 1 :], pattern[: i + 1]):
            return i + 1

    return None


def find_vertical_mirror(pattern: list[list[bool]]) -> int | None:
    """Find a vertical mirror in a pattern and return the number of columns left to the line."""
    rotated_clockwise: list[list[bool]] = list(zip(*pattern[::-1]))  # type: ignore
    return find_horizontal_mirror(rotated_clockwise)


def main() -> None:
    """Day 13 part 1"""
    with open("example.txt", "r", encoding="utf-8") as file:
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
            horizontal = find_horizontal_mirror(pattern)
            if horizontal is not None:
                # print("Horizontal", horizontal, "\n", pattern_to_str(pattern))
                result += horizontal * 100
            vertical = find_vertical_mirror(pattern)
            if vertical is not None:
                # print("Vertical", vertical, "\n", pattern_to_str(pattern))
                result += vertical

        print(result)


if __name__ == "__main__":
    main()
