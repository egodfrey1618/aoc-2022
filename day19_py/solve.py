import random
import re
from typing import List, Optional

NUMBER_OF_MINERALS = 4
GEODE_INDEX = 3
OBSIDIAN_INDEX = 2

# One test case: Blueprints for each kind of mineral robot.
class Blueprints(object):
    def __init__(self, blueprints):
        assert len(blueprints) == NUMBER_OF_MINERALS
        assert all(len(b) == NUMBER_OF_MINERALS for b in blueprints)
        self.blueprints = blueprints

        self.max_mineral_for_any_blueprint = [max(blueprint[i] for blueprint in blueprints) for i in range(NUMBER_OF_MINERALS)]

class State(object):
    def __init__(self, time, minerals, mineral_rates):
        assert len(minerals) == NUMBER_OF_MINERALS
        assert len(mineral_rates) == NUMBER_OF_MINERALS

        self.time = time
        self.minerals = minerals
        self.mineral_rates = mineral_rates

    def is_at_least_as_good_as(self, other) -> bool:
        if not all(m_self >= m_other for (m_self, m_other) in zip(self.minerals, other.minerals)):
            return False
        if not all(m_self >= m_other for (m_self, m_other) in zip(self.mineral_rates, other.mineral_rates)):
            return False
        return True

    def time_to_get_to(self, target_minerals) -> Optional[int]:
        t = 0
        for i, (self_m, target_m) in enumerate(zip(self.minerals, target_minerals)):
            if target_m <= self_m:
                # We already have enough of this mineral
                continue
            mineral_rate = self.mineral_rates[i]
            if mineral_rate == 0:
                # We're not producing this mineral, so will never get this with our current robots.
                return None
            
            amount_needed = target_m - self_m
            time_for_this_mineral = (amount_needed // mineral_rate) + (1 if amount_needed % mineral_rate != 0 else 0)
            t = max(t, time_for_this_mineral)
        return t

    # What possible next states are there? We'll key these off "the next move is to use blueprint X"
    def possible_next_states(self, blueprints : Blueprints) -> List["State"]:
        result = []

        for mineral, blueprint in enumerate(blueprints.blueprints):
            if mineral != GEODE_INDEX and self.mineral_rates[mineral] >= blueprints.max_mineral_for_any_blueprint[mineral]:
                # Optimisation: skip this one. We already have enough robots of this kind to get enough minerals
                # per turn, so this is never going to improve our ability to make more.
                continue

            time_to_have_enough_minerals_for_blueprint = self.time_to_get_to(blueprint)

            if time_to_have_enough_minerals_for_blueprint == None:
                continue

            # We also need one minute for constructing the blueprint itself.
            time_passes = time_to_have_enough_minerals_for_blueprint + 1
            new_time = self.time + time_passes
            new_minerals = []
            for i in range(NUMBER_OF_MINERALS):
                old_amount = self.minerals[i]
                new_amount_before_blueprint = old_amount + (self.mineral_rates[i] * time_passes)
                new_amount_after_blueprint = new_amount_before_blueprint - blueprint[i]
                new_minerals.append(new_amount_after_blueprint)
            new_mineral_rates = self.mineral_rates[:]
            new_mineral_rates[mineral] += 1

            state = State(new_time, new_minerals, new_mineral_rates)
            result.append(state)
        return result

    def lower_bound_for_time_until_next_geode_robot(self, blueprints):
        required_obsidian = blueprints.blueprints[GEODE_INDEX][OBSIDIAN_INDEX]
        obsidian_rate = self.mineral_rates[OBSIDIAN_INDEX]
        obsidian_amount = self.minerals[OBSIDIAN_INDEX]

        extra_time = 0
        # Let's just assume that I can build obsidian robots as fast as possible.
        while obsidian_amount < required_obsidian:
            obsidian_amount += obsidian_rate
            obsidian_rate += 1
            extra_time += 1
        # And 1 step for the geode robot to be built
        extra_time += 1
        return self.time + extra_time

    # Very rough upper bound on whether or not we'll have a given number of geodes at a given time.
    def possible_to_have_this_many_geodes_at_this_time(self, blueprints, geode_amount, time):
        if self.geode_at_time(time) >= geode_amount:
            # We'll already have enough, without building more geode robots.
            return True

        t1 = self.lower_bound_for_time_until_next_geode_robot(blueprints)
        geode_count = self.geode_at_time(t1)

        # Let's assume that after this point, I can make geode robots as fast as I can.
        geode_robot_count = self.mineral_rates[GEODE_INDEX] + 1
        for _ in range(t1+1, time+1):
            geode_count += geode_robot_count
            geode_robot_count += 1
        if geode_count >= geode_amount:
            return True
        else:
            return False

    def geode_at_time(self, t):
        assert t >= self.time
        return self.minerals[GEODE_INDEX] + self.mineral_rates[GEODE_INDEX] * (t - self.time)

    def __repr__(self):
        return "{}, {}, {}".format(self.time, self.minerals, self.mineral_rates)

def solve(blueprints, total_time):
    initial_state = State(0, [0, 0, 0, 0], [1, 0, 0, 0])
    states = [[initial_state]]

    # Step 1: Find a lower bound for the answer.
    lower_bound = 0

    def find_lower_bound_with_choice(f):
        nonlocal lower_bound

        state = initial_state
        while True:
            next_states = [s for s in state.possible_next_states(blueprints) if s.time <= total_time]
            if next_states == []:
                lower_bound = max(lower_bound, state.geode_at_time(total_time))
                break
            else: 
                state = f(next_states)
    find_lower_bound_with_choice(lambda l : l[-1])
    for _ in range(200):
        find_lower_bound_with_choice(random.choice)

    # Step 2: Now search all possible paths - I'm indexing these by "what's the next move I could take at each stage". 
    # There are a couple of optimisations I'm doing:
    # (1) Only try constructing a robot if I haven't already max'd out how many robots of that type I need. I.e., if 
    # the most expensive recipe that needs ore only needs 4 ore, there's no point building more than 4 ore robots.
    # (2) Only consider states which could end up with at least [lower_bound] number of geodes.
    #
    # (2) is done pretty crudely - roughly, assume we can build obsidian robots as fast as we can to get enough
    # obsidian, and then assume we can build geode robots as fast as we can.

    states = [initial_state]
    while states:
        state = states.pop()
        if not state.possible_to_have_this_many_geodes_at_this_time(blueprints, lower_bound, total_time):
            # Recheck the lower bound (it might have changed since we inserted this one) - don't process it if there's no hope.
            continue

        next_states = state.possible_next_states(blueprints)
        next_states = [s for s in next_states if s.time <= total_time]
        next_states = [s for s in next_states if s.possible_to_have_this_many_geodes_at_this_time(blueprints, lower_bound, total_time)]

        for state in next_states:
            lower_bound = max(lower_bound, state.geode_at_time(total_time))
            states.append(state)

    return lower_bound

s = open("input_real").read().strip().split("\n")
blueprints = []

for i, line in enumerate(s):
    ore_cost = int(re.findall("Each ore robot costs ([0-9]*) ore", line)[0])
    clay_cost = int(re.findall("Each clay robot costs ([0-9]*) ore", line)[0])
    obsidian_cost = [int(x) for x in re.findall("Each obsidian robot costs ([0-9]*) ore and ([0-9]*) clay.", line)[0]]
    geode_cost = [int(x) for x in re.findall("Each geode robot costs ([0-9]*) ore and ([0-9]*) obsidian.", line)[0]]

    blueprint = Blueprints([[ore_cost, 0, 0, 0], [clay_cost, 0, 0, 0], [obsidian_cost[0], obsidian_cost[1], 0, 0], [geode_cost[0], 0, geode_cost[1], 0]])
    blueprints.append(blueprint)

# Part 1
TIME = 24
total_score = 0
for i, b in enumerate(blueprints):
    blueprint_number = i + 1
    best = solve(b, TIME)
    total_score += (blueprint_number * best)
    print("For part 1, best for blueprint {} is {}".format(blueprint_number, best))

print("Total score", total_score)

# Part 2
TIME = 32
blueprints = blueprints[:3] # the elephants ate all but the first 3 blueprints
total_score = 1
for i, blueprint in enumerate(blueprints):
    blueprint_number = i + 1
    best = solve(blueprint, 32)
    total_score *= best
    print("For part 2, best for blueprint {} is {}".format(blueprint_number, best))
print("Total score", total_score)