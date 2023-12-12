interface Coord {
  x: number;
  y: number;
}

function getAdjacent(
  coord: Coord,
  width: number,
  height: number,
) {
  return [
    { x: coord.x - 1, y: coord.y - 1 },
    { x: coord.x, y: coord.y - 1 },
    { x: coord.x + 1, y: coord.y - 1 },
    { x: coord.x - 1, y: coord.y },
    { x: coord.x + 1, y: coord.y },
    { x: coord.x - 1, y: coord.y + 1 },
    { x: coord.x, y: coord.y + 1 },
    { x: coord.x + 1, y: coord.y + 1 },
  ].filter((c) => c.x >= 0 && c.x < width && c.y >= 0 && c.y < height);
}

function getPerimeter(
  coords: Coord[],
  width: number,
  height: number,
) {
  const result: Coord[] = [];
  for (const coord of coords) {
    for (const adj of getAdjacent(coord, width, height)) {
      if (
        !coords.some((c) => c.x === adj.x && c.y === adj.y) &&
        !result.some((c) => c.x === adj.x && c.y === adj.y)
      ) {
        result.push(adj);
      }
    }
  }
  return result;
}

function isDigit(char: string) {
  const digit = parseInt(char);
  return !isNaN(digit) && digit >= 0 && digit <= 9;
}

export async function main() {
  const file = await Deno.readTextFile("input.txt");
  const grid = file.split("\n").map((line) => line.split(""));
  const width = grid[0].length;
  const height = grid.length;

  interface Ratio {
    value: number;
    count: number;
  }

  const gears: { [x: number]: { [y: number]: Ratio } } = {};

  const updateAdjacentGears = (coords: Coord[], value: number) => {
    const perimeter = getPerimeter(coords, width, height);
    for (const coord of perimeter) {
      if (grid[coord.y][coord.x] === "*") {
        if (!gears[coord.x]) {
          gears[coord.x] = {};
        }
        if (!gears[coord.x][coord.y]) {
          gears[coord.x][coord.y] = {
            value: 1,
            count: 0,
          };
        }
        const existing = gears[coord.x][coord.y];
        gears[coord.x][coord.y] = {
          value: existing.value * value,
          count: existing.count + 1,
        };
      }
    }
  };

  for (let y = 0; y < grid.length; y++) {
    const line = grid[y];

    let currentNumberStr = "";
    let currentCoords = [];

    for (let x = 0; x < line.length; x++) {
      const char = line[x];

      if (isDigit(char)) {
        currentNumberStr += char;
        currentCoords.push({ x, y });
      } else if (currentNumberStr.length > 0) {
        updateAdjacentGears(currentCoords, parseInt(currentNumberStr));
        currentNumberStr = "";
        currentCoords = [];
      }
    }

    if (currentNumberStr.length > 0) {
      updateAdjacentGears(currentCoords, parseInt(currentNumberStr));
    }
  }

  let sum = 0;
  for (const nested of Object.values(gears)) {
    for (const ratio of Object.values(nested)) {
      if (ratio.count >= 2) {
        sum += ratio.value;
      }
    }
  }

  console.log(sum);
}

if (import.meta.main) {
  main();
}
