from __future__ import annotations
import sys


def part_one(input: str):
    rows = input.splitlines()
    splits = 0
    current_row = list(rows[0])
    for i in range(len(rows)-1):
        next_row = list(rows[i+1])
        for j, c in enumerate(current_row):
            match c:
                case "." | "^":
                    continue
                case "S":
                    next_row[j] = "|"
                case "|":
                    match next_row[j]:
                        case "^":
                            splits+=1
                            if j>0:
                                next_row[j-1] = "|"
                            if j<len(next_row)-1:
                                next_row[j+1] = "|"
                        case _:
                            next_row[j] = "|"

        current_row = next_row
    print(splits)



    ...

def part_two(input: str):
    ...

def main():
    input = sys.stdin.read()
    match sys.argv[1]:
        case "1":
            part_one(input)
        case "2":
            part_two(input)
    sys.exit(0)

if __name__ == "__main__":
    main()
