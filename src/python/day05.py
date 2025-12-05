# Input:
example_input = """
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"""


def parse_input(input: str) -> tuple[list[tuple[int, int]], list[int]]:
    res = input.strip().split("\n\n")
    fresh_ranges_str, available_ids_str = res[0], res[1]
    fresh_ranges = []
    for line in fresh_ranges_str.split("\n"):
        start, end = map(int, line.split("-"))
        fresh_ranges.append((start, end))
    available_ids = [int(line) for line in available_ids_str.split("\n")]
    return fresh_ranges, available_ids


def how_many_fresh_check(
    fresh_ranges: list[tuple[int, int]], available_ids: list[int]
) -> int:
    fresh_count = 0
    for ingredient_id in available_ids:
        for start, end in fresh_ranges:
            if start <= ingredient_id <= end:
                fresh_count += 1
                break
    return fresh_count


def how_many_fresh_all(fresh_ranges: list[tuple[int, int]]) -> int:
    fresh_ranges.sort()
    res = [fresh_ranges[0]]
    for start, end in fresh_ranges:
        _, last_end = res[-1]
        if last_end >= start - 1:
            last_start, last_end = res.pop()
            res.append((min(last_start, start), max(last_end, end)))
        else:
            res.append((start, end))

    return sum([end - start + 1 for start, end in res])


input = open("input/day05.txt", "r").read()
# input = example_input.strip()

fresh_ranges, available_ids = parse_input(input)
print(how_many_fresh_check(fresh_ranges, available_ids))
print(how_many_fresh_all(fresh_ranges))
