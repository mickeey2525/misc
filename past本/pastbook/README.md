# Atcoderの標準入力の受けとり方法

|入力の内容|入力を受け取る方法|変数の中身|
|---|---|---|
|abc|a = input()|a: "abc"(文字列)|
|100|a=int(input())|a: 100(整数)|
|5  abcde|a=int(input)  b=input()| a = 5(整数)  b ="abcde"(文字列)|
|5  12345|n = int(input())  a =list(map(int, input.split))| n:5(整数)  a:[1,2,3,4,5]|
|100 10|n, m = list(map(int, input().split()))|n: 100  m:10|