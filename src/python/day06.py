# Input:
example_input = """
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"""


def parse_input(input: str) -> tuple[list[list[int]], list[str]]:
    res = input.strip().split("\n")
    numbers: list[list[int]] = []
    for line in res[:-1]:
        split_line = [s for s in line.split(" ") if s != ""]
        for i, num_str in enumerate(split_line):
            if len(numbers) <= i:
                numbers.append([])
            numbers[i].append(int(num_str))
    operation_line = res[-1]
    operations = [s for s in operation_line.split(" ") if s != ""]
    return numbers, operations


def calculate(numbers, operations) -> int:
    total = 0
    for nums, op in zip(numbers, operations):
        if op == "*":
            res = 1
            for n in nums:
                res *= n
            total += res
        elif op == "+":
            total += sum(nums)
    return total


def parse_input_column(input: str) -> tuple[list[list[int]], list[str]]:
    res = input.strip().split("\n")
    numbers: list[list[int]] = []
    for line in res[:-1]:
        split_line = list(line)
        for i, char in enumerate(split_line):
            if char != " ":
                while len(numbers) <= i:
                    numbers.append([])
                numbers[i].append(int(char))
    joined_numbers: list[list[int]] = [[]]
    for digit_list in numbers:
        if digit_list == []:
            joined_numbers.append([])
            continue
        number = 0
        for digit in digit_list:
            number = number * 10 + digit
        joined_numbers[-1].append(number)
    operation_line = res[-1]
    operations = [s for s in operation_line.split(" ") if s != ""]
    operations = list(operations)
    return joined_numbers, operations


input = open("input/day06.txt", "r").read()
# input = example_input.strip()
numbers, operations = parse_input(input)
print(calculate(numbers, operations))
numbers, operations = parse_input_column(input)
print(calculate(numbers, operations))
