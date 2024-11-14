def main():
    with open("input.txt") as handle:
        input = handle.read().splitlines()

    required = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

    lines = iter(input)


    valid = 0
    end = False
    while not end:
        # beginning of passport
            passport = next(lines)
            line = next(lines)
            while line:
                passport = passport + " " + line
                try:
                    line = next(lines)
                except StopIteration:
                    end = True
                    break

            fields = dict(keyval.split(":") for keyval in passport.split(" "))

            if set(fields.keys()).issuperset(required):
                valid += 1
    print(valid)

main()
