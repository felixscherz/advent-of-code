def find_row_col(seq: str) -> tuple[int, int]:
    rows = [row for row in range(128)]
    cols = [col for col in range(8)]

    for part in seq:
        match part:
            case "F":
                rows = rows[: len(rows) // 2]
            case "B":
                rows = rows[len(rows) // 2 :]
            case "L":
                cols = cols[: len(cols) // 2]
            case "R":
                cols = cols[len(cols) // 2 :]

    assert len(rows) == 1
    assert len(cols) == 1
    return rows.pop(), cols.pop()


def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()

    seats = iter(input)

    seat = next(seats)
    ids = []
    row, col = find_row_col(seat)
    seat_id = row * 8 + col
    ids.append(seat_id)

    for seat in seats:
        row, col = find_row_col(seat)
        seat_id = row * 8 + col
        ids.append(seat_id)

    sorted_ids = sorted(ids)

    for i, id in enumerate(sorted_ids):
        if sorted_ids[i+1] != id+1:
            print(id+1)
            return

main()
