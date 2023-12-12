def main() -> None:
    """Day 6 part 1"""
    with open("input.txt", "r", encoding="utf-8") as file:
        times_raw, distances_raw = file
        times = [int(t) for t in times_raw.replace("Time:", "").split()]
        distances = [int(d) for d in distances_raw.replace("Distance:", "").split()]

        result = 1

        for i, _ in enumerate(times):
            possibilities = 0
            for button_time in range(times[i] + 1):
                remaining_time = times[i] - button_time
                distance = button_time * remaining_time
                if distance > distances[i]:
                    possibilities += 1
            result *= possibilities

        print(result)


if __name__ == "__main__":
    main()
