# https://atcoder.jp/contests/past202004-open/tasks/past202004_b

S = input()

na = S.count("a")
nb = S.count("b")
nc = S.count("c")

mx = max(na, nb, nc)

if mx == na:
    print("a")
elif mx == nb:
    print("b")
else:
    print("c")
