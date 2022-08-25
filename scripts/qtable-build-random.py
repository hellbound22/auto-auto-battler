import common
import state
import action

if __name__ == "__main__":
    print("Parsing pet table...")
    pets = []

    with open("./packs/std.pets", "r") as fp:
        lines = fp.readlines()

        for l in lines:
            pet = common.Pet(l)
            pets.append(pet)
            
    print("Done!")

    print("Generating state table...")
    state.gen(pets[slice(10)])
    print("\nDone!")

    print("Generating action table...")
    action.gen()
    print("Done!")

    print("Generating random Q-table...")