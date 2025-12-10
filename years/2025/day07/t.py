from __future__ import annotations
import sys
import functools
from typing import NamedTuple


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


class Node(NamedTuple):
    children: list[Node]

# walk through a maze
# if encountering a leaf node, backtrack
# graph is a DAG so there are no cycles

paths = 0
def visit(node: Node) -> None:
    global paths
    if not node.children:
        # found leaf node
        paths += 1
        return
    for child in node.children:
        visit(child)
    

def part_two(input: str):
    rows = input.splitlines()
    splits = 0
    current_row = list(rows[0])
    next_rows = []
    next_rows.append(current_row)
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

        next_rows.append(next_row)
        current_row = next_row

    paths = 0
    height = len(next_rows)
    width = len(next_rows[0])

    class Position(NamedTuple):
        i: int
        j: int

    def traverse(pos: Position) -> None:
        nonlocal paths
        if pos.i == height - 1:
            paths+= 1
            return
        match next_rows[pos.i+1][pos.j]:
            case "|":
                traverse(Position(pos.i+1, pos.j))
            case "^":
                traverse(Position(pos.i+1, pos.j-1))
                traverse(Position(pos.i+1, pos.j+1))

    @functools.cache
    def number_of_paths(pos: Position) -> int:
        if pos.i == height - 1:
            return 1
        match next_rows[pos.i + 1][pos.j]:
            case "|":
                return number_of_paths(Position(pos.i + 1, pos.j))
            case "^":
                return number_of_paths(
                    Position(pos.i + 1, pos.j - 1)
                ) + number_of_paths(Position(pos.i + 1, pos.j + 1))
            case _:
                raise RuntimeError()

    for j, c in enumerate(next_rows[0]):
        if c == "S":
            paths = number_of_paths(Position(0, j))
            break
    print(paths)

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
