from collections import Counter
def valid_password(line: str):
    min_max, letter, password = line.split(" ")
    min_s, max_s = min_max.split("-")
    min = int(min_s)
    max = int(max_s)
    letter = letter.replace(":", "")
    counter = Counter(password)
    return min <= counter[letter] and counter[letter] <= max



def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()


    c = 0
    for line in input:
        if valid_password(line):
            c += 1
    print(c)


main()

