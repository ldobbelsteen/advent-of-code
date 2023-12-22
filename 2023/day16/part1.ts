enum Direction {
  Left,
  Right,
  Down,
  Up,
}

function move(coord: [number, number], d: Direction): [number, number] {
  switch (d) {
    case Direction.Left:
      return [coord[0] - 1, coord[1]];
    case Direction.Right:
      return [coord[0] + 1, coord[1]];
    case Direction.Down:
      return [coord[0], coord[1] + 1];
    case Direction.Up:
      return [coord[0], coord[1] - 1];
  }
}

export async function main() {
  const input = await Deno.readTextFile("input.txt");
  const tiles = input.split("\n").map((line) => line.split(""));

  const energized: Set<Direction>[][] = [];
  for (let y = 0; y < tiles.length; y++) {
    const row = [];
    for (let x = 0; x < tiles[y].length; x++) {
      row.push(new Set<Direction>());
    }
    energized.push(row);
  }

  // Recursively follow a beam's path and mark visited tiles as energized.
  function markEnergized(coord: [number, number], d: Direction) {
    if (
      coord[0] < 0 || coord[0] >= energized[0].length ||
      coord[1] < 0 || coord[1] >= energized.length
    ) {
      return;
    }

    // If already visited, stop in order to break cycles.
    if (energized[coord[1]][coord[0]].has(d)) {
      return;
    }

    // Add incoming direction.
    energized[coord[1]][coord[0]].add(d);

    switch (tiles[coord[1]][coord[0]]) {
      case ".": {
        // Continue in same direction.
        markEnergized(move(coord, d), d);
        break;
      }
      case "|": {
        if (d === Direction.Left || d === Direction.Right) {
          // Continue up and down.
          markEnergized(move(coord, Direction.Up), Direction.Up);
          markEnergized(move(coord, Direction.Down), Direction.Down);
        } else {
          // Continue in the same direction.
          markEnergized(move(coord, d), d);
        }
        break;
      }
      case "-": {
        if (d === Direction.Up || d === Direction.Down) {
          // Continue up and down.
          markEnergized(move(coord, Direction.Left), Direction.Left);
          markEnergized(move(coord, Direction.Right), Direction.Right);
        } else {
          // Continue in the same direction.
          markEnergized(move(coord, d), d);
        }
        break;
      }
      case "/": {
        switch (d) {
          case Direction.Left:
            markEnergized(move(coord, Direction.Down), Direction.Down);
            break;
          case Direction.Right:
            markEnergized(move(coord, Direction.Up), Direction.Up);
            break;
          case Direction.Down:
            markEnergized(move(coord, Direction.Left), Direction.Left);
            break;
          case Direction.Up:
            markEnergized(move(coord, Direction.Right), Direction.Right);
            break;
        }
        break;
      }
      case "\\": {
        switch (d) {
          case Direction.Left:
            markEnergized(move(coord, Direction.Up), Direction.Up);
            break;
          case Direction.Right:
            markEnergized(move(coord, Direction.Down), Direction.Down);
            break;
          case Direction.Down:
            markEnergized(move(coord, Direction.Right), Direction.Right);
            break;
          case Direction.Up:
            markEnergized(move(coord, Direction.Left), Direction.Left);
            break;
        }
        break;
      }
      default: {
        console.error("unknown tile: ", tiles[coord[1]][coord[0]]);
      }
    }
  }

  // Start from top left heading to the right.
  markEnergized([0, 0], Direction.Right);

  // Count number of energized tiles.
  let result = 0;
  for (let x = 0; x < tiles[0].length; x++) {
    for (let y = 0; y < tiles.length; y++) {
      if (energized[y][x].size > 0) {
        result += 1;
      }
    }
  }

  console.log(result);
}

if (import.meta.main) {
  main();
}
