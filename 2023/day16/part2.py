def move(coord: tuple[int, int], direction: str) -> tuple[int, int]:
    match direction:
        case "L":
            return (coord[0] - 1, coord[1])
        case "R":
            return (coord[0] + 1, coord[1])
        case "D":
            return (coord[0], coord[1] + 1)
        case "U":
            return (coord[0], coord[1] - 1)
        case _:
            raise Exception("invalid direction")


def trace(
    tiles: list[list[str]], start_point: tuple[int, int], start_direction: str
) -> int:
    energized: list[list[set[str]]] = []
    for y in range(len(tiles)):
        row: list[set[str]] = []
        for _ in range(len(tiles[y])):
            row.append(set())
        energized.append(row)

    queue: list[tuple[tuple[int, int], str]] = [(start_point, start_direction)]
    while len(queue) > 0:
        coord, direction = queue.pop()

        if (
            coord[0] < 0
            or coord[0] >= len(energized[0])
            or coord[1] < 0
            or coord[1] >= len(energized)
            or direction in energized[coord[1]][coord[0]]
        ):
            continue

        energized[coord[1]][coord[0]].add(direction)

        match tiles[coord[1]][coord[0]]:
            case ".":
                queue.append((move(coord, direction), direction))
            case "|":
                if direction == "L" or direction == "R":
                    queue.append((move(coord, "U"), "U"))
                    queue.append((move(coord, "D"), "D"))
                else:
                    queue.append((move(coord, direction), direction))
            case "-":
                if direction == "U" or direction == "D":
                    queue.append((move(coord, "L"), "L"))
                    queue.append((move(coord, "R"), "R"))
                else:
                    queue.append((move(coord, direction), direction))
            case "/":
                match direction:
                    case "L":
                        queue.append((move(coord, "D"), "D"))
                    case "R":
                        queue.append((move(coord, "U"), "U"))
                    case "D":
                        queue.append((move(coord, "L"), "L"))
                    case "U":
                        queue.append((move(coord, "R"), "R"))
                    case _:
                        raise Exception("invalid direction")
            case "\\":
                match direction:
                    case "L":
                        queue.append((move(coord, "U"), "U"))
                    case "R":
                        queue.append((move(coord, "D"), "D"))
                    case "D":
                        queue.append((move(coord, "R"), "R"))
                    case "U":
                        queue.append((move(coord, "L"), "L"))
                    case _:
                        raise Exception("invalid direction")
            case _:
                raise Exception("invalid tile")

    result = 0
    for row in energized:
        for directions in row:
            if len(directions) > 0:
                result += 1
    return result


def main():
    with open("input.txt", "r") as file:
        tiles = [list(line.strip()) for line in file.readlines()]

        result = 0
        for x in range(len(tiles[0])):
            subresult = trace(tiles, (x, 0), "D")
            if subresult > result:
                result = subresult
        for x in range(len(tiles[0])):
            subresult = trace(tiles, (x, len(tiles) - 1), "U")
            if subresult > result:
                result = subresult
        for y in range(len(tiles)):
            subresult = trace(tiles, (0, y), "R")
            if subresult > result:
                result = subresult
        for y in range(len(tiles)):
            subresult = trace(tiles, (len(tiles[0]) - 1, y), "L")
            if subresult > result:
                result = subresult
        print(result)


if __name__ == "__main__":
    main()
