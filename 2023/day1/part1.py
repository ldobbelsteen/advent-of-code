def main() -> None:
    """Day 1 part 1"""
    with open("input.txt", "r", encoding="utf-8") as file:
        result = 0
        for line in file:
            line = line.rstrip()
            first = None
            last = None
            for char in line:
                if char.isdigit():
                    if first is None:
                        first = char
                    last = char
            assert first is not None
            assert last is not None
            result += int(first + last)
        print(result)


if __name__ == "__main__":
    main()
