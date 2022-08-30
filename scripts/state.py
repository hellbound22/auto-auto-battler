from ast import If
import common;
import itertools;

def gen(pets):
    State(pets, 5)

class State:
    def __init__(self, bucket, size):
        with open("./qtables/std/state.table", "w") as file_desc:
            with open("./qtables/std/meta_state.table", "w") as meta_file:
                comb = list(itertools.product(bucket, repeat=size))

                #l = len(comb) * len(comb) * 10 * 6
                l = len(comb)
                u_index = 0

                #for gold in range(10): #WARN: More values of gold are possible
                    #for shop_tier in range(6): 
                
                last_id = 0

                for x in comb:
                    id = x[0].id
                    if id != last_id:
                        meta_file.write(str(last_id) + "," + str(u_index) + "\n")
                        print(last_id)
                        last_id = id

                    
                    #for y in comb:
                    u_index += 1

                    #file_desc.write(str(gold))
                    #file_desc.write(";")
                    #file_desc.write(str(shop_tier))
                    #file_desc.write(";")
                    file_desc.write(State.lst_str(x))
                    #file_desc.write(";")
                    #file_desc.write(State.lst_str(y))
                    file_desc.write("\n")

                    #print('Writing to file: ' + str(u_index) + " of " + str(l) + " - " + str(round((u_index/l)*100, 2)) + '%  \r', end="")


    def lst_str(comb):
        s = ""

        for p in comb:
            s += p.qtable_str()
            s += '|'

        return s
