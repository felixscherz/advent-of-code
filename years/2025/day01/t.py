import argparse
import sys
from typing import Literal

class Dial:
    def __init__(self, position: int, numbers: int):
        self.position = position
        self.numbers = numbers
        self._set_to_0 = 0
        self._clicks_to_0 = 0
        ...

    def rotate(self, amount: int) -> None:
        # positive: right
        # negative: left
        change = 1 if amount > 0 else -1
        for _ in range(abs(amount)):
            self.position = (self.position + change) % self.numbers
            if self.position == 0:
                self._clicks_to_0 += 1

        if self.position == 0:
            self._set_to_0 += 1


def main():
    parser = argparse.ArgumentParser()

    input = sys.stdin.read()
    dial = Dial(50, 100)
    for instr in input.splitlines():
        direction, amount = instr[:1], instr[1:]
        match direction:
            case 'R':
                dial.rotate(int(amount))
            case 'L':
                dial.rotate(-1*int(amount))
    print(dial._set_to_0)
    print(dial._clicks_to_0)


if __name__ == "__main__":
    main()
