with open("1.txt", "r") as file:
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
        line_rev = line[::-1]

        first_digit_occ_index = None
        first_digit_occ_value = None

        last_digit_occ_index = None
        last_digit_occ_value = None

        for word, value in words.items():
            try:
                first_word_occ_index = line.index(word)
                if (
                    first_digit_occ_index is None
                    or first_word_occ_index < first_digit_occ_index
                ):
                    first_digit_occ_index = first_word_occ_index
                    first_digit_occ_value = value
            except ValueError:
                pass

            try:
                word_rev = word[::-1]
                last_word_occ_index = line_rev.index(word_rev)
                if (
                    last_digit_occ_index is None
                    or last_word_occ_index < last_digit_occ_index
                ):
                    last_digit_occ_index = last_word_occ_index
                    last_digit_occ_value = value
            except ValueError:
                pass

        result += int(str(first_digit_occ_value) + str(last_digit_occ_value))

    print(result)
