# Heuristics

### Performance Upgrades
- [ ] You can't buy pets on top of other friends that dosent have the same id
  - Already returns an error 
- [x] Action (1, _, _) (Buy pet) needs 2nd and 3rd options to be 1..6
- [ ] Action (2, _, _) (Swap pets) needs 2nd and 3rd options to be 1..6
  - [x] This action can be disabled for the time being since order dosent matter for the rewards (hard changed)
- [x] Action (6, _, _) (Buy food) can be disabled for the time being
- [x] Action (3, _, _) (Roll shop) has only one outcome. Remove other possibilities other than (3, 0, 0)
- [x] Action (99, _, _) (End Turn) has only one outcome. Remove other possibilities other than (99, 0, 0)
- [x] Action (4, x, _) (Sell pet) dosent need the third choice
- [x] Action (5, x, _) (Freeze pet) dosent need the third choice
- [x] Action (7, x, _) (Freeze food) dosent need the third choice
- [x] Action (0, _, _) (Do nothing) only needs one entry in the action (or none, who knows?)
- [x] Any actions that are not strictly (99, 0, 0) but cointain 99 are useless


### Efficiency 
- [ ] Rolling with money unspent is unwise (-q points)