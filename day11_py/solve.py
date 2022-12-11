class Monkey(object):
    def __init__(self, items, worry_adjust, monkey_check, monkey_true, monkey_false):
        self.items = items
        self.worry_adjust = worry_adjust
        self.monkey_check = monkey_check
        self.monkey_true = monkey_true
        self.monkey_false = monkey_false

    def pop(self):
        if self.items == []:
            return None
        item = self.items.pop(0)

        if self.worry_adjust == "SQUARE":
            item *= item
        elif self.worry_adjust[0] == "TIMES":
            item *= self.worry_adjust[1]
        elif self.worry_adjust[0] == "ADD":
            item += self.worry_adjust[1]
        else:
            assert False
        item //= 3
        if item % self.monkey_check == 0:
            return (item, self.monkey_true)
        else:
            return (item, self.monkey_false)

    def push(self, item):
        self.items.append(item)

monkeys = []
text = open("input").read().split("\n")
for line in text:
    if line.startswith("Monkey"): continue

    elif line.startswith("  Starting items"):
        line = line.split(": ")[1]
        items = [int(x) for x in line.split(",")]

    elif line.startswith("  Operation: new = "):
        if "old * old" in line:
            worry_adjust = "SQUARE"
        elif "old * " in line:
            ending = int(line.split(" ")[-1])
            # EG: Clearly this should be a lambda function, but I couldn't work out how to get that to close over ending correctly...
            worry_adjust = ("TIMES", ending)
        elif "old + " in line:
            ending = int(line.split(" ")[-1])
            worry_adjust = ("ADD", ending)
        else:
            assert False
    
    elif line.startswith("  Test: divisible by"):
        n = int(line.split(" ")[-1])
        monkey_check = n

    elif line.startswith("    If true:"):
        monkey_true = int(line.split(" ")[-1])
    elif line.startswith("    If false:"):
        monkey_false = int(line.split(" ")[-1])

    elif line == "":
        monkeys.append(Monkey(items, worry_adjust, monkey_check, monkey_true, monkey_false))
    else:
        assert False

monkey_counts = [0 for _ in monkeys]

ROUNDS = 20
for round in range(ROUNDS):
    for i, monkey in enumerate(monkeys):
        while (result := monkey.pop()) != None:
            (item, j) = result
            monkeys[j].push(item)
            monkey_counts[i] += 1

    print("After round {}".format(round+1))
    for i, monkey in enumerate(monkeys):
        print(i, monkey.items)

print("Final monkey counts", monkey_counts)
monkey_counts.sort()
print(monkey_counts[-1] * monkey_counts[-2])


