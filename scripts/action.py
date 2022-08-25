import itertools

def gen(): 
    with open("./qtables/std/action.table", "w") as fp:
        stuff = range(1, 6)
        output = list(itertools.product(stuff, stuff))

        for l in output:
            s = str(l[0]) + ',' + str(l[1]) + '\n'
            fp.write(s)