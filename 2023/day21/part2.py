import copy


def print_grid(grid: list[list[str]], mark: set[tuple[int, int]]):
    grid = copy.deepcopy(grid)
    for reach in mark:
        grid[reach[1]][reach[0]] = "O"
    print("----------------")
    print("\n".join(["".join(row) for row in grid]))


def main():
    total_steps: int = 1000
    total_steps_is_even = total_steps % 2 == 0
    with open("example.txt", "r") as file:
        grid = [list(line.strip()) for line in file.readlines()]
        size_x = len(grid[0])
        size_y = len(grid)

        step = 0
        reachable = 1  # starting point
        border: set[tuple[int, int]] = set()
        prev_border: set[tuple[int, int]] = set()

        for y in range(size_y):
            for x in range(size_x):
                if grid[y][x] == "S":
                    border.add((x, y))

        while step < total_steps:
            new_border: set[tuple[int, int]] = set()
            for tile in border:
                for neighbour in [
                    (tile[0] + 1, tile[1]),
                    (tile[0] - 1, tile[1]),
                    (tile[0], tile[1] + 1),
                    (tile[0], tile[1] - 1),
                ]:
                    if (
                        neighbour not in new_border
                        and neighbour not in border
                        and neighbour not in prev_border
                        and grid[neighbour[1] % size_y][neighbour[0] % size_x] != "#"
                    ):
                        new_border.add(neighbour)
                        if step % 2 == total_steps_is_even:
                            reachable += 1
            prev_border = border
            border = new_border
            step += 1

        print(reachable)


if __name__ == "__main__":
    main()
