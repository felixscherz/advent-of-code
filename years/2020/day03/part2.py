from functools import reduce
import sys


def traverse(x_step, y_step, input) -> int:
    x = 0
    y = 0

    trees = 0
    while y < len(input):
        line = input[y]
        if line[x] == "#":
            trees += 1

        x = (x + x_step) % len(line)
        y += y_step
    print(x_step,y_step,trees, file=sys.stderr)
    return trees


def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()

    slopes = ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))

    answer = reduce(lambda a, b: a * b, [traverse(x_s, y_s, input) for x_s, y_s in slopes])
    print(answer)


main()
