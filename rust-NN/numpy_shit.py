import numpy as np

arr = np.array([1, 2, 3, 4, 5])

print(arr)

print(type(arr))

lol = np.array([[0.20051619, 0.98752763, 0.98651776, 0.99995671],
 [0.72125283, 0.99923698, 0.07252309, 0.97535483],
 [0.45727148, 0.07492859, 0.99738627, 0.97346394],
 [0.88939756, 0.94276406, 0.95781473, 0.97895072]])

ok = np.array([[0.00039417],
 [-0.00257231],
 [-0.00242852],
 [ 0.0039528 ]])
print()
print(lol)
print(ok)
wtf = lol*ok
print()
print(wtf)
print()
print(np.dot(lol,ok))

# print(wtf.size)
# print(wtf.shape)
# sums = []
# for i in range(wtf.shape[0]):
#     print(wtf[i])
#     sum = 0
#     for e in wtf[i]:
#         sum += e
#     sums.append(sum)
    
# print(sums)

aze = np.array([[0.20051619, 0.98752763, 0.98651776, 0.99995671]])
zae = np.array([[ 0.00039417],
 [-0.00257231],
 [-0.00242852],
 [ 0.0039528 ]])
print()
print(aze*zae)
print(zae*aze)
print(np.dot(aze,zae))
print(0.20051619*0.00039417 + 0.98752763*(-0.00257231) + 0.98651776*(-0.00242852) + 0.99995671*0.0039528)