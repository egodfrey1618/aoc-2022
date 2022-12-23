from typing import List, Tuple

# Stores positions as a dense grid. Can expand if you add new points.
class DynamicGrid(object):
    def _is_in_grid(self, pos):
        (x, y) = pos
        if x < self.min_x or x >= self.max_x:
            return False
        if y < self.min_y or y >= self.max_y:
            return False
        return True

    def get(self, pos):
        if not self._is_in_grid(pos):
            return False

        (x, y) = pos
        return self.grid[x - self.min_x][y - self.min_y]

    def __init__(self, init_min_x, init_max_x, init_min_y, init_max_y):
        self.grid = [[False for _ in range(init_min_y, init_max_y)] for _ in range(init_min_x, init_max_x)]
        self.min_x = init_min_x
        self.max_x = init_max_x
        self.min_y = init_min_y
        self.max_y = init_max_y

    def _expand(self, min_x, max_x, min_y, max_y) -> None:
        print("Expanding grid...")
        min_x = min(min_x, self.min_x)
        max_x = max(max_x, self.max_x)
        min_y = min(min_y, self.min_y)
        max_y = max(max_y, self.max_y)

        grid = [[False for _ in range(min_y, max_y)] for _ in range(min_x, max_x)]
        for x in range(self.min_x, self.max_x):
            for y in range(self.min_y, self.max_y):
                grid[x - min_x][y - min_y] = self.grid[x - self.min_x][y - self.min_y]
        self.grid = grid
        self.min_x = min_x
        self.max_x = max_x
        self.min_y = min_y
        self.max_y = max_y

    def set(self, pos, value) -> None:
        (x, y) = pos
        if not self._is_in_grid(pos) and value == True:
            min_x = min(self.min_x, x) - 100
            max_x = max(self.max_x, x) + 100
            min_y = min(self.min_y, y) - 100
            max_y = max(self.max_y, y) + 100
            self._expand(min_x, max_x, min_y, max_y)
        
        (x, y) = pos
        self.grid[x - self.min_x][y - self.min_y] = value

    def _actual_bounding_box(self):
        # Prints the ACTUAL bounding box, which might be different if we have blank spaces around the edges.
        positions = []
        for x in range(self.min_x, self.max_x):
            for y in range(self.min_y, self.max_y):
                pos = (x, y)
                if self.get(pos):
                    positions.append(pos)
        min_x = min(x for (x,y) in positions)
        max_x = max(x for (x,y) in positions)
        min_y = min(y for (x,y) in positions)
        max_y = max(y for (x,y) in positions)
        return (min_x, max_x + 1, min_y, max_y + 1)

    def print(self):
        (min_x, max_x, min_y, max_y) = self._actual_bounding_box()

        for x in range(min_x, max_x):
            row = []
            for y in range(min_y, max_y):
                row.append("#" if self.get((x, y)) else ".")
            print("".join(row))

def move_to_dir(move) -> Tuple[int, int]:
    if move == "W":
        return (0, -1)
    elif move == "E":
        return (0, 1)
    elif move == "N":
        return (-1, 0)
    elif move == "S":
        return (1, 0)
    else:
        assert False

def spaces_that_must_be_free(move) -> List[Tuple[int, int]]:
    dir = move_to_dir(move)
    result = [dir]
    index = dir.index(0)

    def make_altered_in_0_index(i):
        l = list(dir)
        l[index] = i
        return tuple(l)

    result.append(make_altered_in_0_index(1))
    result.append(make_altered_in_0_index(-1))
    return result

def move(pos, dir) -> Tuple[int, int]:
    return (pos[0] + dir[0], pos[1] + dir[1])

move_to_dir_cache = {m : move_to_dir(m) for m in "NSEW"}
spaces_that_must_be_free_cache = {m : spaces_that_must_be_free(m) for m in "NSEW"}
all_dirs = [(x, y) for x in range(-1, 2) for y in range(-1, 2) if (x, y) != (0, 0)]

class ElfConfiguration(object):
    def __init__(self, positions):
        self.grid = DynamicGrid(-10, 10, -10, 100)
        self.elves = positions
        self.move_choices = "NSWE"

        for elf in self.elves:
            self.grid.set(elf, True)

    def simulate_move(self) -> int:
        proposed_positions = []

        for pos in self.elves:
            # If all directions are free, don't move this elf.
            if not any(self.grid.get(move(pos, dir)) for dir in all_dirs):
                proposed_positions.append(pos)
                continue

            # Otherwise, we need to check each possible move.
            moved = False
            for possible_move in self.move_choices:
                if not any(self.grid.get(move(pos, dir)) for dir in spaces_that_must_be_free_cache[possible_move]):
                    new_pos = move(pos, move_to_dir_cache[possible_move])
                    proposed_positions.append(new_pos)
                    moved = True
                    break

            if not moved:
                proposed_positions.append(pos)

        proposed_positions_set = set()
        duped_positions_set = set()

        for p in proposed_positions:
            if p in proposed_positions_set:
                duped_positions_set.add(p)
            else:
                proposed_positions_set.add(p)

        for (i, (original, new)) in enumerate(zip(self.elves, proposed_positions)):
            if new in duped_positions_set:
                proposed_positions[i] = original

        # Actually perform our moves!
        number_different = 0
        for (original, new) in zip(self.elves, proposed_positions):
            if original != new:
                number_different += 1
                self.grid.set(original, False)
                self.grid.set(new, True)
        self.elves = proposed_positions

        # And update our move_choices
        self.move_choices = self.move_choices[1:] + self.move_choices[0]

        # And return how many elves moved
        return number_different
    
    def print(self):
        self.grid.print()

    def score(self):
        (min_x, max_x, min_y, max_y) = self.grid._actual_bounding_box()
        box = (max_y - min_y) * (max_x - min_x)
        return box - len(self.elves)

s = open("input").read().strip().split("\n")
elf_positions = []
for i, line in enumerate(s):
    for j, c in enumerate(line):
        if c == "#":
            elf_positions.append((i, j))
elf_configuration = ElfConfiguration(elf_positions)

elf_configuration.print()
round = 1
while True:
    moved = elf_configuration.simulate_move()
    print(round, moved)
    if moved == 0:
        break
    round += 1
print(round)