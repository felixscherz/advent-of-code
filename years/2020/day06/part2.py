
def main():

    with open("input.txt") as handle:
        input = handle.read().splitlines()

    count = 0

    lines = input

    answers = None

    for line in lines:
        match line:
            case "": # end of a group
                count += len(answers)
                answers = None
            case _:
                if answers is None:
                    answers = set(iter(line))
                else:
                    answers = answers.intersection(set(iter(line)))
    else:
        count += len(answers)
        answers = set()


    print(count)


main()
