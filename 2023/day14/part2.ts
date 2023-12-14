type Puzzle = Cell[][];

enum Cell {
  CubeRock,
  RoundRock,
  Ash,
}

export async function main() {
  const input = await Deno.readTextFile("input.txt");
  const puzzle: Puzzle = input.split("\n").map((line) =>
    line.split("").map((c) => {
      if (c === "#") {
        return Cell.CubeRock;
      } else if (c === "O") {
        return Cell.RoundRock;
      } else {
        return Cell.Ash;
      }
    })
  );

  const hashPuzzle = () => JSON.stringify(puzzle);

  const computeNorthBeamLoad = () => {
    let result = 0;
    for (let x = 0; x < puzzle[0].length; x++) {
      for (let y = 0; y < puzzle.length; y++) {
        if (puzzle[y][x] === Cell.RoundRock) {
          result += puzzle.length - y;
        }
      }
    }
    return result;
  };

  const slideNorth = (x: number, y: number): number => {
    let newY = y;
    while (newY > 0 && puzzle[newY - 1][x] === Cell.Ash) {
      newY -= 1;
    }
    [puzzle[y][x], puzzle[newY][x]] = [puzzle[newY][x], puzzle[y][x]];
    return newY;
  };

  const slideSouth = (x: number, y: number): number => {
    let newY = y;
    while (newY < puzzle.length - 1 && puzzle[newY + 1][x] === Cell.Ash) {
      newY += 1;
    }
    [puzzle[y][x], puzzle[newY][x]] = [puzzle[newY][x], puzzle[y][x]];
    return newY;
  };

  const slideWest = (x: number, y: number) => {
    let newX = x;
    while (newX > 0 && puzzle[y][newX - 1] === Cell.Ash) {
      newX -= 1;
    }
    [puzzle[y][x], puzzle[y][newX]] = [puzzle[y][newX], puzzle[y][x]];
  };

  const slideEast = (x: number, y: number) => {
    let newX = x;
    while (newX < puzzle[0].length - 1 && puzzle[y][newX + 1] === Cell.Ash) {
      newX += 1;
    }
    [puzzle[y][x], puzzle[y][newX]] = [puzzle[y][newX], puzzle[y][x]];
  };

  const tiltNorth = (): number => {
    let northBeamLoadChange = 0;
    for (let x = 0; x < puzzle[0].length; x++) {
      for (let y = 0; y < puzzle.length; y++) {
        if (puzzle[y][x] == Cell.RoundRock) {
          const newY = slideNorth(x, y);
          northBeamLoadChange += y - newY;
        }
      }
    }
    return northBeamLoadChange;
  };

  const tiltSouth = (): number => {
    let northBeamLoadChange = 0;
    for (let x = 0; x < puzzle[0].length; x++) {
      for (let y = puzzle.length - 1; y >= 0; y--) {
        if (puzzle[y][x] == Cell.RoundRock) {
          const newY = slideSouth(x, y);
          northBeamLoadChange += y - newY;
        }
      }
    }
    return northBeamLoadChange;
  };

  const tiltWest = () => {
    for (let y = 0; y < puzzle.length; y++) {
      for (let x = 0; x < puzzle[0].length; x++) {
        if (puzzle[y][x] == Cell.RoundRock) {
          slideWest(x, y);
        }
      }
    }
  };

  const tiltEast = () => {
    for (let y = 0; y < puzzle.length; y++) {
      for (let x = puzzle[0].length - 1; x >= 0; x--) {
        if (puzzle[y][x] == Cell.RoundRock) {
          slideEast(x, y);
        }
      }
    }
  };

  const spinCycle = (): number => {
    let northBeamLoadChange = 0;
    northBeamLoadChange += tiltNorth();
    tiltWest();
    northBeamLoadChange += tiltSouth();
    tiltEast();
    return northBeamLoadChange;
  };

  let load = computeNorthBeamLoad();
  const loads = [load];
  const encountered: Record<string, number> = {
    [hashPuzzle()]: 0,
  };

  const total = 1000000000;
  for (let i = 1; i <= total; i++) {
    load += spinCycle();
    const hash = hashPuzzle();
    if (encountered[hash] === undefined) {
      encountered[hash] = i;
      loads.push(load);
    } else {
      const cycleSize = i - encountered[hash];
      const offset = (total - encountered[hash]) % cycleSize;
      const loadIndex = encountered[hash] + offset;
      load = loads[loadIndex];
      break;
    }
  }

  console.log(load);
}

if (import.meta.main) {
  main();
}
