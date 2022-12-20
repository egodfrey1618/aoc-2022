from typing import List

# Keep track of the circle as a doubly-linked-list
# It's still O(n) to mix (you've got to step forward enough spaces to see where to move to), but
# it makes it a little easier to keep track of the original order.
class Node(object):
    def __init__(self, n, length, prev, next):
        self.n = n
        self.length = length
        self.prev = prev
        self.next = next
    
    def mix_single(self):
        k = self.n % (self.length - 1) 
        if k > 0:
            target = self.next # Target is the node we want to sit before this one, after it's moved.
            self.prev.next = self.next
            self.next.prev = self.prev
            while k > 1:
                target = target.next
                k -= 1
            self.next = target.next
            self.prev = target
            target.next = self
            self.next.prev = self
        elif k < 0:
            target = self.prev # Target is the node we want to sit after this one, after it's moved.
            self.next.prev = self.prev
            self.prev.next = self.next
            while k < -1:
                target = target.prev
                k += 1
            self.prev = target.prev
            self.next = target
            target.prev = self
            self.prev.next = self
        else:
            pass

    def to_node_list(self):
        nodes = [self]
        while nodes[-1].next != self:
            nodes.append(nodes[-1].next)
        return nodes

    def __repr__(self):
        nodes = self.to_node_list()
        return " -> ".join(str(node.n) for node in nodes)

    def to_int_list(self):
        nodes = self.to_node_list()
        return [x.n for x in nodes]

def node_of_int_list(l : List[int]) -> List[Node]:
    result = [Node(n, len(l), None, None) for n in l]
    for i in range(len(result)):
        result[i].prev = result[i-1]
        result[i].next = result[(i+1) % len(result)]
    return result

def score(nodes):
    zero_node = [node for node in nodes if node.n == 0][0]
    values = zero_node.to_int_list()
    score = sum([values[c % len(values)] for c in [1000, 2000, 3000]])
    print(score)

l = [int(x) for x in open("input").read().strip().split("\n")]
# l = [1, 2, -3, 3, -2, 0, 4]

# Part 1
nodes = node_of_int_list(l)

for n in nodes:
    n.mix_single()
score(nodes)

# Part 2
DECRYPTION_KEY = 811589153 
ROUNDS = 10
nodes = node_of_int_list([x * DECRYPTION_KEY for x in l])

for i in range(ROUNDS):
    print("Applying round {}/{}".format(i+1, ROUNDS))
    for n in nodes:
        n.mix_single()
score(nodes)