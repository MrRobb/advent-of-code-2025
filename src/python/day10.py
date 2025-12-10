import z3
from collections import deque

example_input = """
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"""

def parse_input(input) -> list[list[str], list[list[int]], list[int]]:
  res = []
  for line in input.strip().split("\n"):
    indicator_light_diagram = [ char == '#' for char in line.split(" ")[0]][1:-1]
    button_wiring_schematics = [ list(map(int, button[1:-1].split(","))) for button in line.split(" ")[1:-1]]
    joltage_requirements = list(map(int, line.split(" ")[-1][1:-1].split(",")))
    res.append((indicator_light_diagram, button_wiring_schematics, joltage_requirements))
  return res

def solve_with_indicator_lights(machine):
  indicator_light_diagram, button_wiring_schematics, joltage_requirements = machine

  solver = z3.Optimize()
  presses = [ z3.Int(f"button_{i}") for i in range(len(button_wiring_schematics)) ]

  # Button presses are positive integers
  for i in range(len(button_wiring_schematics)):
    solver.add(presses[i] >= 0)

  # The indicator light is on if the sum of the number of presses for each button that affects the indicator light is odd
  for i in range(len(indicator_light_diagram)):
    terms = []
    for j in range(len(button_wiring_schematics)):
      if i in button_wiring_schematics[j]:
        # if the button affects the indicator light, add the number of presses to the terms
        terms.append(presses[j])
    equation = z3.Sum(terms) % 2 == int(indicator_light_diagram[i])
    solver.add(equation)

  # Minimize the total number of button presses
  solver.minimize(z3.Sum(presses))

  if solver.check() == z3.sat:
    return solver.model().eval(z3.Sum(presses)).as_long()
  return -1

def part1(input):
  total_presses = 0
  for machine in input:
    presses = solve_with_indicator_lights(machine)
    total_presses += presses
  return total_presses

def solve_with_joltage(machine):
  indicator_light_diagram, button_wiring_schematics, joltage_requirements = machine

  solver = z3.Optimize()
  presses = [ z3.Int(f"button_{i}") for i in range(len(button_wiring_schematics)) ]
  
  # Button presses are positive integers
  for i in range(len(button_wiring_schematics)):
    solver.add(presses[i] >= 0)

  # The sum of the number of presses for each button that affects the joltage requirement must equal the joltage requirement
  for i in range(len(joltage_requirements)):
    terms = []
    for j in range(len(button_wiring_schematics)):
      if i in button_wiring_schematics[j]:
        # if the button affects the joltage requirement, add the number of presses to the terms
        terms.append(presses[j])
    equation = z3.Sum(terms) == joltage_requirements[i]
    solver.add(equation)

  # Minimize the total number of button presses
  solver.minimize(z3.Sum(presses))

  if solver.check() == z3.sat:
    return solver.model().eval(z3.Sum(presses)).as_long()
  return -1

def part2(input):
  total_presses = 0
  for machine in input:
    presses = solve_with_joltage(machine)
    total_presses += presses
  return total_presses


input = open("input/day10.txt", "r").read()
# input = example_input.strip()
print("PART 1 =", part1(parse_input(input)))
print("PART 2 =", part2(parse_input(input)))