from typing import TypeAlias
from collections import deque

Coord: TypeAlias = tuple[int, int]


def main():
    steps: int = 26501365
    assert steps % 2 == 1

    with open("input.txt", "r") as file:
        grid = [list(line.strip()) for line in file.readlines()]
        start = (len(grid) // 2, len(grid) // 2)
        assert len(grid[0]) == len(grid)  # grid is square
        assert grid[start[1]][start[0]] == "S"  # starting point is in the middle

        distances: dict[Coord, int] = {}
        border: deque[tuple[int, Coord]] = deque([(0, start)])
        while len(border) > 0:
            distance, coord = border.popleft()
            if coord in distances:
                continue
            distances[coord] = distance
            for neighbour in [
                (coord[0] + 1, coord[1]),
                (coord[0] - 1, coord[1]),
                (coord[0], coord[1] + 1),
                (coord[0], coord[1] - 1),
            ]:
                if (
                    neighbour[0] >= 0
                    and neighbour[0] < len(grid)
                    and neighbour[1] >= 0
                    and neighbour[1] < len(grid)
                ):
                    if (
                        neighbour not in distances
                        and grid[neighbour[1]][neighbour[0]] != "#"
                    ):
                        border.append((distance + 1, neighbour))

        assert steps == 131 * 202300 + 65
        assert len(grid) == 2 * 65 + 1

        grids = 202300
        even_grid_count = pow(grids, 2)
        odd_grid_count = pow(grids + 1, 2)
        negative_odd_corners = grids + 1
        positive_even_corners = grids

        even = sum(map(lambda d: d % 2 == 0, distances.values()))
        odd = sum(map(lambda d: d % 2 == 1, distances.values()))
        even_corner = sum(map(lambda d: d % 2 == 0 and d > 65, distances.values()))
        odd_corner = sum(map(lambda d: d % 2 == 1 and d > 65, distances.values()))

        result = (
            even_grid_count * even
            + odd_grid_count * odd
            - negative_odd_corners * odd_corner
            + positive_even_corners * even_corner
        )

        print(result)


if __name__ == "__main__":
    main()
