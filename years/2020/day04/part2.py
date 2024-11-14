import re
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
        try:
            line = next(lines)
        except StopIteration:
            end = True
            line = ""

        while line:
            passport = passport + " " + line
            try:
                line = next(lines)
            except StopIteration:
                end = True
                break


        fields = dict(keyval.split(":") for keyval in passport.split(" "))

        if set(fields.keys()).issuperset(required):
            if not all(
                (
                    len(fields["byr"]) == 4,
                    int(fields["byr"]) >= 1920,
                    int(fields["byr"]) <= 2002,
                    len(fields["iyr"]) == 4,
                    int(fields["iyr"]) >= 2010,
                    int(fields["iyr"]) <= 2020,
                    len(fields["eyr"]) == 4,
                    int(fields["eyr"]) >= 2020,
                    int(fields["eyr"]) <= 2030,
                )
            ):
                continue

            hgt = fields["hgt"]
            if not hgt.endswith(("cm", "in")):
                continue
            if hgt.endswith("cm") and int(hgt[:-2]) < 150:
                continue
            if hgt.endswith("cm") and int(hgt[:-2]) > 193:
                continue
            if hgt.endswith("in") and int(hgt[:-2]) < 59:
                continue
            if hgt.endswith("in") and int(hgt[:-2]) > 76:
                continue

            hcl = fields["hcl"]
            hair = re.compile(r"^#[0-9a-f]{6}$")
            if not hair.search(hcl):
                continue
            if fields["ecl"] not in ("amb", "blu", "brn", "gry", "hzl", "oth", "grn"):
                continue


            valid_pid = re.compile(r"^[0-9]{9}$")
            if not valid_pid.search(fields["pid"]):
                continue
            valid += 1
    print(valid)


main()
