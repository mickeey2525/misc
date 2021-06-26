n = int(input())
list = []
for i in range(n):
    list.append(int(input()))
for j in range(n):
    if j < 1:
        continue
    else:
        diff = list[j] - list[j-1]
        z = abs(diff)
        if diff == 0:
            print(f"stay")
        elif diff > 0:
            print(f"up {z}")
        else:
            print(f"down {z}")