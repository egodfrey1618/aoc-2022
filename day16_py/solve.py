from copy import deepcopy
from typing import List, Optional
import re
import random

class Graph(object):
    def __init__(self):
        self.name_to_index = {}
        self.flow_rates = []
        self.edge_matrix : List[List[Optional[int]]] = []

    def add_vertex(self, vertex, flow_rate):
        self.name_to_index[vertex] = len(self.name_to_index)
        self.flow_rates.append(flow_rate)
        for row in self.edge_matrix:
            row.append(None)
        self.edge_matrix.append([None] * len(self.name_to_index))

    def add_edge(self, v1, v2, weight):
        i = self.name_to_index[v1]
        j = self.name_to_index[v2]
        self.edge_matrix[i][j] = weight

    def __str__(self):
        s = ""
        s += "Vertex names:\n"
        s += str(self.name_to_index) + "\n"
        for i, row in enumerate(self.edge_matrix):
            s += "{}: {}\n".format(i, self.edge_matrix[i])
        return s

STARTING_VERTEX = "AA"
ALLOWED_TIME = 30

def compress_zero_flow_rate(g : Graph) -> Graph:
    # Step 1: Floyd-Marshall, to solve the APSP problem.
    apsp_matrix = [row[:] for row in g.edge_matrix]

    for i in range(len(g.name_to_index)):
        for j in range(len(g.name_to_index)):
            for k in range(len(g.name_to_index)):
                if apsp_matrix[j][i] != None and apsp_matrix[i][k] != None:
                    candidate = apsp_matrix[j][i] + apsp_matrix[i][k] # type: ignore
                    if apsp_matrix[j][k] == None or apsp_matrix[j][k] > candidate:
                        apsp_matrix[j][k] = candidate

    # Step 2: Only keep some of the vertices.
    new_graph = Graph()
    keep_vertices = [v for (v, i) in g.name_to_index.items() if g.flow_rates[i] != 0]
    if STARTING_VERTEX not in keep_vertices:
        keep_vertices.append(STARTING_VERTEX)

    for v in keep_vertices:
        new_graph.add_vertex(v, g.flow_rates[g.name_to_index[v]])
    for v1 in keep_vertices:
        for v2 in keep_vertices:
            if v1 != v2:
                weight = apsp_matrix[g.name_to_index[v1]][g.name_to_index[v2]]
                new_graph.add_edge(v1, v2, weight)

    return new_graph

# Step 1: Parse the graph.
lines = open("input").read().strip().split("\n")
parsed = []
for line in lines:
    valve = re.findall("Valve ([A-Z]*) ", line)[0]
    flow_rate = int(re.findall("flow rate=([0-9]*);", line)[0])
    connected = re.findall("tunnels? leads? to valves? (.*)", line)[0].split(", ")
    parsed.append((valve, flow_rate, connected))

g = Graph()
for (v, flow_rate, _) in parsed:
    g.add_vertex(v, flow_rate)
for (v1, _flow_rate, neighbours) in parsed:
    for v2 in neighbours:
        g.add_edge(v1, v2, 1)

# Step 2: Compress flow rate 0 vertices out of the graph (except for the starting one)
g = compress_zero_flow_rate(g)

# Step 3: We're going to define [State], which is a partial path through the graph
class PartialState(object):
    def __init__(self, time_remaining, disallow_these_vertices):
        self.path = [g.name_to_index[STARTING_VERTEX]]
        self.disallow_these_vertices = set(disallow_these_vertices)
        self.time_remaining = time_remaining
        self.total_flow = 0
        self.total_flow_rate = 0

    def copy(self) -> "PartialState":
        s = deepcopy(self)
        return s

    def add_vertex_exn(self, vertex) -> None:
        if vertex in self.path:
            assert False, "can't add vertex to path more than once"
 
        # The +1 includes the time for opening this valve.
        current_vertex = self.path[-1]
        time_to_get_there = g.edge_matrix[current_vertex][vertex] + 1 # type: ignore

        if time_to_get_there > self.time_remaining:
            assert False, "wouldn't have time to reach this matrix and turn the flow on"

        self.path.append(vertex)
        self.total_flow += (self.total_flow_rate * time_to_get_there)
        self.total_flow_rate += g.flow_rates[vertex]
        self.time_remaining -= time_to_get_there

    def possible_next_vertices(self) -> List[int]:
        result = []
        path_set = set(self.path)
        for v in g.name_to_index.values():
            if v not in path_set and v not in self.disallow_these_vertices:
                # The +1 includes the time for opening this valve.
                if g.edge_matrix[self.path[-1]][v] + 1 <= self.time_remaining: # type: ignore
                    result.append(v)
        return result

    def total_flow_finishing_here(self):
        return self.total_flow + self.total_flow_rate * self.time_remaining

# Step 4: Brute-force. I was going to do some kind of heuristic here to prune the search space.
# But it turned out to be not necessary - this takes ~3s on the real input.
state = PartialState(ALLOWED_TIME, [])
best_solution = ([], -1)

def bfs(state : PartialState):
    global best_solution
    next_vertices = state.possible_next_vertices()
    if next_vertices == []:
        solution = state.total_flow_finishing_here()
        yield (state.path, solution)
    else:
        for next_vertex in next_vertices:
            next_state = state.copy()
            next_state.add_vertex_exn(next_vertex)
            for x in bfs(next_state):
                yield x

if False:
    best_with_just_human = -1
    for s in bfs(state):
        best_with_just_human = max(best_with_just_human, s[1])
    print("Best with just the human, part 1: {}".format(best_with_just_human))

# Step 5: The same sort of idea, but with the elephant.
# This works like so:
# - W.l.o.g., we're going to assume the human manages to open at least as much flow as the elephant
# - Compute all possible paths for the human, put them in decreasing order of score
# - For each one, find the best path for the elephant that doesn't overlap with the human path
# - We can stop when the human score is less than half of the lower bound so far (because of the w.l.o.g.)
#
# This took 24s, which I'm broadly happy with. I suspect an improvement would be to throw a heuristic into
# the BFS to trim paths that aren't going to end up big enough.
ELEPHANT_TRAINING_TIME = 4

state = PartialState(ALLOWED_TIME - ELEPHANT_TRAINING_TIME, [])
human_solutions = list(bfs(state))
human_solutions.sort(key=lambda s: s[1], reverse=True)
lower_bound = human_solutions[0][1]

print("Computed all possible paths for the human, now working through them in order...")
print("Best with just the human, part 2: {}".format(lower_bound))

for (path, score) in human_solutions:
    if score * 2 <= lower_bound:
        # There's no point continuing. W.l.o.g. the human gave us more flow. So break here.
        break

    elephant_state = PartialState(ALLOWED_TIME - ELEPHANT_TRAINING_TIME, path)
    elephant_best = max(h[1] for h in bfs(elephant_state))

    lower_bound = max(lower_bound, score + elephant_best)
print("Best I got, working with the elephant: {}".format(lower_bound))