import z3
from typing import TypeAlias

Coord: TypeAlias = tuple[int, int, int]
Hailstone: TypeAlias = tuple[
    Coord, tuple[int, int, int]
]  # start, (x_velocity, y_velocity, z_velocity)


def main():
    with open("input.txt", "r") as file:
        hailstones: list[Hailstone] = []
        for line in file.readlines():
            left, right = line.strip().split(" @ ")
            left = left.replace(",", " ").split()
            right = right.replace(",", " ").split()
            hailstones.append(
                (
                    (int(left[0]), int(left[1]), int(left[2])),
                    (int(right[0]), int(right[1]), int(right[2])),
                )
            )

        solver = z3.Solver()

        rock_x_start, rock_y_start, rock_z_start = (
            z3.Int("rock_x_start"),
            z3.Int("rock_y_start"),
            z3.Int("rock_z_start"),
        )
        rock_x_velocity, rock_y_velocity, rock_z_velocity = (
            z3.Int("rock_x_velocity"),
            z3.Int("rock_y_velocity"),
            z3.Int("rock_z_velocity"),
        )
        collision_times = [
            z3.Int(f"collision_time_{i}") for i in range(len(hailstones))
        ]

        for i, h in enumerate(hailstones):
            solver.add(
                rock_x_start + collision_times[i] * rock_x_velocity
                == h[0][0] + collision_times[i] * h[1][0]
            )
            solver.add(
                rock_y_start + collision_times[i] * rock_y_velocity
                == h[0][1] + collision_times[i] * h[1][1]
            )
            solver.add(
                rock_z_start + collision_times[i] * rock_z_velocity
                == h[0][2] + collision_times[i] * h[1][2]
            )

        solver.check()
        m = solver.model()
        result = (
            m[rock_x_start].as_long()
            + m[rock_y_start].as_long()
            + m[rock_z_start].as_long()
        )
        print(result)


if __name__ == "__main__":
    main()
