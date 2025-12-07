import sys


def main():
    input = sys.stdin.read()
    match sys.argv[1]:
        case "1":
            part_one(input)
        case "2":
            part_two(input)
    sys.exit(0)


def part_one(input: str):
    banks = input.splitlines()
    jolts = []
    for bank in banks:
        # find the largest joltage and its position
        max_joltage = -1
        max_joltage_i = 0
        for i, joltage in enumerate(bank):
            if int(joltage) > max_joltage:
                max_joltage = int(joltage)
                max_joltage_i = i

        submax_joltage = -1
        if max_joltage_i == len(bank) - 1:
            for i, joltage in enumerate(bank[:-1]):
                if int(joltage) > submax_joltage:
                    submax_joltage = int(joltage)
            jolts.append(submax_joltage * 10 + max_joltage)
        else:
            for i, joltage in enumerate(bank[max_joltage_i + 1 :]):
                if int(joltage) > submax_joltage:
                    submax_joltage = int(joltage)
            jolts.append(max_joltage * 10 + submax_joltage)
    print(sum(jolts))


def part_two(input: str): ...


if __name__ == "__main__":
    main()
