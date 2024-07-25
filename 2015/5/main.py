from sys import stdin

total = 0
vowels = ['a', 'e', 'i', 'o', 'u']
forbid = ["ab", "cd", "pq", "xy"]

for line in stdin:
    vc = 0
    dl = False
    ct = True
    prev = ''
    for char in line:
        if char in vowels:
            vc += 1
        if char == prev:
            dl = True
        if prev + char in forbid:
            ct = False
            break
        prev = char
    if (vc > 2) and dl and ct:
        total += 1

print(total)
