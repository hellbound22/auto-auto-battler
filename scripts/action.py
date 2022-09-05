import itertools

def gen(): 
    with open("./qtables/std/action.table", "w") as fp:
        stuff = list(range(0, 8)) + [99]
        output = list(itertools.product(stuff, stuff, stuff))

        for l in output:
            s = str(l[0]) + ',' + str(l[1]) + ',' + str(l[2])+ '\n'
            fp.write(s)