BASE = 5

def to_snafu_digit(d : int) -> str:
    if d == -2:
        return "="
    elif d == -1:
        return "-"
    elif d in [0, 1, 2]:
        return str(d)
    else:
        assert False, "invalid digit"

def to_snafu(n : int) -> str:
    result = []
    while n > 0:
        d = n % BASE
        if d > 2:
            d -= 5
        n = (n - d) // BASE
        assert d in [-2, -1, 0, 1, 2]
        result.append(d)
    result = result[::-1]
    if result == []:
        return "0"
    else:
        return "".join(to_snafu_digit(d) for d in result)

def of_snafu_digit(c : str) -> int:
    for d in [-2, -1, 0, 1, 2]:
        if c == to_snafu_digit(d):
            return d
    assert False
    
def of_snafu(s : str) -> int:
    n = 0
    for c in s:
        n *= BASE
        n += of_snafu_digit(c)
    return n

numbers = []
s = open("input").read().strip().split("\n")

for line in s:
    numbers.append(of_snafu(line))
print(to_snafu(sum(numbers)))