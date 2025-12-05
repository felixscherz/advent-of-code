import sys


def main():
    input = sys.stdin.read()
    match sys.argv[1]:
        case "1":
            part_one(input)
        case "2":
            part_two(input)

    

def valid(id: str) -> bool:
    if id.startswith('0'):
        return False
    if id[:len(id) // 2] == id[len(id) // 2:]:
        return False
    return True

def part_one(input: str) -> None:
    ranges = input.split(",")
    invalid_ids_total = 0
    for range_ in ranges:
        lower,upper = range_.split("-")
        
        for id in range(int(lower), int(upper)+1):
            if not valid(str(id)):
                invalid_ids_total += id
    print(invalid_ids_total)

def valid_two(id: str) -> bool:
    for n in reversed(range(1, len(id))):
        x = len(id) / n
        if not x.is_integer():
            continue
        # id can be divided into x chunks of length n

        if int(x)*id[:n] == id:
            return False
    return True


def part_two(input: str) -> None:
    ranges = input.split(",")
    invalid_ids_total = 0
    for range_ in ranges:
        lower,upper = range_.split("-")
        
        for id in range(int(lower), int(upper)+1):
            if not valid_two(str(id)):
                invalid_ids_total += id
    print(invalid_ids_total)



if __name__ == "__main__":
    main()
