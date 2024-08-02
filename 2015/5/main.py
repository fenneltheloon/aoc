from sys import stdin

total = 0

for line in stdin:
    length = len(line)-1
    d = False
    s = False
    for i in range(length-2):
        s1 = line[:i]
        s2 = line[i:i+2]
        s3 = line[i+2:]
        if s2 in s1 or s2 in s3:
            d = True
        if line[i] == line[i+2]:
            s = True
    if d and s:
        total += 1

print(total)
