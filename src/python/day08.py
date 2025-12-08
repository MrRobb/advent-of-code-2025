import math

example_input = """
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"""

def parse_input(input):
    return [list(map(int, line.split(","))) for line in input.strip().split("\n")]

def count_connected_components(graph) -> int:
    visited = set()
    def dfs(node):
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                dfs(neighbor)

    count = []
    for i in range(len(graph)):
        if i not in visited:
            visited_before_size = len(visited)
            visited.add(i)
            dfs(i)
            visited_after_size = len(visited)
            count.append(visited_after_size - visited_before_size)
    return math.prod(sorted(count)[-3:])

def calculate_joined_graph(graph, max_joints) -> tuple[dict[int, list[int]], tuple[int, int]]:
  distances = {}
  for i in range(len(graph)):
      for j in range(i + 1, len(graph)):
          distances[(i, j)] = math.sqrt((graph[i][0] - graph[j][0]) ** 2 + (graph[i][1] - graph[j][1]) ** 2 + (graph[i][2] - graph[j][2]) ** 2)
  distances = sorted(distances.items(), key=lambda x: x[1])

  joined_graph = [ [] for _ in range(len(graph))]
  joints = 0
  in_graph = set()
  last_joints = None
  for (i, j), d in distances:
      if joints >= max_joints:
          break
      if i not in in_graph or j not in in_graph:
          last_joints = (i, j)
      in_graph.add(i)
      in_graph.add(j)
      joined_graph[i].append(j)
      joined_graph[j].append(i)
      joints += 1
  return joined_graph, last_joints


def part1(input):
    joined_graph, last_joints = calculate_joined_graph(input, max_joints=1000)
    return count_connected_components(joined_graph)

def part2(input):
    joined_graph, last_joints = calculate_joined_graph(input, max_joints=1000000)
    return input[last_joints[0]][0] * input[last_joints[1]][0]


input = open("input/day08.txt", "r").read()
# input = example_input.strip()
print("PART 1 =", part1(parse_input(input)))
print("PART 2 =", part2(parse_input(input)))