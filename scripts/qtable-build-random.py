import itertools

class QShop:
    def __init__(self, bucket, file_desc):
        self.sets = []
        set_len = len(bucket)+1
        comb_len = 0
        first = True
        u_index = 0


        def powerset(iterable):
            "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
            s = list(iterable)
            return itertools.chain.from_iterable(itertools.combinations(s, r) for r in range(6))

        comb = powerset(bucket)
        for x, subset in enumerate(comb):
            file_desc.write(QShop.qtable_lst_str(subset))
            print('Combination: ' + str(x) + '\r', end="")

    def qtable_lst_str(comb):
        c = comb
        s = ""
        for p in c:
            s += p.qtable_str()
            s += '|'
        
        s += '\n'
        return s



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

pets = []

with open("./packs/std.pets", "r") as fp:
    lines = fp.readlines()

    for l in lines:
        pet = Pet(l)
        pets.append(pet)


with open("./qtables/std.pets", "w") as fp:
    print("Writing to file...")
    QShop(pets, fp)
    pass