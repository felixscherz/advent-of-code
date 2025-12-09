from __future__ import annotations
from operator import mul, add
from functools import reduce
import sys


def part_one(input: str):
    rows = [line.split() for line in input.splitlines()]
    
    number_of_problems = len(rows[0])
    operands = len(rows) - 1
    total = 0
    for i in range(number_of_problems):
        op = None
        match rows[operands][i]:
            case "*":
                op = mul
            case "+":
                op = add
            case _:
                raise RuntimeError()
        total += reduce(op, [int(row[i]) for row in rows if row[i] not in ("*", "+")])
    print(total)




        


    ...

def part_two(input: str):
    rows = [list(line) for line in input.splitlines()]
    columns = []
    for j in reversed(range(len(rows[0]))):
        column = []
        for i in range(len(rows)):
            column.append(rows[i][j])
        columns.append(column)


    total = 0
    numbers = []
    for col in columns:
        if col[-1] in ("*", "+"):
            numbers.append(int("".join(col[:-1])))
            op = None
            match col[-1]:
                case "*":
                    op = mul
                case "+":
                    op = add
                case _:
                    raise RuntimeError()
            total += reduce(op, numbers)
            numbers = []
            continue
        if not "".join(col).strip():
            continue
        numbers.append(int("".join(col)))

            
    print(total)

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
