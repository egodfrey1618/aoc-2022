# Done quickly because I'm in a rush to get out, so some of this is a little messy / hardcoded.
from typing import List, Tuple

lines = open("input").read().strip().split("\n")

droplet_cubes = []
for line in lines:
    droplet_cubes.append(tuple([int(x) for x in line.split(",")]))
droplet_cubes = set(droplet_cubes)

def neighbours(cube) -> List[Tuple[int, int, int]]:
    result = []
    for coord in range(3):
        for diff in [-1, 1]:
            next_cube = list(cube)
            next_cube[coord] += diff
            next_cube = tuple(next_cube)
            result.append(next_cube)
    return result

# Part 1
total_faces = 0
for cube in droplet_cubes:
    total_neighbours = len([x for x in neighbours(cube) if x in droplet_cubes])
    this_cube_faces = 6 - total_neighbours
    total_faces += this_cube_faces
print(total_faces)

# Part 2: OK, a bit different this time. 
# Rough plan: We're going to try and find connected components of points outside the droplet.
# The outside one will be of infinite size, so we'll stop once we're, idk, bigger than the 
# size of the droplet overall.
def find_component(start_cube, bound):
    result = [start_cube]
    result_set = set(result)
    result_position = 0

    while result_position < len(result):
        cube = result[result_position]
        result_position += 1

        for neighbour in neighbours(cube):
            if neighbour in droplet_cubes or True in [abs(i) > bound for i in cube]:
                # Inside the droplet, or outside bounds.
                continue
            if neighbour in result_set:
                # Already processed.
                continue
            result.append(neighbour)
            result_set.add(neighbour)

    return result

found = droplet_cubes.copy()
cube_to_component = {}
components = []

BOUND = 25 # Just eyeballed this, too lazy to calculate.

for i in range(-1*BOUND, BOUND):
    for j in range(-1*BOUND, BOUND):
        for k in range(-1*BOUND, BOUND):
            cube = (i, j, k)
            if cube not in found:
                component = find_component(cube, BOUND)
                component_num = len(components)
                cube_to_component[cube] = component_num
                components.append(component)
                found = found.union(component)

# 2_000 was eyeballed. There's a large internal component in one of mine!
internal_components = [c for c in components if len(c) < 2_000] 
for component in internal_components:
    for cube in component:
        droplet_cubes.add(cube)

total_faces = 0
for cube in droplet_cubes:
    total_neighbours = len([x for x in neighbours(cube) if x in droplet_cubes])
    this_cube_faces = 6 - total_neighbours
    total_faces += this_cube_faces
print(total_faces)