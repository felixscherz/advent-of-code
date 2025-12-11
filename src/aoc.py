import argparse
import os
import re
import sys
from pathlib import Path
import textwrap
import webbrowser

import requests

BASE_URL = "https://adventofcode.com"
AOC_COOKIE_ENV_NAME = "AOC_COOKIE"
INPUT_FILENAME = "input.txt"


def get_input(year: int | None = None, day: int | None = None, session: str | None = None) -> int:
    if not year or not day:
        year, day = determine_year_day()

    try:
        with open(INPUT_FILENAME) as handle:
            return 0
    except FileNotFoundError:
        pass

    if not session:
        session = os.environ[AOC_COOKIE_ENV_NAME]
    response = requests.get(f"{BASE_URL}/{year}/day/{day}/input", headers={"Cookie": session})
    assert response.ok

    with open(INPUT_FILENAME, "w") as handle:
        handle.write(response.text)

    return 0


def determine_year_day() -> tuple[int, int]:
    cwd = Path.cwd()
    day_s = cwd.stem
    if not day_s.startswith("day"):
        raise AssertionError(f"Unexpected working directory: {cwd}")
    year_s = cwd.parent.stem
    year = int(year_s)
    day = int(day_s[len("day") :])
    return year, day


TOO_QUICK = re.compile('You gave an answer too recently.*to wait.')
WRONG = re.compile(r"That's not the right answer.*?\.")
RIGHT = "That's the right answer!"
ALREADY_DONE = re.compile(r"You don't seem to be solving.*\?")

def submit(year: int, day: int, part: int, answer: int, session: str | None = None) -> int:
    if not year or not day:
        year, day = determine_year_day()

    if not session:
        session = os.environ[AOC_COOKIE_ENV_NAME]
    response = requests.post(
        f"{BASE_URL}/{year}/day/{day}/answer", headers={"Cookie": session}, data={"level": part, "answer": answer}
    )
    for error_regex in (WRONG, TOO_QUICK, ALREADY_DONE):
        error_match = error_regex.search(response.text)
        if error_match:
            print(f"\033[31m{error_match[0]}\033[0m")
            return 1
    
    if RIGHT in response.text:
        print("\033[32mAnswer was correct!\033[0m")
        return 0
    else:
        print(response.text)
        return 1

def init():
    content = textwrap.dedent("""
    from __future__ import annotations
    import sys

    def part_one(input: str):
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
    """)
    print(content)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(title="aoc", dest="command", required=True)
    subparsers.add_parser("load")
    submit_parser = subparsers.add_parser("submit")
    submit_parser.add_argument("--part", type=int, required=True)
    subparsers.add_parser("open")
    subparsers.add_parser("init")

    return parser.parse_args()


def main():
    options = parse_args()
    match options.command:
        case "load":
            raise SystemExit(get_input())
        case "submit":
            year, day = determine_year_day()
            answer = int(sys.stdin.read())
            success = submit(year=year, day=day, part=options.part, answer=answer)
            raise SystemExit(success)
        case "open":
            year, day = determine_year_day()
            webbrowser.open(f"https://adventofcode.com/{year}/day/{day}")
        case "init":
            init()
        case _:
            ...
