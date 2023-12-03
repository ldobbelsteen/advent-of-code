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

function isEnginePart(
  coords: Coord[],
  grid: string[][],
  width: number,
  height: number,
) {
  return getPerimeter(coords, width, height).some(
    (c) => (grid[c.y][c.x] !== "." && !isDigit(grid[c.y][c.x])),
  );
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

  let sum = 0;

  const checkEnginePart = (coords: Coord[], value: number) => {
    if (isEnginePart(coords, grid, width, height)) {
      sum += value;
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
        checkEnginePart(currentCoords, parseInt(currentNumberStr));
        currentNumberStr = "";
        currentCoords = [];
      }
    }

    if (currentNumberStr.length > 0) {
      checkEnginePart(currentCoords, parseInt(currentNumberStr));
    }
  }

  console.log(sum);
}

if (import.meta.main) {
  main();
}
