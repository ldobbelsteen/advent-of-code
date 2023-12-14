export async function main() {
  const input = await Deno.readTextFile("input.txt");
  const rows = input.split("\n").map((line) => line.split(""));

  // Move a rounded rock upwards until hitting the border, a cube rock
  // or another rounded rock. Return the new y-index of the rock.
  const moveRoundedUp = (x: number, y: number): number => {
    let newY = y;
    while (newY > 0 && rows[newY - 1][x] === ".") {
      newY -= 1;
    }
    rows[y][x] = ".";
    rows[newY][x] = "O";
    return newY;
  };

  let result = 0;
  for (let x = 0; x < rows[0].length; x++) {
    for (let y = 0; y < rows.length; y++) {
      if (rows[y][x] == "O") {
        const newY = moveRoundedUp(x, y);
        result += rows.length - newY;
      }
    }
  }

  console.log(result);
}

if (import.meta.main) {
  main();
}
