original_grid = open("input/day07.txt").read().splitlines()
grid = [list(line) for line in original_grid]
splits = 0
beams = [[int(c == "S") for c in grid[0]]]

for i, line in enumerate(grid[1:], start=1):
    new_beams = [0 for _ in line]
    for j, x in enumerate(original_grid[i]):
        if grid[i - 1][j] == "|" and x == ".":
            grid[i][j] = "|"
            new_beams[j] += beams[i - 1][j]
        if grid[i - 1][j] == "S" and x == ".":
            grid[i][j] = "|"
            new_beams[j] += beams[i - 1][j]
        if grid[i - 1][j] == "|" and x == "^":
            splits += 1
            grid[i][j - 1] = "|"
            grid[i][j + 1] = "|"
            new_beams[j - 1] += beams[i - 1][j]
            new_beams[j + 1] += beams[i - 1][j]

    beams.append(new_beams)

print(splits)
print(sum(beams[-1]))
