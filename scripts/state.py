import common;

class State:
    def __init__(self, bucket, file_desc, size):
        comb = list(common.powerset(bucket, size))
        print("Done!")
        crew = list(comb)

        l = len(crew) * len(crew) * 10 * 6
        u_index = 0

        for gold in range(10): #WARN: More values of gold are possible
            for shop_tier in range(6): 
                for x in crew:
                    for y in crew:
                        file_desc.write(str(gold))
                        file_desc.write(";")
                        file_desc.write(str(shop_tier))
                        file_desc.write(";")
                        file_desc.write(State.lst_str(x))
                        file_desc.write(";")
                        file_desc.write(State.lst_str(y))
                        file_desc.write(";")

                        print('Writing to file: ' + str(u_index) + " of " + str(l) + " - " + str(round((u_index/l)*100, 2)) + '%  \r', end="")

                        u_index += 1
        
        print("\n")


    def lst_str(comb):
        s = ""

        for p in comb:
            s += p.qtable_str()
            s += '|'

        return s
