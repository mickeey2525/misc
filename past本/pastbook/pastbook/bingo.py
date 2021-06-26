# https://atcoder.jp/contests/abc157/tasks/abc157_b

A = []

for _ in range(0, 3):
    row =list(map(int, input().split()))

    A.append(row)

M = []
for i in range(0,3):
    row = []
    for j in range(0, 3):
        row.append(False)
    M.append(row)

N = int(input())

for _ in range(0,N):
    b = int(input())
    for i in range(0, 3):
        for j in range(0,3):
            if b == A[i][j]:
                M[i][j] = True

bingo = False

for i in range(0,3):
    if M[i][0] and M[i][1] and M[i][2] == True:
        bingo = True

for i in range(0,3):
    if [0][i] and M[1][i] and M[2][i] == True:
        bingo = True

if M[0][0] and M[1][1] and M[2][2] == True:
    bingo = True

if M[2][0] and M[1][1] and M[2][0] == True:
    bingo = True

if bingo == True:
    print("Yes")
else:
    print("No")