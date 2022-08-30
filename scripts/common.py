import itertools

def powerset(iterable, size):
    list_combinations = list()

    for n in range(size):
        list_combinations += list(itertools.combinations_with_replacement(iterable, n))

    return list_combinations


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
        #return "{}".format(self.id)