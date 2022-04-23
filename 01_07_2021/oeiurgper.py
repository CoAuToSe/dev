# 3 Observations
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
 
def mean_squared_error(y, X = []):
    return sum((y-X)**2)/len(X)

# def mean_squared_error(y, X = []):
#     return ((y - X) ** 2).mean()

def gradient_descent(_X, _y, state=0 ,_learningrate=0.06, _epochs=5):
    trace = pd.DataFrame(columns=['a', 'b', 'mse'])
    X = np.array(_X)
    y = np.array(_y)
    a, b = 0.0, 0.0 # Initialisation aléatoire de a et b
    mse = []
    N = len(_X) 
    temp_a = a
    temp_b = b
    temp_mse = mean_squared_error(y, (a*X + b))
    # temp_learningrate = _learningrate

    mse = mean_squared_error(y, (a*X + b))
    trace = trace.append(pd.DataFrame(data=[[a, b, mse]], 
                                      columns=['a', 'b', 'mse'], 
                                      index=['epoch ' + str(0)]))
    for i in range(_epochs):
        delta = y - (a*X + b)
        # delta_square = (y - (a*X + b))**2
        temp_learningrate = _learningrate/((i+1)**0.25)
        temp2a_learningrate = _learningrate*(2*sum(X**2))/N
        temp2b_learningrate = _learningrate*(2)/N
        temp3a_learningrate = _learningrate/((N**2-2*sum(X**2))**(1/2))
        temp3b_learningrate = _learningrate/((N**2-2)**(1/2))
        # print(_learningrate,temp2a_learningrate, temp2b_learningrate, temp3a_learningrate, temp3b_learningrate, temp_learningrate)
        # Updating a and b
        # print(delta,delta_square,X,X.dot(delta).sum(), delta.sum(), N)
        if state == 1:
            a = a -temp3a_learningrate * (-2 * X.dot(delta).sum()) # on retire un gradient à a
            b = b -temp3b_learningrate * (-2 * delta.sum()) # idem pour b
        else:
            if state == 2:
                a = a -_learningrate * (-2 * X.dot(delta).sum() / N) # on retire un gradient à a
                b = b -_learningrate * (-2 * delta.sum() / N) # idem pour b
            else:
                if state == 3:
                    a = a -temp2a_learningrate * (-2 * X.dot(delta).sum() / N) # on retire un gradient à a
                    b = b -temp2b_learningrate * (-2 * delta.sum() / N) # idem pour b
                else:
                    if state ==4:
                        a = a -temp_learningrate * (-2 * X.dot(delta).sum() / N) # on retire un gradient à a
                        b = b -temp_learningrate * (-2 * delta.sum() / N) # idem pour b
                    else:
                        if state == 5:
                            temp = X.copy()
                            temp2 =X.dot(delta).sum()
                            temp3 =X.dot(temp).sum()
                            a = a -_learningrate * ((-2 * temp2 / N)*(1 - (-2 * temp3 /N)) ) # on retire un gradient à a
                            b = b -_learningrate * ((-2 * delta.sum() / N )*(1 - (2/N)) ) # idem pour b
                        else:
                            if state == 6:
                                temp = X.copy()
                                temp2 =X.dot(delta).sum()
                                temp3 =X.dot(temp).sum()
                                a = a -_learningrate * ((-2 * temp2 / N)*(1 + (2 * temp3 /N)) ) # on retire un gradient à a
                                b = b -_learningrate * ((-2 * delta.sum() / N )*(1 + (2/N)) ) # idem pour b
        # a = a -_learningrate * (-2 * X.dot(delta).sum()) # on retire un gradient à a
        # b = b -_learningrate * (-2 * delta.sum()) # idem pour b
        # print(y, (a*X + b),a,X,b)
        mse = mean_squared_error(y, (a*X + b))
 
        trace = trace.append(pd.DataFrame(data=[[a, b, mse]], 
                                          columns=['a', 'b', 'mse'], 
                                          index=['epoch ' + str(i+1)]))
        # print(i+1, a, temp_a,temp_a==a, b, temp_b,temp_b==b, mse, temp_mse, temp_mse==mse)
        # print(i+1)
        if temp_a==a and temp_b==b and temp_mse==mse:
            # print(i)
            break
        temp_a = a
        temp_b = b
        temp_mse = mse
 
    return a, b, trace
# 2320 1.0999999999999988 1.0999999999999988 True  -0.0666666666666641  -0.06666666666666408 False 0.05555555555555553  0.055555555555555504 False
# 2321 1.0999999999999988 1.0999999999999988 True  -0.0666666666666641  -0.0666666666666641  True  0.05555555555555553  0.05555555555555553  True

