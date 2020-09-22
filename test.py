from turtle import *
from PIL import Image

ht()

b = int(input())

delta = b//2
screensize(4 * delta, 4 * delta)
delay(0)

n, k = list(map(int, input().split()))
P = n * [None]
for i in range(n):
    P[i] = list(map(int, input().split()))
    P[i][1] -= 3 * delta

L = k*[None]
for i in range(k):
    L[i] = list(map(int, input().split()))
    L[i][1] -= 3 * delta
    L[i][3] -= 3 * delta


up()

for p in P:
    setpos(p[0], p[1])
    down()
    dot((2*delta)//100, 'red')
    up()


color('black')
for l in L:
    setpos(l[0], l[1])
    down()
    setpos(l[2], l[3])
    up()


getscreen().getcanvas().postscript(
    file='out.eps', height=4 * delta, width=4 * delta)

img = Image.open('out.eps')
img.save('out.png', 'png')
