# Input:
example_input = """
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"""


def parse_input(input: str) -> list[list[str]]:
    lines = input.strip().split("\n")
    return [list(line) for line in lines]


def neighbors(x: int, y: int, grid: list[list[str]]) -> list[tuple[int, int]]:
    deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    result = []
    for dx, dy in deltas:
        nx, ny = x + dx, y + dy
        if 0 <= nx < len(grid[0]) and 0 <= ny < len(grid):
            result.append((nx, ny))
    return result


def count_rolls(x: int, y: int, grid: list[list[str]]) -> int:
    count = 0
    for nx, ny in neighbors(x, y, grid):
        if grid[ny][nx] == "@":
            count += 1
    return count


def count_total_rolls(grid: list[list[str]]) -> int:
    total = 0
    found = True
    while found == True:
        found = False
        for nx in range(len(grid)):
            for ny in range(0, len(grid[0])):
                if grid[ny][nx] == "@" and count_rolls(nx, ny, grid) < 4:
                    found = True
                    total += 1
                    grid[ny][nx] = "."
    return total


input = open("input/day04.txt", "r").read()
# input = example_input.strip()

m = parse_input(input)
print(count_total_rolls(m))
