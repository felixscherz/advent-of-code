from __future__ import annotations
import sys

class Range:
    def __init__(self, start: int, stop: int) -> None:
        self.start = start
        self.stop = stop

    def __contains__(self, value: int | Range) -> bool:
        match value:
            case int():
                return self.start <= value and self.stop >= value
            case Range():
                return self.start <= value.start and self.stop >= value.stop

    def __repr__(self) -> str:
        return f"Range(start={self.start}, stop={self.stop})"

    def __len__(self) -> int:
        return self.stop - self.start + 1

def part_one(input: str):
    ranges = []
    iterator = iter(input.splitlines())
    for line in iterator:
        if not line:
            break
        a, b = line.split("-")
        ranges.append(Range(int(a), int(b)))

    ids = []
    for line in iterator:
        ids.append(int(line))

    total = 0
    for id in ids:
        for range_ in ranges:
            if id in range_:
                total +=1
                break

    print(total)

def part_two(input: str):
    ranges = []
    iterator = iter(input.splitlines())
    for line in iterator:
        if not line:
            break
        a, b = line.split("-")
        ranges.append(Range(int(a), int(b)))

    ranges.sort(key=lambda x: (x.start, -(x.stop - x.start)))
    non_overlapping_ranges = []
    while True:
        if not ranges:
            break
        current_range = ranges.pop(0)
        next_to_check = []
        non_overlapping_ranges.append(current_range)
        for i, other in enumerate(ranges):
            if other in current_range:
                continue
            elif other.start <= current_range.stop:
                if other.stop >= current_range.stop:
                    current_range.stop = other.stop
            else:
                next_to_check.append(other)
        ranges = next_to_check

    print(sum((len(r) for r in non_overlapping_ranges)))

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
