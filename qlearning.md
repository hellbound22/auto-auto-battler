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

