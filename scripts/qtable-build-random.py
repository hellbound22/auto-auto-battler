import common
import state
import qtable

pets = []

with open("./packs/std.pets", "r") as fp:
    lines = fp.readlines()

    for l in lines[slice(10)]:
        pet = common.Pet(l)
        pets.append(pet)


with open("./qtables/std.pets", "w") as fp:
    print("Generating Q-Table...")
    state.State(pets, fp, 5)
    pass