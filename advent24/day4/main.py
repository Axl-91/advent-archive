import sys


def countXmas(lines, i, j):
    directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]
    xmasFound = 0

    for dirI, dirJ in directions:
        try:
            if (
                i + 3 * dirI >= 0
                and j + 3 * dirJ >= 0
                and lines[i + dirI][j + dirJ] == "M"
                and lines[i + 2 * dirI][j + 2 * dirJ] == "A"
                and lines[i + 3 * dirI][j + 3 * dirJ] == "S"
            ):
                xmasFound += 1
        except IndexError:
            pass

    return xmasFound


def countCrossMas(lines, i, j):
    if i + 2 >= len(lines) or j + 2 >= len(lines[i]):
        return 0

    if lines[i][j] == "M" and lines[i + 1][j + 1] == "A" and lines[i + 2][j + 2] == "S":
        if lines[i + 2][j] == "S" and lines[i][j + 2] == "M":
            return 1
        if lines[i + 2][j] == "M" and lines[i][j + 2] == "S":
            return 1

    if lines[i][j] == "S" and lines[i + 1][j + 1] == "A" and lines[i + 2][j + 2] == "M":
        if lines[i + 2][j] == "S" and lines[i][j + 2] == "M":
            return 1
        if lines[i + 2][j] == "M" and lines[i][j + 2] == "S":
            return 1

    return 0


def part1(lines):
    total = 0
    len_line = len(lines[0])

    for i in range(len(lines)):
        for j in range(len_line):
            if lines[i][j] == "X":
                total += countXmas(lines, i, j)

    print("Part 1 Total:", total)


def part2(lines):
    total = 0
    len_line = len(lines[0])

    for i in range(len(lines)):
        for j in range(len_line):
            total += countCrossMas(lines, i, j)

    print("Part 2 Total:", total)


def main():
    lines = []

    with open("input", "r") as inputFile:
        for line in inputFile:
            lines.append(line.strip())

    part1(lines)
    part2(lines)

    return 0


if __name__ == "__main__":
    sys.exit(main())
