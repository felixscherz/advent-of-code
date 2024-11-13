def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()

    x = 0
    y = 0

    x_step = 3
    y_step = 1

    trees = 0
    lines = iter(input)
    line = next(lines)
    while y <= len(input):
        if line[x] == "#":
            trees += 1

        x = (x + x_step) % len(line)
        y = y + y_step
        try:
            line = next(lines)
        except StopIteration:
            pass
    print(trees)

main()




