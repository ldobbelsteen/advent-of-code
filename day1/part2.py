with open("input.txt", "r") as file:
    words = {
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }

    result = 0

    for line in file:
        line = line.rstrip()

        first_digit_occ_index = None
        first_digit_occ_value = None

        last_digit_occ_index = None
        last_digit_occ_value = None

        for word, value in words.items():
            first_word_occ_index = line.find(word)
            if first_word_occ_index >= 0 and (
                first_digit_occ_index is None
                or first_word_occ_index < first_digit_occ_index
            ):
                first_digit_occ_index = first_word_occ_index
                first_digit_occ_value = value

            last_word_occ_index = line.rfind(word)
            if last_word_occ_index >= 0 and (
                last_digit_occ_index is None
                or last_word_occ_index > last_digit_occ_index
            ):
                last_digit_occ_index = last_word_occ_index
                last_digit_occ_value = value

        result += int(str(first_digit_occ_value) + str(last_digit_occ_value))

    print(result)
