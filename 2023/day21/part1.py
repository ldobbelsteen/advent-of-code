def main():
    total_steps = 64
    with open("input.txt", "r") as file:
        grid = [list(line.strip()) for line in file.readlines()]
        reachable: set[tuple[int, int]] = set()
        step = 0

        # Find starting position and set as reachable
        for y in range(len(grid)):
            for x in range(len(grid[0])):
                if grid[y][x] == "S":
                    reachable.add((x, y))

        while step < total_steps:
            new_reachable: set[tuple[int, int]] = set()
            for reach in reachable:
                for neighbour in [
                    (reach[0] + 1, reach[1]),
                    (reach[0] - 1, reach[1]),
                    (reach[0], reach[1] + 1),
                    (reach[0], reach[1] - 1),
                ]:
                    if (
                        neighbour[0] > 0
                        and neighbour[0] < len(grid[0])
                        and neighbour[1] > 0
                        and neighbour[1] < len(grid)
                        and grid[neighbour[1]][neighbour[0]] != "#"
                    ):
                        new_reachable.add(neighbour)
            reachable = new_reachable
            step += 1

        print(len(reachable))


if __name__ == "__main__":
    main()
