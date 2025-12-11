from __future__ import annotations
from functools import reduce, cache
from operator import mul
import sys
from typing import NamedTuple, Sequence

class Point(NamedTuple):
    x: int
    y: int
    z: int

@cache
def distance(a: Point, b: Point) -> float:
    return (a.x - b.x)**2 + (a.y-b.y)**2 + (a.z - b.z)**2


def find_closest_pair(points: Sequence[Point], exclude: set[tuple[Point, Point]]) -> tuple[Point, Point]:
    lowest_pair = None
    lowest_distance = None

    for i in range(len(points)):
        for j in range(len(points)):
            if i == j:
                continue
            if (points[i], points[j]) in exclude or (points[j], points[i]) in exclude:
                continue
            current_distance = distance(points[i], points[j])
            if not lowest_distance:
                lowest_distance = current_distance
                lowest_pair = (i,j)
                continue
            if current_distance < lowest_distance:
                lowest_pair = (i,j)
                lowest_distance = current_distance
    return points[lowest_pair[0]], points[lowest_pair[1]]






def part_one(input: str):
    points = []
    for row in input.splitlines():
        coords = [int(c) for c in row.split(",")]
        points.append(Point(*coords))

    circuit_mapping: dict[Point, set[Point]] = {}

    joined = set()
    for _ in range(1000):
        a, b = find_closest_pair(points, joined)
        joined.add((a,b))
        match circuit_mapping.get(a), circuit_mapping.get(b):
            case set() as circuit_of_a, set() as circuit_of_b:
                circuit_of_a.update(circuit_of_b)
                for point in circuit_of_b:
                    circuit_mapping[point] = circuit_of_a
            case set() as circuit_of_a, None:
                circuit_of_a.add(b)
                circuit_mapping[b] = circuit_of_a
            case None, set() as circuit_of_b:
                circuit_of_b.add(a)
                circuit_mapping[a] = circuit_of_b
            case None, None:
                circuit = {a, b}
                circuit_mapping[a] = circuit
                circuit_mapping[b] = circuit


    circuits = set(frozenset(s) for s in circuit_mapping.values())
    circuits = list(circuits)
    circuits.sort(key=lambda x: len(x), reverse=True)

    total = reduce(mul, [len(c) for c in circuits[:3]])
    print(total)

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

