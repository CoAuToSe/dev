# 3 Observations
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
 
# plt.ion()
def printls(*args):
  return print(*args, sep='', end='\r')
  # import sys, time
  # for i in xrange(0, 101, 10):
  #   print('\r>> You have finished %d%%' % i,)
  #   sys.stdout.flush()
  #   time.sleep(2)
  # print

def mean_squared_error(y, X = []):
    return sum((y-X)**2)/len(X)

# def mean_squared_error(y, X = []):
#     return ((y - X) ** 2).mean()
coeff = 1
# Q1, Q2 = 0,0 
def gradient_descent(_x, _y, _z, çbeta, elipse, state , çfig, àax, _learningrate = 0, _epochs=5):
    # global Q1,Q2
    # trace = pd.DataFrame(columns=['A', 'dA', 'B', 'dB', 'C', 'dC', 'D', 'dD', 'my_delta'])
    trace = pd.DataFrame(columns=['A', 'B', 'C', 'D', 'E', 'F', 'my_delta'])
    x = np.array(_x)
    y = np.array(_y)
    z = np.array(_z)
    # a, b = 0.0, 0.0 # Initialisation aléatoire de a et b
    # mse = []
    N = len(_x) 
    # temp_mse = mean_squared_error(y, (a*x + b))
    # temp_learningrate = _learningrate   
    # A,B,C,D = 1.0, 1.0, 0.0, 0.0    
    A = çbeta[0][1]
    B = çbeta[1][1]
    C = çbeta[2][1]
    # C = sum(x)/len(x) + np.random.randn()
    # D = sum(y)/len(y) + np.random.randn()
    D = çbeta[0][0]
    E = çbeta[1][0]
    F = çbeta[2][0]
    # Q1 = beta[2]
    # Q2 = beta[3]
    # Q2 = beta[3]
    # C = Q1
    # D = Q2
    temp_A = A
    temp_B = B
    temp_C = C
                                   
    equation = lambda A,B,C,D,X,Y: A*X.dot(X).sum()+B*Y.dot(Y).sum()+C*X.sum()+D*Y.sum()

    equation1 = lambda A,B,C,D,X,Y: ((X-C)*(X-C))/(A*A)+( ((Y-D)*(Y-D)) /(B*B))
    equation2 = lambda A,B,C,D,X,Y: ((X*(1-1/((X/A)**2+(Y/B)**2)**(1/2)))**2+(Y*(1-1/((X/A)**2+(Y/B)**2)**(1/2)))**2)**(1/2)
    equation3 = lambda A,B,C,D,X,Y: ((X-( C+(X-C)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2+ (Y-( D+(Y-D)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2)**(1/2)
    equation4 = lambda A,B,C,D,X,Y: ((C-X)**2+(D-Y)**2)**(1/2)-((C-( C+(X-C)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2+ (D-( D+(Y-D)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2)**(1/2)
    equation5 = lambda A,B,C,D,E,F,X,Y: ((C-X)**2+(D-Y)**2)**(1/2)-((C-( C+(X-C)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2+ (D-( D+(Y-D)/( ( (((X-C)**2)/(A**2))+(((Y-D)**2)/(B**2)) )**(1/2) )) )**2)**(1/2)
    
    equation6 = lambda Cx,Cy,X,Y,S1,S2: dist2D(Cx, Cy, X, Y) - dist2D(Cx-(Cx+ (X-Cx)/dist12D(X, Y, Cx, Cy, S1, S2) ) ,Cy-(Cy+ (Y-Cy)/dist12D(X, Y, Cx, Cy, S1, S2)))
    equation7 = lambda Cx,Cy,Cz,X,Y,Z,S1,S2,S3: dist3D(Cx, Cy, Cz, X, Y, Z) - dist3D(Cx, Cy, Cz, Cx+ (X-Cx)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3) ,Cy+ (Y-Cy)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3), Cz+ (Z-Cz)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))
    
    dist12D = lambda X1,Y1,X2,Y2,S1,S2 : ((X1-X2)**2/S1**2+(Y1-Y2)**2/S2**2)**(1/2)
    dist13D = lambda X1,Y1,Z1,X2,Y2,Z2,S1,S2,S3 : ((X1-X2)**2/S1**2+(Y1-Y2)**2/S2**2+(Z1-Z2)**2/S3**2)**(1/2)
    dist2D = lambda X1,Y1,X2,Y2 : ((X1-X2)**2+(Y1-Y2)**2)**(1/2)
    dist3D = lambda X1,Y1,Z1,X2,Y2,Z2 : ((X1-X2)**2+(Y1-Y2)**2+(Z1-Z2)**2)**(1/2)
    
    # derive1 = lambda A,B,C,D,X,Y: 2*A*X.dot(X.dot(X.dot(X))).sum()+2*B*X.dot(X.dot(Y.dot(Y))).sum()+2*C*X.dot(X.dot(X)).sum()+2*D*X.dot(X.dot(Y)).sum()-2*X.dot(X).sum()
    # derive1(A,B,C,D,x,y) = derive en fonction de A
    # derive1(B,A,D,C,y,x) = derive en fonction de B
    acc1 = lambda X : 2*X.dot(X.dot(X.dot(X))).sum()
    # acc1(x)
    # acc1(y)
    derive2= lambda A,B,C,D,X,Y: 2*C*X.dot(X).sum()+2*A*X.dot(X.dot(X)).sum()+2*B*Y.dot(Y.dot(X)).sum()+2*D*X.dot(Y).sum()-2*X.sum()
    # derive2(A,B,C,D,x,y) = derive en fonction de C
    # derive2(B,A,D,C,y,x) = derive en fonction de D
    acc2 = lambda X : 2*X.dot(X).sum()
    # acc2(x)
    # acc2(y)
    my_delta = np.array(equation7(D, E, F, x, y, z, A, B, C))
    
    # dA = ( (-2/N)* (1) * x.dot(x.dot(my_delta)).sum() )
    # dB = ( (-2/N)* (1) * y.dot(y.dot(my_delta)).sum() )
    # dC = (-2/N)* x.dot(my_delta).sum()
    # dD = (-2/N)* y.dot(my_delta).sum()

    # mse = mean_squared_error(y, (a*x + b))
    # trace = trace.append(pd.DataFrame(data=[[A, dA, B, dB, C, dC, D, dD, my_delta.sum()]], 
    #                                   columns=['A', 'dA', 'B', 'dB', 'C', 'dC', 'D', 'dD', 'my_delta'], 
    #                                   index=['epoch ' + str(0)]))
                                      
        
    trace = trace.append(pd.DataFrame(data=[[A, B, C, D, E, F, my_delta.sum()]], 
                                      columns=['A', 'B', 'C', 'D', 'E', 'F', 'my_delta'], 
                                      index=['epoch ' + str(0)]))
                                      
    print("\nbegin: x\n", x, "\nend: x\n","\nbegin: y\n", y, "\nend: y\n","\nbegin: z\n", z, "\nend: z\n")

    lowest = [100000,0,0,0,0,0]
    
    # t_temp = np.linspace(0, 2*np.pi, 100)
    for i in range(_epochs):
        # print(àax)
        # plot_ellipse_2D(graph_plot_more(çfig, àax, 0, 0, 2), t_temp, D, E, A, B, 'r')
        # displayPreResult(A, B, C, D, E, F, my_delta, trace, x, y, elipse, çbeta)
        # plt.pause(0.1)
        printls(" ",i+1,' '*(len(str(_epochs))-len(str(i+1))),'/', _epochs, " | ",str(((i+1)*100)/_epochs)+"%")
        # delta = y - (a*x + b)
        my_delta = np.array(equation7(D, E, F, x, y, z, A, B, C))
        # print("\nbegin: state\n", i,"|",A,"|",B,"|",C,"|",D,"|",my_delta.sum(),"\n", ellipse,"\nend: state\n","\nbegin: my_delta\n", my_delta, "\nend: my_delta\n")
        # for j in my_delta:
        #   print(j) 
        if (abs(my_delta.sum()) >= abs(lowest[0])) and i >= 10 : break
        if abs(my_delta.sum()) <= abs(lowest[0]): lowest=[my_delta.sum(),i,A,B,C,D,E,F]
        # delta_square = (y - (a*X + b))**2

        # temp_learningrate = _learningrate/((i+1)**0.25)
        # temp2a_learningrate = _learningrate*(2*sum(X**2))/N
        # temp2b_learningrate = _learningrate*(2)/N
        # temp3a_learningrate = _learningrate/((N**2-2*sum(X**2))**(1/2))
        # temp3b_learningrate = _learningrate/((N**2-2)**(1/2))
        # print(_learningrate,temp2a_learningrate, temp2b_learningrate, temp3a_learningrate, temp3b_learningrate, temp_learningrate)
        # Updating a and b
        # print(delta,delta_square,X,X.dot(delta).sum(), delta.sum(), N)
        if state == 1:
            # a = a -temp3a_learningrate * (-2 * x.dot(delta).sum()) # on retire un gradient à a
            # b = b -temp3b_learningrate * (-2 * delta.sum()) # idem pour b
            # print("lol")
            
            # A_temp = (A/abs(A))*max(abs(A), 0.01)
            # B_temp = (B/abs(B))*max(abs(B), 0.01)
            # dA = (1/(A_temp*A_temp*A_temp)) * (x-C_temp).dot(x-C_temp).sum() 
            # dB = (1/(B_temp*B_temp*B_temp)) * (y-D_temp).dot(y-D_temp).sum() 
            # dC = ((10) * (C_temp-x).sum()/(A_temp*A_temp))**2
            # dD = ((10) * (D_temp-y).sum()/(B_temp*B_temp))**2 


            # if abs(dA) >= abs(dB) and abs(dA) >= abs(dC) and abs(dA) >= abs(dD):
            #   A = A -_learningrate * ( (-2*(1/N-2/(N*N)+1/(N*N*N) ))* (1/(A_temp*A_temp*A_temp)) * (x-C_temp).dot(x-C_temp).sum() )
            # if abs(dB) >= abs(dA) and abs(dB) >= abs(dC) and abs(dB) >= abs(dD):
            #   B = B -_learningrate * ( (-2*(1/N-2/(N*N)+1/(N*N*N) ))* (1/(B_temp*B_temp*B_temp)) * (y-D_temp).dot(y-D_temp).sum() )
            # if abs(dC) >= abs(dA) and abs(dC) >= abs(dB) and abs(dC) >= abs(dD):
            #   C = C -_learningrate * ( (2*(1/N-2/(N*N)+1/(N*N*N) ))* (10) * (C_temp-x).sum()/(A_temp*A_temp) )
            # if abs(dD) >= abs(dA) and abs(dD) >= abs(dB) and abs(dD) >= abs(dC):
            #   D = D -_learningrate * ( (2*(1/N-2/(N*N)+1/(N*N*N) ))* (10) * (D_temp-y).sum()/(B_temp*B_temp) )
            
            # A = A -_learningrate * ( (-2*(1/N-2/(N*N)+1/(N*N*N) ))* (1/(A_temp*A_temp*A_temp)) * ((x-C_temp)**2).dot(my_delta).sum() )
            # B = B -_learningrate * ( (-2*(1/N-2/(N*N)+1/(N*N*N) ))* (1/(B_temp*B_temp*B_temp)) * ((y-D_temp)**2).dot(my_delta).sum() )
            # C = C -_learningrate * ( (2*(1/N-2/(N*N)+1/(N*N*N) ))* (2/(A_temp*A_temp)) * ((C_temp-x.dot(my_delta))).sum() )
            # D = D -_learningrate * ( (2*(1/N-2/(N*N)+1/(N*N*N) ))* (2/(B_temp*B_temp)) * ((D_temp-y.dot(my_delta))).sum() )
            
            # A = A -_learningrate * ( (-2*(1/N))* (1/(A_temp*A_temp*A_temp)) * (abs((x-C_temp)**2)).dot(my_delta).sum() )
            # B = B -_learningrate * ( (-2*(1/N))* (1/(B_temp*B_temp*B_temp)) * (abs((y-D_temp)**2)).dot(my_delta).sum() )
            # C = C -_learningrate * ( (-2*(1/(N*N)))* (1/(A_temp*A_temp)) * (((C_temp-x).dot(my_delta)).sum()) )
            # D = D -_learningrate * ( (-2*(1/(N*N)))* (1/(B_temp*B_temp)) * (((D_temp-y).dot(my_delta)).sum()) )
            
            A_temp = A
            B_temp = B
            C_temp = C
            D_temp = D
            E_temp = E
            F_temp = F
            
            A = A -_learningrate * ( (-2*(1/N))* (1/(A_temp*A_temp*A_temp)) * (abs((x-D_temp)**2)).dot(my_delta).sum() )
            B = B -_learningrate * ( (-2*(1/N))* (1/(B_temp*B_temp*B_temp)) * (abs((y-E_temp)**2)).dot(my_delta).sum() )
            C = C -_learningrate * ( (-2*(1/N))* (1/(C_temp*C_temp*C_temp)) * (abs((z-F_temp)**2)).dot(my_delta).sum() )
            
            D = D -_learningrate * ( (-2*(1/(N*N)))* (1/(A_temp*A_temp)) * (((D_temp-x).dot(my_delta)).sum()) )
            E = E -_learningrate * ( (-2*(1/(N*N)))* (1/(B_temp*B_temp)) * (((E_temp-y).dot(my_delta)).sum()) )
            F = F -_learningrate * ( (-2*(1/(N*N)))* (1/(C_temp*C_temp)) * (((F_temp-z).dot(my_delta)).sum()) )
            
            # A = A -_learningrate * ( (-2/N)* (1) * x.dot(x.dot(my_delta)).sum() )
            # B = B -_learningrate * ( (-2/N)* (1) * y.dot(y.dot(my_delta)).sum() )
            # A = A -const*_learningrate * ( (-2/N)* x.dot(x.dot(my_delta)).sum() )
            # B = B -const*_learningrate * ( (-2/N)* y.dot(y.dot(my_delta)).sum() )
            # C = C -const*_learningrate * ( (-2/N)* x.dot(my_delta).sum() )
            # D = D -const*_learningrate * ( (-2/N)* y.dot(my_delta).sum() )
            # print("new",A,B,C,D,x,y)
        # else:
        #     if state == 2:
        #         # a = a -_learningrate * (-2 * x.dot(delta).sum() / N) # on retire un gradient à a
        #         # b = b -_learningrate * (-2 * delta.sum() / N) # idem pour b

        #         A_temp = A
        #         B_temp = B
        #         C_temp = C
        #         D_temp = D
        #         dA = derive1(A_temp, B_temp, C_temp, D_temp, x, y)
        #         dB = derive1(B_temp, A_temp, D_temp, C_temp, y, x)
        #         dC = derive2(A_temp, B_temp, C_temp, D_temp, x, y)
        #         dD = derive2(B_temp, A_temp, D_temp, C_temp, y, x)

        #         A = A +const*_learningrate * dA / N
        #         B = B +const*_learningrate * dB / N
        #         C = C +const*_learningrate * dC / N
        #         D = D +const*_learningrate * dD / N
                # print("new",A,B,C,D,x,y)
            # else:
                # if state == 3:
                #     a = a -temp2a_learningrate * (-2 * x.dot(delta).sum() / N) # on retire un gradient à a
                #     b = b -temp2b_learningrate * (-2 * delta.sum() / N) # idem pour b
                # else:
                #     if state ==4:
                #         a = a -temp_learningrate * (-2 * x.dot(delta).sum() / N) # on retire un gradient à a
                #         b = b -temp_learningrate * (-2 * delta.sum() / N) # idem pour b
                #     else:
                #         if state == 5:
                #             temp = x.copy()
                #             temp2 =x.dot(delta).sum()
                #             temp3 =x.dot(temp).sum()
                #             a = a -_learningrate * ((-2 * temp2 / N)*(1 - (-2 * temp3 /N)) ) # on retire un gradient à a
                #             b = b -_learningrate * ((-2 * delta.sum() / N )*(1 - (2/N)) ) # idem pour b
                #         else:
                #             if state == 6:
                #                 a = a -_learningrate * ((-2 * x.dot(delta).sum() / N)*(1 + (2 * x.dot(x.copy()).sum() /N)) ) # on retire un gradient à a
                #                 b = b -_learningrate * ((-2 * delta.sum() / N )*(1 + (2/N)) ) # idem pour b
        # a = a -_learningrate * (-2 * X.dot(delta).sum()) # on retire un gradient à a
        # b = b -_learningrate * (-2 * delta.sum()) # idem pour b
        # print(y, (a*X + b),a,X,b)
        # mse = mean_squared_error(y, (a*x + b))
        # mse2 = mean_squared_error(y, ( ( (1-A**2*(x-B)**2)/(C**2) )**(1/2) + D ) )
        # mse2 = mean_squared_error(y, (A*x**2) )
        # print(mse2)
 
        # trace = trace.append(pd.DataFrame(data=[[A, dA, B, dB, C, dC, D, dD, my_delta.sum()]], 
        #                                   columns=['A', 'dA', 'B', 'dB', 'C', 'dC', 'D', 'dD', 'my_delta'], 
        #                                   index=['epoch ' + str(i+1)]))
                                          
        trace = trace.append(pd.DataFrame(data=[[A, B, C, D, E, F, my_delta.sum()]], 
                                          columns=['A', 'B', 'C', 'D', 'E', 'F', 'my_delta'], 
                                          index=['epoch ' + str(i+1)]))
        # print(i+1, a, temp_a,temp_a==a, b, temp_b,temp_b==b, mse, temp_mse, temp_mse==mse)
        # print(i+1)
        # if temp_A == A and temp_B == B:
        #     # print(i)
        #     break
        # temp_A = A
        # temp_B = B
        # temp_mse = mse
    print('\n', lowest)
    return A, B, C, D, E, F, my_delta.sum(), trace
# 2320 1.0999999999999988 1.0999999999999988 True  -0.0666666666666641  -0.06666666666666408 False 0.05555555555555553  0.055555555555555504 False
# 2321 1.0999999999999988 1.0999999999999988 True  -0.0666666666666641  -0.0666666666666641  True  0.05555555555555553  0.05555555555555553  True

# 9999 1.1000000000000514 1.0999999999999488 False -0.06666666666664432 -0.06666666666668944 False 0.055555555555555504 0.05555555555555556  False
#10000 1.0999999999999488 1.1000000000000514 False -0.06666666666668944 -0.06666666666664432 False 0.05555555555555556  0.055555555555555504 False

# 9999 1.0999999984358646 1.0999999984336053 False -0.06666666311102022 -0.06666666310588418 False 0.055555555555555504 0.05555555555555556  False
#10000 1.0999999984381206 1.0999999984358646 False -0.06666666311614873 -0.06666666311102022 False 0.05555555555555553  0.055555555555555504 False

def plot_ellipse_2D(çax, çt, çX, çY, çS1, çS2, çcolor):
  çEll = np.array([çS1*np.cos(çt) , çS2*np.sin(çt)])  
  çax.plot( çX + çEll[0,:] , çY + çEll[1,:], çcolor)     #initial ellipse

def update_ellipse_2D(çax, çt, çX, çY, çS1, çS2, çcolor):
  çEll = np.array([çS1*np.cos(çt) , çS2*np.sin(çt)])  
  çax.plot( çX + çEll[0,:] , çY + çEll[1,:], çcolor)     #initial ellipse
  plt.pause(0.1)

def graph_plot_var(çax, çname_var, çtrace):
  çax.set_title(çname_var)
  çax.set_xticks([])
  # çax.yticks([])
  çax.plot(çtrace[çname_var])

def remove_plot(çax, y1, y2):
    for E,e in enumerate(çax):
        if y2>E and y1<=E:
            try:
                e.remove()
                print("removed :")
            except:
                print("tried to remove, but failed :")
        print(E, e, y1, y2)


# def update_line(hl, new_data):
#     hl.set_xdata(numpy.append(hl.get_xdata(), new_data))
#     hl.set_ydata(numpy.append(hl.get_ydata(), new_data))
#     plt.draw()

# def anime(ax, xdata, ydata):
#     lines.set_xdata(xdata)
#     lines.set_ydata(ydata)
#     #Need both of these in order to rescale
#     ax.relim()
#     ax.autoscale_view()
#     #We need to draw *and* flush
#     figure.canvas.draw()
#     figure.canvas.flush_events()

def graph_plot_more(çfig, çax, p1, P1, P2):
    remove_plot(çax, P1, P2)
    # print(çax[p1])
    return çfig.add_subplot(çax[p1].get_gridspec()[p1,P1:P2])

# def graph_plot_ellipse2(çfig, çax, çp1, çP1, çP2, çxlabel, çylabel, çXvalues, çYvalues, çX, çY, çS1, çS2, çcolor):
#     ax1 = graph_plot_more(çfig, çax, çp1, çP1, çP2)
#     ax1.grid(True)
#     # ax[0].title("points")
#     ax1.set_xlabel(çxlabel)
#     ax1.set_ylabel(çylabel)
#     ax1.scatter(çXvalues,çYvalues)
#     from math import pi, cos, sin
#     t = np.linspace(0, 2*pi, 100)
#     plot_ellipse_2D(ax1, t, çX, çY, çS1, çS2, çcolor)

def graph_plot_ellipse(çfig, çax, çP = [0,0,0], çlabel= ["",""], çvalues = [], çelip = [[0,0,0,0,'']]):
    ax1 = graph_plot_more(çfig, çax, çP[0], çP[1], çP[2])
    ax1.grid(True)
    # ax[0].title("points")
    ax1.set_xlabel(çlabel[0])
    ax1.set_ylabel(çlabel[1])
    ax1.scatter(çvalues[0], çvalues[1])
    from math import pi, cos, sin
    t = np.linspace(0, 2*pi, 100)
    for e in çelip:
        plot_ellipse_2D(ax1, t, e[0], e[1], e[2], e[3], e[4])

# def displayPreResult(_A, _B, _C, _D, _E, _F, _my_delta, _trace, Xvalues, Yvalues, _Elips, çbeta):
#     fig, ax = plt.subplots(3,6)
#     plt.plot([], [])
#     graph_plot_ellipse(fig, ax[0], [0, 0, 2], ["", ""], [Xvalues, Yvalues], [[_D, _E, _A, _B, 'g'], [çbeta[0][0], çbeta[1][0], çbeta[0][1], çbeta[1][1], 'r']])
#     plt.draw()
    

def displayResult(_A, _B, _C, _D, _E, _F, _my_delta, _trace, Xvalues, Yvalues, fig, ax ,_Elips, çbeta):
    # fig, ax = plt.subplots(3,6)
  # fig.figure( figsize=(30,5))
  # print(Q1,Q2)
 
  # ax[0].subplot(1, 1, 1)
#   for e in ax[0]:
#     e.remove()
#   ax1 = fig.add_subplot(ax[0, 1].get_gridspec()[0,0:2])
#   graph_plot_ellipse(fig, ax[0], 0, 2, 4, "x, A,C","y, B,D",Xvalues,Yvalues,_D, _E, _A, _B, 'g')
    graph_plot_ellipse(fig, ax[0], [0, 0, 2], ["", ""], [Xvalues, Yvalues], [[_D, _E, _A, _B, 'g'], [çbeta[0][0], çbeta[1][0], çbeta[0][1], çbeta[1][1], 'r']])
    graph_plot_ellipse(fig, ax[0], [0, 2, 4], ["", ""], [Xvalues, Yvalues], [[_D, _E, _A, _B, 'g']])
    graph_plot_ellipse(fig, ax[0], [0, 4, 6], ["", ""], [Xvalues, Yvalues], [[_D, _E, _A, _B, 'g']])
    graph_plot_var(ax[1][0], 'A', _trace)
    graph_plot_var(ax[1][1], 'B', _trace)
    graph_plot_var(ax[1][2], 'C', _trace)
    graph_plot_var(ax[1][3], 'D', _trace)
    graph_plot_var(ax[1][4], 'E', _trace)
    graph_plot_var(ax[1][5], 'F', _trace)
    graph_plot_var(graph_plot_more(fig, ax[2], 2, 0, 6), 'my_delta', _trace)
#   ax1 = graph_plot_more(fig, ax[0], 0, 0, 2)
#   ax1.grid(True)
#   # ax[0].title("points")
#   ax1.set_xlabel("x, A,C")
#   ax1.set_ylabel("y, B,D")
#   ax1.scatter(Xvalues,Yvalues)
#   from math import pi, cos, sin
#   t = np.linspace(0, 2*pi, 100)
#   plot_ellipse_2D(ax1, t, _D, _E, _A, _B, 'g')
#   Ell = np.array([_A*np.cos(t) , _B*np.sin(t)])  
#   ax1.plot( _C+Ell[0,:] , _D+Ell[1,:], 'g' )     #initial ellipse

#   Ell = np.array([beta[0]*coeff*np.cos(t) , beta[1]*coeff*np.sin(t)])  
#   ax1.plot( Q1+Ell[0,:] , Q2+Ell[1,:], 'r')     #initial ellipse
  
#   Ell = np.array([(beta[0]+abs(alpha[0])+abs(alpha[2]))*coeff*np.cos(t) , (beta[1]+abs(alpha[1])+abs(alpha[3]))*coeff*np.sin(t)])  
#   ax1.plot( Q1+Ell[0,:] , Q2+Ell[1,:], 'k')     #initial ellipse
  
#   Ell = np.array([(beta[0]-abs(alpha[0])-abs(alpha[2]))*coeff*np.cos(t) , (beta[1]-abs(alpha[1])-abs(alpha[3]))*coeff*np.sin(t)])  
#   ax1.plot( Q1+Ell[0,:] , Q2+Ell[1,:], 'k')     #initial ellipse
  
#   plt.plot([X[0], X[len(X)-1]], [_a * X[0] + _b, _a * X[len(X)-1] + _b], 'r-', lw=2)
 
  # ax[1][0].subplot(1, 5, 1)
#   ax[1][0].set_title("A")
#   ax[1][0].set_xticks([])
#   # plt.yticks([])
#   ax[1][0].plot(_trace['A'])
  
#   plt.subplot(1, 9, 3)
#   plt.title("dA")
#   plt.plot(_trace['dA'])
 
  # ax[1].subplot(1, 5, 2)
#   ax[1][1].set_title("B")
#   ax[1][1].set_xticks([])
#   # plt.yticks([])
#   ax[1][1].plot(_trace['B'])
 
#   plt.subplot(1, 9, 5)
#   plt.title("dB")
#   plt.plot(_trace['dB'])
  
  # ax[1].subplot(1, 5, 3)
#   ax[1][2].set_title("C")
#   ax[1][2].set_xticks([])
#   # plt.yticks([])
#   ax[1][2].plot(_trace['C'])

#   plt.subplot(1, 9, 7)
#   plt.title("dC")
#   plt.plot(_trace['dC'])

#   # ax[1].subplot(1, 5, 4)
#   ax[1][3].set_title("D")
#   ax[1][3].set_xticks([])
#   # plt.yticks([])
#   ax[1][3].plot(_trace['D'])
  
#   plt.subplot(1, 9, 9)
#   plt.title("dD")
#   plt.plot(_trace['dD'])

#   # ax[1].subplot(1, 5, 5)
#   ax[1][4].set_title("my_delta")
#   ax[1][4].set_xticks([])
#   # plt.yticks([])
#   ax[1][4].plot(_trace['my_delta'])
  
  # plt.subplot(1, 5, 5)
    print(_trace)
    print(_Elips)
    # fig.tight_layout()
    figManager = plt.get_current_fig_manager()
    figManager.window.showMaximized()
    plt.show(block = False)


# X = [1.0, 2.0, 3.0]
# y = [1.2, 1.8, 3.4]
# print(A,B)
# X = []
# y = []
# for i in range(0,1000):
#     to_x =np.random(0,1)
#     X.append(to_x)
#     y.append(A*to_x + B)
# np.random.seed(1727773457)
# np.random.seed(72172712)

# Ainit,Binit = 1.1, -2/3
# samples2 = np.random.randn(num_var)
# X = samples + ellipse[0]
# Y = ( ( (1-ellipse[2]**2*(X-ellipse[1])**2)/(ellipse[3]**2) )**(1/2) + ellipse[1] ) 
# X = ellipse[2]*np.cos(2*samples1) 
# Y = ellipse[3]*np.sin(2*samples1) 
def veleurs(çalpha, çbeta, çnum_var, çrandi, çstating):
    samples1 = np.random.randn(çnum_var)
    ellipse = [ çbeta[0][0] + çalpha[0][0]*(np.random.randn()), çbeta[0][1] + çalpha[0][1]*(np.random.randn()), 
                çbeta[1][0] + çalpha[1][0]*(np.random.randn()), çbeta[1][1] + çalpha[1][1]*(np.random.randn()),
                çbeta[2][0] + çalpha[2][0]*(np.random.randn()), çbeta[2][1] + çalpha[2][1]*(np.random.randn()) ]
    Xinit = çstating[0] * (ellipse[0] + ellipse[1]*np.cos(samples1) + çrandi*np.random.randn(num_var))
    Yinit = çstating[1] * (ellipse[2] + ellipse[3]*np.sin(samples1) + çrandi*np.random.randn(num_var))
    Zinit = çstating[2] * (ellipse[4] + ellipse[5]*np.sin(samples1) + çrandi*np.random.randn(num_var))
    return Xinit, Yinit, Zinit, ellipse
# print(X,Y)
# X1 = samples
# Y1 = Ainit*samples**2 + Binit *  + np.random.randn(num_var)

def do_show(çalpha, çbeta, çnum_step, çnum_var, çrandi, çstating, learningrate = 0.05, _state=1):
    # print(A,B)
    # plt.show(block = False)
    # plt.ion()
    fig, ax = plt.subplots(3,6)
    _X, _Y ,_Z, _Elip = veleurs(çalpha, çbeta, çnum_var, çrandi, çstating)
    çA, çB, çC, çD, çE, çF, çmy_delta, çtrace = gradient_descent(_X, _Y, _Z, çbeta, _Elip, _state, fig, ax, _learningrate = learningrate, _epochs=çnum_step)
    displayResult(çA, çB, çC, çD, çE, çF, çmy_delta, çtrace, _X, _Y, fig, ax, _Elip, çbeta)
    plt.show(block = True)

num_step = 1000
num_var = 100
stating = [1,1,0]#[scale pos,..]
alpha = [[10, 10], [10, 10], [0, 0]]#[[scale alea coef, scale alea pos], ..]
beta = [[1000, 100, 1], [-1000, 100, 1], [0, 1, 0]]#[[scale pos, scale coef, scale alea pos], ..]
randi = 1

do_show(alpha, beta, num_step, num_var, randi, stating, learningrate = 1) 
# do_show(alpha, beta, num_step, num_var, randi, stating, learningrate = 1) 
# do_show(alpha, beta, num_step, num_var, randi, stating, learningrate = 1) 
# do_show(alpha, beta, num_step, num_var, randi, stating, learningrate = 1) 

# ellipse = [beta[0] + alpha[0]*(np.random.randn()), beta[1] + alpha[1]*(np.random.randn()), beta[2] + alpha[2]*(np.random.randn()), beta[3] + alpha[3]*(np.random.randn())]#[centre.x, centre.y, scalar.x, scalar.y]
# samples1 = np.random.randn(num_var)
# Xinit = ellipse[2]+ellipse[0]*np.cos(samples1) + randi * np.random.randn(num_var)
# Yinit = ellipse[3]+ellipse[1]*np.sin(samples1) + randi * np.random.randn(num_var)
# do_show(Xinit.copy(),Yinit.copy(),1,num_step)

# ellipse = [beta[0] + alpha[0]*(np.random.randn()), beta[1] + alpha[1]*(np.random.randn()), beta[2] + alpha[2]*(np.random.randn()), beta[3] + alpha[3]*(np.random.randn())]#[centre.x, centre.y, scalar.x, scalar.y]
# samples1 = np.random.randn(num_var)
# Xinit = ellipse[2]+ellipse[0]*np.cos(samples1) + randi * np.random.randn(num_var)
# Yinit = ellipse[3]+ellipse[1]*np.sin(samples1) + randi * np.random.randn(num_var)
# do_show(Xinit.copy(),Yinit.copy(),1,num_step)

# ellipse = [beta[0] + alpha[0]*(np.random.randn()), beta[1] + alpha[1]*(np.random.randn()), beta[2] + alpha[2]*(np.random.randn()), beta[3] + alpha[3]*(np.random.randn())]#[centre.x, centre.y, scalar.x, scalar.y]
# samples1 = np.random.randn(num_var)
# Xinit = ellipse[2]+ellipse[0]*np.cos(samples1) + randi * np.random.randn(num_var)
# Yinit = ellipse[3]+ellipse[1]*np.sin(samples1) + randi * np.random.randn(num_var)
# do_show(Xinit.copy(),Yinit.copy(),1,num_step)

# ellipse = [beta[0] + alpha[0]*(np.random.randn()), beta[1] + alpha[1]*(np.random.randn()), beta[2] + alpha[2]*(np.random.randn()), beta[3] + alpha[3]*(np.random.randn())]#[centre.x, centre.y, scalar.x, scalar.y]
# samples1 = np.random.randn(num_var)
# Xinit = ellipse[2]+ellipse[0]*np.cos(samples1) + randi * np.random.randn(num_var)
# Yinit = ellipse[3]+ellipse[1]*np.sin(samples1) + randi * np.random.randn(num_var)
# do_show(Xinit.copy(),Yinit.copy(),1,num_step)

# ellipse = [beta[0] + alpha[0]*(np.random.randn()), beta[1] + alpha[1]*(np.random.randn()), beta[2] + alpha[2]*(np.random.randn()), beta[3] + alpha[3]*(np.random.randn())]#[centre.x, centre.y, scalar.x, scalar.y]
# samples1 = np.random.randn(num_var)
# Xinit = ellipse[2]+ellipse[0]*np.cos(samples1) + randi * np.random.randn(num_var)
# Yinit = ellipse[3]+ellipse[1]*np.sin(samples1) + randi * np.random.randn(num_var)
# do_show(Xinit.copy(),Yinit.copy(),1,num_step)

# do_show(X.copy(),Y.copy(),2,num_step)
# do_show(X.copy(),Y.copy(),1,num_step)
# do_show(X.copy(),Y.copy(),5,num_step)
# do_show(X.copy(),Y.copy(),6,num_step)
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
