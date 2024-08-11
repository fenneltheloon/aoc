# Plan

Define a rectangle object with 2 (x,y) pairs and a boolean state (on, off)

Okay so the core of this is a function that takes an array of rectangles and a
new rectangle and returns a new array of rectangles with the new rectangle 
applied

Iterate over squares in array, check for which case matches

17 separate cases:

XOO     OXO     OOX     OOO     OOO     OOO     OOO     OOO     OOO     OOO     XXX
OOO     OOO     OOO     XOO     OXO     OOX     OOO     OOO     OOO     OOO     XXX
OOO     OOO     OOO     OOO     OOO     OOO     XOO     OXO     OOX     OOO     XXX
 10      8       9       2       0       1       6       4       5       E       15

XXX  OOO     OOO     XOO     OXO     OOX
OOO XXX     OOO     XOO     OXO     OOX
OOO     OOO     XXX     XOO     OXO     OOX
 11      3       7       14      12      13

Original square (x1,y1), (x2y2), new square (p1,q1), (p2,q2)

Do this first
10 is y1 >= q2 ||
q1 >= y2 ||
x1 >= p2 ||
p1 >= x2

y1 >= q1
q2 >= y2
x1 >= p1
p2 >= x2


Ok check for case E first

THEN
Make each check sequentially, get a bit value, bitshift
0000: 5
0001: 6
0010: 4
0011: 13
0100: 8
0101: 9
0110: 7
0111: 14
1000: 2
1001: 3
1010: 1
1011: 12
1100: 16
1101: 17
1110: 15
1111: 11

```python
if (y1 >= q2 || q1 >= y2 || x1 >= p2 || p1 >= x2):
    add [(x1, y1), (x2, y2), old]

value = 0
for item in [(y1 >= q1), (q2 >= y2), (x1 >= p1), (p2 >= x2)]:
    value << 1
    value | item

switch value:
    0: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p1-1, q2), old]
        [(p1, q1), (p2, q2), new]
        [(p2+1, q1), (x2, q2), old]
        [(x1, q2+1), (x2, y2), old]
    1: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p1-1, q2), old]
        [(p1, q1), (x2, q2), new]
        [(x1, q2+1), (x2, y2), old]
    2: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p2, q2), new]
        [(p2+1, q1), (x2, q2), old]
        [(x1, q2+1), (x2, y2), old]
    3: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (x2, q2), new]
        [(x1, q2+1), (x2, y2), old]
    4: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p1-1, y2), old]
        [(p1, q1), (p2, y2), new]
        [(p2+1, q1), (x2, y2), old]
    5: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p1-1, y2), old]
        [(p1, q1), (x2, y2), new]
    6: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (p2, y2), new]
        [(p2+1, q1), (x2, y2), old]
    7: add [(x1, y1), (x2, q1-1), old]
        [(x1, q1), (x2, y2), new]
    8: add [(x1, y1), (p1-1, q2), old]
        [(p1, y1), (p2, q2), new]
        [(p2+1, y1), (x2, q2), old]
        [(x1, q2+1), (x2, y2), old]
    9: add [(x1, y1), (p1-1, q2), old]
        [(p1, y1), (x2, q2), new]
        [(x1, q2+1), (x2, y2), old]
    10: add [(x1, y1), (p2, q2), new]
          [(p2+1, y1), (x2, q2), old]
          [(x1, q2+1), (x2, y2), old]
    11: add [(x1, y1), (x2, q2), new]
          [(x1, q2+1), (x2, y2), old]
    12: add [(x1, y1), (p1-1, y2), old]
          [(p1, y1), (p2, y2), new]
          [(p2+1, y1), (x2, y2), old]
    13: add [(x1, y1), (p1-1, y2), old]
          [(p1, y1), (x2, y2), new]
    14: add [(x1, y1), (p2, y2), new]
          [(p2+1, y1), (x2, y2), old]
    15: add [(x1, y1), (x2, y2), new]
```




Need to accommodate case 10
```python
# (Top y layer)
if y1 >= q1:
    # (11, 15, 16, 17)
    if q2 >= y2:
        # (11, 15)
        if x1 >= p1:
            if p2 >= x2: 11
            elif p2 >= x1: 15
            else: 10
        # (16, 17)
        elif p2 >= x2: 17
        elif p2 >= x1: 16
        else: 10
    # (12, 1, 2, 3)
    elif x1 >= p1:
        if p2 >= x2: 12
        elif p2 >= x1: 10
        else: 1
    elif p2 >= x2: 3
    elif p2 >= x1: 1
    else: 2
# (Bottom y layer) (7, 8, 9, 14)
elif q2 >= y2:
    # (7, 14)
    if x1 >= p1:
        if p2 >= x2: 14
        elif p2 >= x1: 10
        else: 7
    elif p2 >= x2: 9
    elif p2 >= x1: 10
    else: 8
# Middle y layer (4, 5, 6, 13)
elif x1 >= p1:
    if p2 >= x2: 13
    elif p2 >= x1: 10
    else: 4
elif p2 >= x2: 6
elif p2 >= x1: 10
else: 5

```


