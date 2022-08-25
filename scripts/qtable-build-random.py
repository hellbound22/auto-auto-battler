import itertools
import json

class QCrew:
    def __init__(self, bucket, file_desc, size):
        def powerset(iterable, size):
            "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
            s = list(iterable)
            return itertools.chain.from_iterable(itertools.combinations(s, r) for r in range(size+1))

        comb = list(powerset(bucket, size))
        print("Done!")
        crew = list(comb)

        l = len(crew) * len(crew) * 10 * 6
        u_index = 0

        for gold in range(10): #WARN: More values of gold are possible
            for shop_tier in range(6): #WARN: More values of gold are possible
                for x in crew:
                    for y in crew:
                        file_desc.write(str(gold))
                        file_desc.write(";")
                        file_desc.write(str(shop_tier))
                        file_desc.write(";")
                        file_desc.write(QCrew.qtable_lst_str(x))
                        file_desc.write(";")
                        file_desc.write(QCrew.qtable_lst_str(y))
                        file_desc.write("\n")

                        print('Writing to file: ' + str(u_index) + " of " + str(l) + " - " + str(round((u_index/l)*100, 2)) + '%  \r', end="")

                        u_index += 1
        
        print("\n")


    def qtable_lst_str(comb):
        s = ""

        for p in comb:
            s += p.qtable_str()
            s += '|'

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

    for l in lines[slice(10)]:
        pet = Pet(l)
        pets.append(pet)


with open("./qtables/std.pets", "w") as fp:
    print("Generating Q-Table...")
    QCrew(pets, fp, 5)
    pass