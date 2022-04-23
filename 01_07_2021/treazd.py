import numpy as np
def f(x):
    return 2 * x * x * np.cos(x) - 5 * x

x = [-1.]
f(x[0])

def df(x):
    return 4 * x * np.cos(x) - 2 * x * x * np.sin(x) - 5

# slope = df(x[0])
# slope
# # -5.47827

alpha = 0.05

# x.append(x[0] - alpha * slope)
# x[1]

# x.append(x[1] - alpha * df(x[1]))
# x[2]

x = [-1.]
for i in range(20):
    x.append(x[i] - alpha * df(x[i]))
x