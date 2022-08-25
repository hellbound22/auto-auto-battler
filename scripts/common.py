import itertools

def powerset(iterable, size):
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return itertools.chain.from_iterable(itertools.combinations(s, r) for r in range(size+1))


class Pet:
    def __init__(self, line):
        s = line.split(',')
        self.id = s[0]
        self.power = s[3]
        self.health = s[4]
        self.level = 1
        self.xp = 0
        pass

    def qtable_str(self):
        return "{},{},{},{},{}".format(self.id, self.power, self.health, self.level, self.xp)