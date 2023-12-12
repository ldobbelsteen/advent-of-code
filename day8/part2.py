from math import lcm


def compute_intervals(
    node: str, nodes: dict[str, tuple[str, str]], directions: list[bool]
):
    """
    Compute the intervals after which ..Z nodes appear when starting at some node with some
    directions. The presence of a cycle is assumed. The index to which is jumped back and the cost
    of that jump are returned alongside the intervals.
    """
    z_steps: list[int] = []
    cycle_jump_index = None
    cycle_jump_interval = None

    direction_index: int = 0

    visited_count = 1
    visited: dict[tuple[str, int], int] = {(node, direction_index): 1}
    last_visited = (node, direction_index)

    while True:
        direction = directions[direction_index]
        direction_index += 1
        if direction_index >= len(directions):
            direction_index = 0

        next_visit = (
            nodes[last_visited[0]][0] if direction else nodes[last_visited[0]][1],
            direction_index,
        )

        if next_visit in visited:
            repeated = visited[next_visit]
            cycle_jump_index, z_step = next(
                (i, s) for i, s in enumerate(z_steps) if s >= repeated
            )
            cycle_jump_interval = visited_count - z_steps[-1] + z_step - repeated + 1
            break
        else:
            visited_count += 1
            visited[next_visit] = visited_count
            if next_visit[0][-1] == "Z":
                z_steps.append(visited_count - 1)
            last_visited = next_visit

    intervals = [
        z_steps[i] - z_steps[i - 1] if i > 0 else z_steps[i]
        for i in range(len(z_steps))
    ]

    return intervals, cycle_jump_index, cycle_jump_interval


def main() -> None:
    """Day 8 part 2"""
    with open("input.txt", "r", encoding="utf-8") as file:
        lines = file.readlines()

        # Get the directions in a list of going either left or right
        directions = [
            d == "L" for d in lines.pop(0).strip()
        ]  # true is left, false is right

        # Get the nodes and put them in a dictionary
        start_nodes = []
        nodes: dict[str, tuple[str, str]] = {}
        lines.pop(0)
        for line in lines:
            node, tup = line.strip().split(" = ")
            left, right = tup.replace("(", "").replace(")", "").split(", ")
            nodes[node] = (left, right)
            if node[-1] == "A":
                start_nodes.append(node)

        # Keep track of the the number of steps and direction index for each starting node.
        steps_taken = [0 for _ in start_nodes]
        interval_index = [0 for _ in start_nodes]
        intervals = [compute_intervals(n, nodes, directions) for n in start_nodes]

        # If all starting nodes have only one repeating ..Z and cycles without a tail,
        # return LCM of periods.
        if all(
            [
                len(ints) == 1 and cycle_jump_index == 0
                for (ints, cycle_jump_index, _) in intervals
            ]
        ):
            periods = [
                sum(ints[cycle_jump_index + 1 :]) + cycle_jump_interval
                for (ints, cycle_jump_index, cycle_jump_interval) in intervals
            ]
            print(lcm(*periods))
        else:
            # Else, bruteforce. Takes the state with the minimum currently taken steps and advances
            # to the next ..Z until all the currently taken steps match. This case does not happen
            # for the given input.txt...
            while True:
                if interval_index[0] == len(intervals[0][0]):
                    steps_taken[0] += intervals[0][2]
                    interval_index[0] = intervals[0][1]
                else:
                    steps_taken[0] += intervals[0][0][interval_index[0]]
                    interval_index[0] += 1
                interval_index = [
                    ii for _, ii in sorted(zip(steps_taken, interval_index))
                ]  # sort by steps_taken
                intervals = [
                    ivs for _, ivs in sorted(zip(steps_taken, intervals))
                ]  # sort by steps_taken
                steps_taken.sort()
                if all(
                    [steps_taken[i] == steps_taken[0] for i in range(len(steps_taken))]
                ):
                    break
            print(steps_taken[0])


if __name__ == "__main__":
    main()
