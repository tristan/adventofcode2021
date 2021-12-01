from typing import List

def window(arr: List[int], k: int):
    for i in range(len(arr) - k + 1):
        yield arr[i:i+k]

def part1(input: List[int]) -> int:
    result = 0
    for (a, b) in window(input, 2):
        if a < b:
            result += 1
    return result

def part1(input: List[int]) -> int:
    result = 0
    for (a, b) in window(input, 2):
        if a < b:
            result += 1
    return result

def part2(input: List[int]) -> int:
    return part1([
        sum(values) for values in window(input, 3)
    ])

if __name__ == "__main__":
    with open("day_01_input.txt") as f:
        input = [
            int(line)
            for line in
            f.readlines()
            if line != ""
        ]
    print("part1:", part1(input))
    print("part2:", part2(input))
