grid = {}
input = input()

x = 0
y = 0

for char in input:
    try:
        grid[(x, y)] += 1
    except KeyError:
        grid[(x, y)] = 0
    match char:
        case 'v':
            y -= 1
        case '^':
            y += 1
        case '>':
            x += 1
        case '<':
            x -= 1


print(len(grid))
