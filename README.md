# World

## Resources

- arena size
  - x
  - y
- background color
- clicked location Maybe (x, y)
- entity size
- entity mesh

## Systems

- Draw the entities
- Draw the arena
- Move humans randomly
- Move humans towards each other
- Move humans away from each other
- Move humans towards the average velocity of nearby humans
- Stop humans at the edge of the arena
- Create zombie
- move zombie towards nearest human
- handle zombies biting humans
- handle dead humans
- move humans away from zombies

## Entities

### Human

- location(x, y)
- velocity(x, y)
- accleration(x, y)
- sight range
- speed
- color

### Dead Human

- location(x, y)
- color
- decomposition time

### Zombie

- location(x, y)
- velocity(x, y)
- accleration(x, y)
- speed
- color

# Stories

- [ ] Setup
  - [ ] We can see the arena
- [ ] Humans
  - [ ] A human is in the arena
  - [ ] Humans move randomly around the arena
  - [ ] Humans are attracted to each other
  - [ ] Humans are repelled from each other
  - [ ] Humans are attracted to the average velocity of the group
  - [ ] Humans stop at the edge of the arena
- [ ] Zombie
  - [ ] A zombie can be created with a mouse click
  - [ ] A zombie will move towards the nearest human unless there are no more humans
  - [ ] Zombies move slower than humans
  - [ ] When zombies touch a human, the human dies
  - [ ] After 10 seconds, the dead human disapears
  - [ ] When the dead human is removed, there is a chance that a zombie will appear where the human was
  - [ ] Humans move away from zombies that they can see

# Crates

- rand
- eyre
