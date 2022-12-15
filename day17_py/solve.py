# y goes down: y=0 is the floor, y=10 is 10 steps in the air.

NUMBER_OF_SHAPES = 5

class Shape(object):
    def __init__(self, n, highest_rock):
        n %= NUMBER_OF_SHAPES
        
        # Co-ordinates are all measured relative to a point on the left-hand side.
        if n == 0:
            self.coords = [(0, 0), (1, 0), (2, 0), (3, 0)]
        elif n == 1:
            self.coords = [(0, 0), (1, 0), (2, 0), (1, 1), (1, -1)]
        elif n == 2:
            self.coords = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]
        elif n == 3:
            self.coords = [(0, y) for y in range(0, 4)]
        elif n == 4:
            self.coords = [(0, 0), (0, 1), (1, 0), (1, 1)]
        else:
            assert False

        self.x_coord = 2
        lowest_y_position_in_shape = min(x[1] for x in self.coords)
        self.y_coord = highest_rock + 3 - lowest_y_position_in_shape

    def absolute_coords(self):
        for (x, y) in self.coords:
            yield (self.x_coord + x, self.y_coord + y)

GRID_WIDTH = 7
class Grid(object):
    def _add_empty_row(self):
        self.grid.append([False for _ in range(GRID_WIDTH)])

    def _smallest_rock(self):
        return len(self.grid)

    def _add_shape(self):
        self.shape = Shape(self.shape_index, self._smallest_rock())
        self.shape_index += 1

    def __init__(self):
        self.grid = []
        self.shape_index = 0

    def _shape_fits(self):
        for (x, y) in self.shape.absolute_coords():
            if x < 0 or x >= GRID_WIDTH:
                return False
            if y < 0: 
                return False
            if y >= len(self.grid):
                # This is allowed - we don't add rows until we freeze a piece
                pass
            else:
                if self.grid[y][x] == True:
                    return False
        return True
    
    def try_move_shape(self, direction) -> bool:
        self.shape.x_coord += direction[0]
        self.shape.y_coord += direction[1]

        if self._shape_fits():
            return True
        else:
            self.shape.x_coord -= direction[0]
            self.shape.y_coord -= direction[1]
            return False

    def freeze_piece(self):
        for (x, y) in self.shape.absolute_coords():
            while y >= len(self.grid):
                self._add_empty_row()
            self.grid[y][x] = True
        self._add_shape()

    def print_top(self):
        coords = set(self.shape.absolute_coords())

        def print_row(y, row):
            chars = ["#" if r else "." for r in row]
            for x in range(GRID_WIDTH):
                if (x, y) in coords:
                    chars[x] = "@"
            print("".join(chars))
        
        def print_row_y(y):
            if y < len(self.grid):
                print_row(y, self.grid[y])
            else:
                print_row(y, [False for _ in range(GRID_WIDTH)])

        for y in range(len(self.grid) + 7, max(len(self.grid) - 3, 0), -1):
            print_row_y(y)

s = open("input").read().strip()

def single_step(g, s_index):
    while True:
        move = s[s_index % len(s)]
        s_index += 1
    
        if move == ">":
            g.try_move_shape((1, 0))
        elif move == "<":
            g.try_move_shape((-1, 0))
        else:
            assert False
           
        if not g.try_move_shape((0, -1)):
            g.freeze_piece()
            break
    return s_index

def simulate_grid(steps) -> Grid:
    g = Grid()
    g._add_shape()
    s_index = 0
    for i in range(steps):
        s_index = single_step(g, s_index)
    return g

# Part 1: Nice and simple.
total_pieces = 2022
print(simulate_grid(total_pieces)._smallest_rock())

# Part 2, ... less simple.
g = Grid()
g._add_shape()
s_index = 0

for i in range(1, 10_000):
    s_index = single_step(g, s_index)
    if s_index % len(s) == 0 and g.shape_index % NUMBER_OF_SHAPES == 0:
        print(f"After {i} moves, our counters are back to the start")
        g.print_top()

# OK, so with that little experiement above: 
# At move 1724, and every 1725 moves after, we have the same pattern, and our jet counter is back to the start.
# So we'll repeat this state every 1725 moves.

def compute_using_big_steps(x):
    START = 1724
    BIG_STEP = 1725
    if x <= START:
        assert False, "only works for bigger x's, sorry"

    number_of_big_steps = (x - START) // BIG_STEP
    little_step_size = (x - START) % BIG_STEP

    initial_height = simulate_grid(START)._smallest_rock()

    size_of_big_step = simulate_grid(START + BIG_STEP)._smallest_rock() - initial_height
    size_of_little_step = simulate_grid(START + little_step_size)._smallest_rock() - initial_height

    return (number_of_big_steps * size_of_big_step) + size_of_little_step + initial_height

print(compute_using_big_steps(10000))
print(simulate_grid(10000)._smallest_rock())

print(compute_using_big_steps(1000000000000))