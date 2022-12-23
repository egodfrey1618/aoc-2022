from typing import List, Tuple
import re

class Formula(object):
    def __init__(self, monkey, str):
        self.monkey = monkey
        x = re.findall("^[0-9]*$", str)

        # No parsing for EQUAL, I just change that by hand below.
        if x != []:
            self.formula = ("NUMBER", int(x[0]))
        else:
            formula = re.findall("^([a-z]+) (.) ([a-z]+)$", str)[0]
            formula = ("FORMULA", (formula[1], formula[0], formula[2]))
            self.formula = formula

    def relevant_monkeys(self):
        l = [self.monkey]
        if self.formula[0] == "FORMULA":
            l.extend(self.formula[1][1:]) # type: ignore
        elif self.formula[0] == "EQUAL":
            l.append(self.formula[1]) # type: ignore
        return l
    
    # If we have len(relevant_monkeys) - 1 values, gives you the other.
    def solve_exn(self, other_monkey_values) -> Tuple[str, int]:
        if self.formula[0] == "NUMBER":
            return (self.monkey, self.formula[1]) # type: ignore
        elif self.formula[0] == "EQUAL":
            monkey0 = self.monkey
            monkey1 = self.formula[1]
            if monkey0 in other_monkey_values:
                (monkey0, monkey1) = (monkey1, monkey0)
            return (monkey0, other_monkey_values[monkey1]) # type: ignore
        elif self.formula[0] == "FORMULA":
            operation = self.formula[1][0] # type: ignore
            monkey0 = self.monkey
            monkey1 = self.formula[1][1] # type: ignore
            monkey2 = self.formula[1][2] # type: ignore

            known = [m in other_monkey_values for m in [monkey0, monkey1, monkey2]]

            # Shuffle around the formulas so the unknown monkey is at the front.
            # This is just a bunch of horrendous case-bashing.
            if known.count(False) != 1:
                assert False, "[solve_exn] was called, but we didn't know 2 of the monkeys."
            if known[0] == False:
                # Keep as-is.
                pass
            elif known[1] == False:
                (monkey0, monkey1, monkey2) = (monkey1, monkey0, monkey2)
                # M0 = M1 + M2 => M1 = M0 - M2
                # M0 = M1 - M2 => M1 = M0 + M2
                # M0 = M1 * M2 => M1 = M0 / M2
                # M0 = M1 / M2 => M1 = M0 * M2
                if operation == "+":
                    operation = "-"
                elif operation == "-":
                    operation = "+"
                elif operation == "*":
                    operation = "/"
                elif operation == "/":
                    operation = "*"
                else: 
                    assert False
            elif known[2] == False:
                # M0 = M1 + M2 => M2 = M0 - M1
                # M0 = M1 - M2 => M2 = M1 - M0
                # M0 = M1 * M2 => M2 = M0 / M1
                # M0 = M1 / M2 => M2 = M1 / M0
                if operation in "-/":
                    (monkey0, monkey1, monkey2) = (monkey2, monkey1, monkey0)
                else:
                    (monkey0, monkey1, monkey2) = (monkey2, monkey0, monkey1)
                if operation == "+":
                    operation = "-"
                elif operation == "*":
                    operation = "/"

            # Actually do the operation.
            v1 = other_monkey_values[monkey1]
            v2 = other_monkey_values[monkey2]
            if operation == "+":
                f = lambda x, y : x + y
            elif operation == "-":
                f = lambda x, y : x - y
            elif operation == "*":
                f = lambda x, y : x * y
            elif operation == "/":
                f = lambda x, y : x // y
            else:
                assert False
            return (monkey0, f(v1, v2))
        else:
            assert False, "BUG, wrong formula tag"

def solve(formulas : List[Formula]):
    monkey_to_formula_indices = {}
    for index, formula in enumerate(formulas):
        for monkey in formula.relevant_monkeys():
            if monkey not in monkey_to_formula_indices:
                monkey_to_formula_indices[monkey] = []
            monkey_to_formula_indices[monkey].append(index)
        
    sizes = [len(formula.relevant_monkeys()) for formula in formulas]
    monkey_values = {}

    print(monkey_to_formula_indices)

    formulas_to_process = [i for i, f in enumerate(formulas) if sizes[i] == 1]
    while formulas_to_process:
        formula_index = formulas_to_process.pop()
        if sizes[formula_index] == 1:
            formula = formulas[formula_index]
            (monkey, value) = formula.solve_exn(monkey_values)
            monkey_values[monkey] = value
            for formula_index2 in monkey_to_formula_indices[monkey]:
                sizes[formula_index2] -= 1
                if sizes[formula_index2] == 1:
                    formulas_to_process.append(formula_index2)
    print(monkey_values)

formulas = []
s = open("input").read().strip().split("\n")
for line in s:
    monkey_name = re.findall("^([a-z]*): ", line)[0]
    rest = re.findall("^.*: (.*)$", line)[0]
    formulas.append(Formula(monkey_name, rest))

# Part 1
solve(formulas)

# Part 2 - hack around with the formulas, then rerun my solver.
root_formula = [f for f in formulas if f.monkey == "root"][0]
formulas = [f for f in formulas if f.monkey != "root" and f.monkey != "humn"]

new_formula = Formula("xxxx", "3")
new_formula.monkey = root_formula.formula[1][1]
new_formula.formula = ("EQUAL", root_formula.formula[1][2]) # type: ignore
formulas.append(new_formula)

solve(formulas)