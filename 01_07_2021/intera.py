import matplotlib.pyplot as plt
import time

print(0)
plt.pause(1)
print(1)
plt.style.context('dark_background')
plt.pause(1)
print(2)
plt.ion()
plt.pause(1)
print(3)
plt.plot([1.6, 2.7])
plt.pause(1)
print(4)
plt.title("interactive test")
plt.pause(1)
print(5)
plt.xlabel("index")
plt.pause(1)
print(6)
ax = plt.gca()
plt.pause(1)
print(7)
ax.plot([3.1, 2.2])
plt.pause(1)
print(8)
# plt.draw()
plt.pause(1)
print(9)
plt.show(block = True)
print(10)