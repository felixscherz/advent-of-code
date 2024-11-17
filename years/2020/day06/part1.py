def main():

    with open("input.txt") as handle:
        input = handle.read().splitlines()

    count = 0

    lines = input

    answers = set()

    for line in lines:
        match line:
            case "": # end of a group
                count += len(answers)
                answers = set()
            case _:
                answers.update(iter(line))
    else:
        count += len(answers)
        answers = set()


    print(count)


main()
