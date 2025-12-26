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


def main():
    total = 0
    lines = []
    with open("input", "r") as inputFile:
        for line in inputFile:
            lines.append(line.strip())

    len_line = len(lines[0])
    for i in range(len(lines)):
        for j in range(len_line):
            if lines[i][j] == "X":
                total += countXmas(lines, i, j)

    print(total)
    return 0


if __name__ == "__main__":
    sys.exit(main())
