from math import lcm


def get_intervals(node: str, nodes: dict[str, tuple[str, str]], directions: list[bool]):
    """
    Compute the intervals after which ..Z nodes appear when starting at some node with some directions.
    The presence of a cycle is assumed. The index to which is jumped back and the cost of that jump are
    returned alongside the intervals.
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


with open("input.txt", "r") as file:
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
    # Structure: [[steps_taken, interval_index, (intervals, cycle_jump_index, cycle_jump_interval)]]
    states: list[tuple[int, int, tuple[list[int], int, int]]] = [
        [0, 0, get_intervals(n, nodes, directions)] for n in start_nodes
    ]

    # If all starting nodes have only one repeating ..Z and cycles without a tail, return LCM of periods.
    if all(
        [
            len(intervals) == 1 and cycle_jump_index == 0
            for _, _, (intervals, cycle_jump_index, _) in states
        ]
    ):
        periods = [
            sum(intervals[cycle_jump_index + 1 :]) + cycle_jump_interval
            for _, _, (intervals, cycle_jump_index, cycle_jump_interval) in states
        ]
        print(lcm(*periods))
    else:
        # Else, bruteforce. Takes the state with the minimum currently taken steps and advances to the next ..Z
        # until all the currently taken steps match. This case does not happen for the given input.txt...
        while True:
            if states[0][1] == len(states[0][2][0]):
                states[0][0] += states[0][2][2]
                states[0][1] = states[0][2][1]
            else:
                states[0][0] += states[0][2][0][states[0][1]]
                states[0][1] += 1
            states.sort(key=lambda s: s[0])
            if all([state[0] == states[0][0] for state in states]):
                break
        print(states[0][0])
