import argparse
import os
import sys
import re
from pathlib import Path

import requests

BASE_URL = "https://adventofcode.com"
AOC_COOKIE_ENV_NAME = "AOC_COOKIE"


def get_input(year: int | None = None, day: int | None = None, session: str | None = None) -> list[str]:
    if not year or not day:
        year, day = determine_year_day()

    cache_key = "input.txt"
    try:
        with open(cache_key) as handle:
            return handle.read().splitlines()
    except FileNotFoundError:
        pass

    if not session:
        session = os.environ[AOC_COOKIE_ENV_NAME]
    response = requests.get(f"{BASE_URL}/{year}/day/{day}/input", headers={"Cookie": session})
    assert response.ok

    with open(cache_key, "w") as handle:
        handle.write(response.text)

    return response.text.splitlines()


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

def submit(year: int, day: int, part: int, answer: int, session: str | None = None) -> bool:
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
            print(error_match[0])
            return False
    
    if RIGHT in response.text:
        return True
    else:
        print(response.text)
        return False


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(title="aoc", dest="command", required=True)
    subparsers.add_parser("load")
    submit_parser = subparsers.add_parser("submit")
    submit_parser.add_argument("--part", type=int, required=True)

    return parser.parse_args()


def main():
    options = parse_args()
    match options.command:
        case "load":
            get_input()
        case "submit":
            year, day = determine_year_day()
            answer = int(sys.stdin.read())
            success = submit(year=year, day=day, part=options.part, answer=answer)
            if success:
                print("\033[32mAnswer was correct!\033[0m")
        case _:
            ...
    raise SystemExit(0)