# 9999 1.1000000000000514 1.0999999999999488 False -0.06666666666664432 -0.06666666666668944 False 0.055555555555555504 0.05555555555555556  False
#10000 1.0999999999999488 1.1000000000000514 False -0.06666666666668944 -0.06666666666664432 False 0.05555555555555556  0.055555555555555504 False

# 9999 1.0999999984358646 1.0999999984336053 False -0.06666666311102022 -0.06666666310588418 False 0.055555555555555504 0.05555555555555556  False
#10000 1.0999999984381206 1.0999999984358646 False -0.06666666311614873 -0.06666666311102022 False 0.05555555555555553  0.055555555555555504 False
def displayResult(_a, _b, _trace):
  plt.figure( figsize=(30,5))
 
  plt.subplot(1, 4, 1)
  plt.grid(True)
  plt.title("Distribution & line result")
  plt.scatter(X,y)
  plt.plot([X[0], X[len(X)-1]], [_a * X[0] + _b, _a * X[len(X)-1] + _b], 'r-', lw=2)
 
  plt.subplot(1, 4, 2)
  plt.title("Iterations (Coeff. a) per epochs")
  plt.plot(_trace['a'])
 
  plt.subplot(1, 4, 3)
  plt.title("Iterations (Coeff. b) per epochs")
  plt.plot(_trace['b'])
 
  plt.subplot(1, 4, 4)
  plt.title("MSE")
  plt.plot(_trace['mse'])
 
  print (_trace)
  plt.show()

# X = [1.0, 2.0, 3.0]
# y = [1.2, 1.8, 3.4]
A,B = 1.1, -2/3
# print(A,B)
# X = []
# y = []
# for i in range(0,1000):
#     to_x =np.random(0,1)
#     X.append(to_x)
#     y.append(A*to_x + B)
# np.random.seed(1727773457)
# np.random.seed(72172712)

num_step = 10000
num_var = 3

samples = np.random.randn(num_var)
X = samples
y = A*samples + B + np.random.randn(num_var)

def do_show(A,B,C=1,D=1000):
    # print(A,B)
    a, b, trace = gradient_descent(A, B, C, _epochs=D)
    displayResult(a, b, trace)

do_show(X.copy(),y.copy(),1,num_step)
do_show(X.copy(),y.copy(),2,num_step)
do_show(X.copy(),y.copy(),5,num_step)
do_show(X.copy(),y.copy(),6,num_step)
# a, b, trace = gradient_descent(X, y, 3, _epochs=100000)
# displayResult(a, b, trace)
# a, b, trace = gradient_descent(X, y, 4, _epochs=100000)
# displayResult(a, b, trace)
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())
# print(np.random.RandomState().get_state())

# 0.951333380260973
# 0.9513333802609729
# 0.9513349597305195
# 0.951333380260973

# 285 1.1305812000178983 1.130581200017898 False -0.6648922306427782 -0.6648922306427781 False 0.951333380260973 0.951333380260973 True       
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014590149504239829
# 286 1.1305812000178983 1.1305812000178983 True -0.6648922306427784 -0.6648922306427782 False 0.951333380260973 0.951333380260973 True       
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014577423674859143
# 287 1.1305812000178983 1.1305812000178983 True -0.6648922306427785 -0.6648922306427784 False 0.9513333802609731 0.951333380260973 False
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014564753151219703
# 288 1.1305812000178983 1.1305812000178983 True -0.6648922306427786 -0.6648922306427785 False 0.951333380260973 0.9513333802609731 False     
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014552137502179978
# 289 1.1305812000178983 1.1305812000178983 True -0.6648922306427786 -0.6648922306427786 True 0.951333380260973 0.951333380260973 True        

# 285 1.1305812000178983 1.130581200017898 False -0.6648922306427782 -0.6648922306427781 False 0.951333380260973 0.951333380260973 True
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014590149504239829
# 286 1.1305812000178983 1.1305812000178983 True -0.6648922306427784 -0.6648922306427782 False 0.951333380260973 0.951333380260973 True       
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014577423674859143
# 287 1.1305812000178983 1.1305812000178983 True -0.6648922306427785 -0.6648922306427784 False 0.9513333802609731 0.951333380260973 False     
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014564753151219703
# 288 1.1305812000178983 1.1305812000178983 True -0.6648922306427786 -0.6648922306427785 False 0.951333380260973 0.9513333802609731 False     
# 0.06 0.13112392898332514 0.00011999999999999999 5.993454529946038e-05 5.999994000009e-05 0.014552137502179978
# 289 1.1305812000178983 1.1305812000178983 True -0.6648922306427786 -0.6648922306427786 True 0.951333380260973 0.951333380260973 True        
