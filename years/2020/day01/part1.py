def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()

    for a in input:
        for b in input:
            if int(a) + int(b) == 2020:
                print(int(a)*int(b))
                return

main()
