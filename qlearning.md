# Q-learing

## Initial test
It only decides what to buy
state = crew + gold + shop
reinforcement = sum of all pet stats  > This will have to change once we add more food and abilities
action = buy pet and at which slot

### State
gold;shop_tier;shop_pets(separated by |);crew_pets(separated by |)

- exanple
  - 10;2;3,1,2,1,0|36,1,2,1,0|45,5,8,1,0;3,1,2,1,0|36,1,2,1,0|45,5,8,1,0|37,5,3,1,0|38,3,5,1,0

##### Pet
- Pet example
id,power,health,level,xp > include tiers later
1,2,1,1,0

### Action
buy pet X(store slot) to position Y(crew slot)
- example
1,4

### Reinforcement 
sum of pets power and health / no of turns passed

### Q-table
line where its number is the line number of the state table, and each number is the Q-value of an action. The index of this number is the line number of the action index
- example
n,n,n,n...

### Tables
- state
- action
- q-table

### Order
lookup state -> lookup q-table(generated and updated by rust) -> lookup action

loop this until money is zero

new order (uses the same table for states; optimized?):
lookup shop state -> lookup crew state -> lookup q-table(generated and updated by rust) -> lookup action

#### File sizes
state with 10 pets (g/st) = 1,7mb
state with all pets (g/st) = 16,2gb (est)
state with all pets (g) = 2,7gb (est)
state with all pets = 274mb
