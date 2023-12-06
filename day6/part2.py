with open("input.txt", "r") as file:
    time_raw, distance_raw = file
    time = int(time_raw.replace("Time:", "").replace(" ", ""))
    record = int(distance_raw.replace("Distance:", "").replace(" ", ""))

    result = 0
    for button_time in range(time + 1):
        remaining_time = time - button_time
        distance = button_time * remaining_time
        if distance > record:
            result += 1

    print(result)
