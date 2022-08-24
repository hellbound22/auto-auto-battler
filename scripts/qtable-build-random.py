import itertools



class QShop:
    def __init__(self, bucket):
        self.sets = []
        for L in range(0, len(bucket)+1):
            for subset in itertools.combinations(bucket, 3):
                self.sets.append(subset)

    def qtable_lst_str(self):
        s = []
        for c in self.sets:
            s.append("{}|{}|{}\n".format(c[0].qtable_str(), c[1].qtable_str(), c[2].qtable_str()))
        
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
        #print(pet.qtable_str())
    
qshop = QShop(pets)
#print(qshop.qtable_lst_str()[0])
print(len(qshop.sets))

with open("./qtables/std.pets", "w") as fp:
    for set in qshop.qtable_lst_str():
        fp.write(set)

    pass