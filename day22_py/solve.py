import re
from typing import Tuple, Optional

class GridWithSpaces(object):
    def __init__(self):
        self.width = 0
        self.rows = []

    def add_row(self, row):
        # Empty space means... empty space!
        # It's possible a row doesn't extend long enough - if so, this will extend it.
        row_width = len(row)
        if row_width > self.width:
            for i, r in enumerate(self.rows):
                self.rows[i] = r + " " * (row_width - self.width)
            self.width = row_width
        else:
            row = row + " " * (self.width - row_width)

        self.rows.append(row)

    def move(self, pos, dir) -> Optional[Tuple[int, int]]:
        def get_value(pos):
            return self.rows[pos[0]][pos[1]]

        assert get_value(pos) == "."

        (d1, d2) = dir
        while True:
            (x, y) = pos
            pos = ((x + d1) % len(self.rows), (y + d2) % self.width)
            value = get_value(pos)
            if value == ".":
                return pos
            elif value == "#":
                # We hit a wall before hitting an empty space, so we can't move.
                return None
            elif value == " ":
                # Continue walking. 
                continue
            else:
                assert False

    # Unlike above, doesn't return None if we can't move, returns the actual position we end up in.
    def move_n(self, pos, dir, n) -> Tuple[int, int]:
        for _ in range(n):
            next_pos = self.move(pos, dir)
            if next_pos == None:
                break
            else:
                pos = next_pos
        return pos

    def start_pos(self) -> Tuple[int, int]:
        # First free cell in first row
        row = self.rows[0]
        cols = [i for i, c in enumerate(row) if c == "."]
        return (0, cols[0])

class Cube(object):
    def __init__(self, faces):
        self.faces = faces[:]
        self.glued_points = {}
        self.width = len(self.faces[0])

    # Glue two edges of a face together. The dirs should point "out" of the face.
    def glue(self, face1, face2, dir1, dir2, flip):
        def points_on_edge(dir):
            if dir == (0, -1):
                return [(x, 0) for x in range(self.width)]
            elif dir == (0, 1):
                return [(x, self.width - 1) for x in range(self.width)]
            elif dir == (-1, 0):
                return [(0, x) for x in range(self.width)]
            elif dir == (1, 0):
                return [(self.width - 1, x) for x in range(self.width)]
            else:
                assert False

        def opposite_dir(dir):
            (x, y) = dir
            return (-1 * x, -1 * y)

        points1 = points_on_edge(dir1)
        points2 = points_on_edge(dir2)
        if flip: points2 = points2[::-1]
    
        for (p1, p2) in zip(points1, points2):
            p1_off_edge = tuple(x + d for x, d in zip(p1, dir1))
            p2_off_edge = tuple(x + d for x, d in zip(p2, dir2))
            self.glued_points[(face1, p1_off_edge, dir1)] = (face2, p2, opposite_dir(dir2))
            self.glued_points[(face2, p2_off_edge, dir2)] = (face1, p1, opposite_dir(dir1))

    def move(self, face, pos, dir):
        (orig_face, orig_pos, orig_dir) = (face, pos, dir)

        (x, y) = pos
        (d1, d2) = dir
        pos = (x+d1, y+d2)

        if pos[0] < self.width and pos[1] < self.width and pos[0] >= 0 and pos[1] >= 0:
            (face, pos, dir) = (orig_face, pos, orig_dir)
        else:
            # Otherwise, this should be one of our glued edges.
            (face, pos, dir) = self.glued_points[(orig_face, pos, orig_dir)]

        value = self.faces[face][pos[0]][pos[1]] 
        if value == ".":
            return (face, pos, dir)
        elif value == "#":
            return (orig_face, orig_pos, orig_dir)
        else:
            assert False

    def move_n(self, face, pos, dir, n):
        for _ in range(n):
            (face, pos, dir) = self.move(face, pos, dir)
        return (face, pos, dir)

    def start_pos(self):
        return (0, 0)

up = (-1, 0)
down = (1, 0)
left = (0, -1)
right = (0, 1)

def turn_dir(current_dir, turn):
    dirs = [right, down, left, up]

    dir_index = dirs.index(current_dir)
    if turn == "R":
        dir_index += 1
    elif turn == "L":
        dir_index -= 1
    else:
        assert False
    dir_index %= len(dirs)
    return dirs[dir_index]

def score_dir(dir):
    dirs = [right, down, left, up]
    return dirs.index(dir)

# Parsing
s = open("input")
lines = []
while True:
    line = s.readline().rstrip()
    if line == "":
        break
    lines.append(line)

parsed_moves = []
moves = s.readline().strip()
moves = re.findall("([0-9]+[LR]?)", moves)
for move in moves:
    if move[-1] in "LR":
        parsed_moves.append(int(move[:-1]))
        parsed_moves.append(move[-1])
    else:
        parsed_moves.append(int(move))

# Part 1
grid = GridWithSpaces()
for line in lines:
    grid.add_row(line)

pos = grid.start_pos()
dir = right

for move in parsed_moves:
    if type(move) == int:
        pos = grid.move_n(pos, dir, move)
    elif type(move) == str:
        dir = turn_dir(dir, move)
    else:
        assert False

(row, col) = pos
row += 1
col += 1
print(1000 * row + 4 * col + score_dir(dir))

# Part 2
"""
This is pretty gross - a lot of this reasoning was done by hand and some scribbles.

The real input looks like this:

 01
 2
34
5

I can imagine this folding into a cube where these are the faces:
0 - top
2 - front
1 - right
4 - bottom
3 - left
5 - back

Now, I found it REALLY hard to visualise what orientation of these should be. 
Finding the right orientation of edges to glue together below was largely drawing out lots
of pictures, and lots of squinting to decide how to flip the edges.
"""
WIDTH = 50
face0 = [row[WIDTH:WIDTH*2] for row in grid.rows[:WIDTH]]
face1 = [row[WIDTH*2:] for row in grid.rows[:WIDTH]]
face2 = [row[WIDTH:WIDTH*2] for row in grid.rows[WIDTH:WIDTH*2]]
face3 = [row[:WIDTH] for row in grid.rows[WIDTH*2:WIDTH*3]]
face4 = [row[WIDTH:WIDTH*2] for row in grid.rows[WIDTH*2:WIDTH*3]]
face5 = [row[:WIDTH] for row in grid.rows[WIDTH*3:WIDTH*4]]

cube = Cube([face0, face1, face2, face3, face4, face5])
# We have 12 pairs of edges that need to be glued together. I mostly figured this out by hand.
cube.glue(0, 2, down, up, False)
cube.glue(2, 4, down, up, False)
cube.glue(4, 5, down, right, False)
cube.glue(5, 0, left, up, False) # the "strip" starting at the top, around the front.
cube.glue(0, 1, right, left, False)
cube.glue(1, 4, right, right, True)
cube.glue(4, 3, left, right, False)
cube.glue(3, 0, left, left, True) # the "strip" starting at the top, around the sides
cube.glue(2, 1, right, down, False)
cube.glue(1, 5, up, down, False)
cube.glue(5, 3, up, down, False)
cube.glue(3, 2, up, left, False) # the "strip" around the middle

pos = cube.start_pos()
face = 0
dir = right
for move in parsed_moves:
    if type(move) == int:
        (face, pos, dir) = cube.move_n(face, pos, dir, move)
    elif type(move) == str:
        dir = turn_dir(dir, move)
    else:
        assert False
print(face, pos, dir)

# I then hardcoded these based on the answer that above gave me.
row = 11
col = 100+12
dir = (-1, 0)
print(1000 * row + 4 * col + score_dir(dir))