import requests
import os
import inspect
from pathlib import Path

BASE_URL = "https://adventofcode.com"
AOC_COOKIE_ENV_NAME = "AOC_COOKIE"


def get_input(year: int, day: int, session: str | None = None) -> list[str]:

    # cache next to calling file
    caller_filename = inspect.stack()[1].filename
    caller_dir = Path(caller_filename).parent
    cache_key = caller_dir / f"input_{year}_{day}.txt"

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

def determine_input() -> list[str]:
    # organized into years/<year>/<day>/<parts>.py
    caller_filename = inspect.stack()[1].filename
    parents = iter(Path(caller_filename).parents)
    day_s = next(parents).stem
    year_s = next(parents).stem

    day = int(day_s[len("day"):])
    year = int(year_s)
    print(day, year)

    return get_input(year=year, day=day)

def submit(year: int, day: int, part: int, answer: str, session: str | None = None) -> bool:
    if not session:
        session = os.environ[AOC_COOKIE_ENV_NAME]
    response = requests.post(f"{BASE_URL}/{year}/day/{day}/answer", headers={"Cookie": session}, data={"level": part, "answer": answer})
    breakpoint()
    assert response.ok

