example_input = """
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"""

class Present:
  def __init__(self, shape):
    self.shape = shape

  def __repr__(self):
    return "\n" + "\n".join(["".join(row) for row in self.shape]) + "\n"

  def width(self):
    return max([len(row) for row in self.shape])

  def height(self):
    return len(self.shape)

  def cells(self) -> int:
    return sum([1 for row in self.shape for cell in row if cell == '#'])


class Region:
  def __init__(self, width, height, presents):
    self.width = width
    self.height = height
    self.presents = presents

  def __repr__(self):
    return f"{self.width}x{self.height}: {self.presents}"


def parse_input(input):
  presents = input.split("\n\n")
  regions = presents.pop()
  present_list = []
  region_list = []

  for present in presents:
    lines = present.split("\n")
    shape = [list(line) for i, line in enumerate(lines) if i != 0]
    present_list.append(Present(shape))

  for region in regions.split("\n"):
    dimensions, presents = region.split(": ")
    width, height = dimensions.split("x")
    presents_region = [int(p) for p in presents.split(" ")]
    region_list.append(Region(int(width), int(height), presents_region))

  return present_list, region_list
    

def region_fits(region, present_list):
  return region.width * region.height >= sum([p * q.cells() for p, q in zip(region.presents, present_list)])


def part1(input):
  presents, regions = input
  how_many_fit = 0
  for region in regions:
    if region_fits(region, presents):
      how_many_fit += 1
  return how_many_fit
    
def part2(input):
  return "\u2605"
    

input = open("input/day12.txt", "r").read()
# input = example_input.strip()
print("PART 1 =", part1(parse_input(input)))
print("PART 2 =", part2(parse_input(input)))