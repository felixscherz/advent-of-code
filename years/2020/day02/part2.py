from collections import Counter
def valid_password(line: str):
    min_max, letter, password = line.split(" ")
    min_s, max_s = min_max.split("-")
    a = int(min_s)
    b = int(max_s)
    letter = letter.replace(":", "")
    a_contains = password[a-1] == letter
    b_contains = password[b-1] == letter
    return a_contains + b_contains == 1



def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()


    c = 0
    for line in input:
        if valid_password(line):
            c += 1
    print(c)


main()


