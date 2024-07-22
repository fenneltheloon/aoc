def step(op, char, grid):
    match char:
        case 'v':
            op[1] -= 1
        case '^':
            op[1] += 1
        case '>':
            op[0] += 1
        case '<':
            op[0] -= 1
    if op not in grid:
        grid.append(op.copy())
    return op, grid


grid = [[0, 0]]
input = input()

s = [0, 0]
rs = [0, 0]
is_rs = False

for char in input:
    if is_rs:
        (rs, grid) = step(rs, char, grid)
        is_rs = False
    else:
        (s, grid) = step(s, char, grid)
        is_rs = True

print(len(grid))
