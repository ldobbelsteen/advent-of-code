with open("input.txt", "r") as file:
    result = 0
    for line in file:
        line = line.rstrip()
        first = None
        last = None
        for char in line:
            if char.isdigit():
                if first is None:
                    first = char
                last = char
        result += int(first + last)
    print(result)
