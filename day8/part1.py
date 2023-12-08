with open("input.txt", "r") as file:
    lines = file.readlines()

    # Get the instruction string
    instructions = lines.pop(0).strip()

    # Get the nodes and put them in a dictionary
    nodes: dict[str, tuple[str, str]] = {}
    lines.pop(0)
    for line in lines:
        node, tup = line.strip().split(" = ")
        left, right = tup.replace("(", "").replace(")", "").split(", ")
        nodes[node] = (left, right)

    # Follow the path until ZZZ
    instruction_index = 0
    current_node = "AAA"
    steps_taken = 0
    while current_node != "ZZZ":
        instruction = instructions[instruction_index]
        if instruction == "L":
            current_node = nodes[current_node][0]
        else:
            assert instruction == "R"
            current_node = nodes[current_node][1]
        instruction_index += 1
        if instruction_index >= len(instructions):
            instruction_index = 0
        steps_taken += 1

    print(steps_taken)
