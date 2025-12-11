from functools import cache

example_input = """
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"""

def parse_input(input) -> dict[str, list[str]]:
  g = {}
  for line in input.strip().split("\n"):
    v, ulist = line.split(": ")
    g[v] = ulist.split(" ")
  return g

def bfs(g: dict[str, list[str]]) -> int:
  q = []
  q.append(["you", 0])
  paths = []
  while q:
    v, d = q.pop(0)
    if v in q:
      continue
    if v == "out":
      paths.append(d)
      continue
    for u in g[v]:
      q.append([u, d + 1])
  return len(paths)

def dfs(g: dict[str, list[str]]):
  @cache
  def dfs_util(v: str, dac: bool, fft: bool, end: str) -> int:
    if v == end:
      if dac and fft:
        return 1
      return 0
    if v == "dac":
      dac = True
    if v == "fft":
      fft = True
    finished = 0
    for u in g[v]:
      finished += dfs_util(u, dac, fft, end)
    return finished

  return dfs_util("svr", False, False, "out")

def part1(input):
  return bfs(input)

def part2(input):
  return dfs(input)

input = open("input/day11.txt", "r").read()
# input = example_input.strip()
print("PART 1 =", part1(parse_input(input)))
print("PART 2 =", part2(parse_input(input)))