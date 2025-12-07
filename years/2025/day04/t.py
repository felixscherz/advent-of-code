from __future__ import annotations
from io import StringIO
from typing import Generic, Iterator, TypeVar
import sys


def main():
    input = sys.stdin.read()
    match sys.argv[1]:
        case "1":
            part_one(input)
        case "2":
            part_two(input)
    sys.exit(0)

T = TypeVar("T")

def index_to_coordinate(i: int, height: int, width: int) -> tuple[int, int]:
    row = i // width
    col = i % width
    return (row, col)

def coordinate_to_index(coordinate: tuple[int, int], height: int, width: int) -> int:
    return coordinate[0] * width + coordinate[1]

class GridIterator(Generic[T]):
    def __init__(self, grid: Grid) -> None:
        self.next_pos = (0,0)
        self.grid = grid

    def __iter__(self) -> Iterator:
        return self

    def __next__(self) -> T:
        if self.next_pos[0] >= self.grid.height:
            raise StopIteration()
        t = self.grid[self.next_pos]
        next_row = self.next_pos[0]
        next_col = self.next_pos[1] + 1
        if next_col >= self.grid.width:
            next_row = next_row + 1
            next_col = 0
        self.next_pos = (next_row, next_col)
        return t

class Grid(Generic[T]):
    def __init__(self, rows: list[list[T]]) -> None:
        if not rows:
            raise ValueError("Cannot create a grid from an empty list.")
        if not len(set((len(row) for row in rows))) == 1:
            raise ValueError("All rows must be of the same length.")
        self.rows = rows
        self.height = len(rows)
        self.width = len(rows[0])

    def __getitem__(self, key: tuple[int, int]) -> T:
        return self.rows[key[0]][key[1]]

    def __setitem__(self, key: tuple[int,int], value: T) -> None:
        self.rows[key[0]][key[1]] = value

    def __iter__(self) -> Iterator[T]:
        return GridIterator[T](self)

    def adjacents(self, pos: tuple[int, int]) -> list[tuple[int, int]]:
        steps = (
            (-1, 0),  # up
            (1, 0),  # down
            (0, -1),  # left
            (0, 1),  # right
            (1, 1),  # southeast
            (-1, 1),  # northeast
            (1, -1),  # soutwest
            (-1, -1),  # northwest
        )

        positions = [ ]
        for step in steps:
            new_pos = (pos[0] + step[0], pos[1] + step[1])
            if new_pos[0] >= 0 and new_pos[0] < self.height and new_pos[1] >= 0 and new_pos[1] < self.width:
                positions.append(new_pos)
        return positions


def part_one(input: str):
    rows = [list(line) for line in input.splitlines()]
    map = Grid[str](rows)
    total = 0
    b = StringIO()
    for i, x in enumerate(map):
        if x == ".":
            b.write(".")
            continue
        pos = index_to_coordinate(i, map.height, map.width)
        adjacents = map.adjacents(pos)
        count = sum([1 for adj in adjacents if map[adj] == "@"])
        if count < 4:
            total += 1
    print(total)


def part_two(input: str):
    rows = [list(line) for line in input.splitlines()]
    map = Grid[str](rows)
    total = 0

    while True:
        changed = False
        for i, x in enumerate(map):
            if x == ".":
                continue
            pos = index_to_coordinate(i, map.height, map.width)
            adjacents = map.adjacents(pos)
            count = sum([1 for adj in adjacents if map[adj] == "@"])
            if count < 4:
                total += 1
                map[pos] = "."
                changed = True
        if not changed:
            break
    print(total)


if __name__ == "__main__":
    main()
