import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from math import pi, cos, sin

t = np.linspace(0, 2*pi, 100)
list_to_show = []
list_to_show2 = []


# 6 paramètres, pas de tilt
#

# distance 3D
dist3D = lambda X1,Y1,Z1,X2,Y2,Z2 : ((X1-X2)**2+(Y1-Y2)**2+(Z1-Z2)**2)**(1/2)
dist13D = lambda X1,Y1,Z1,X2,Y2,Z2,S1,S2,S3 : ((X1-X2)**2/S1**2+(Y1-Y2)**2/S2**2+(Z1-Z2)**2/S3**2)**(1/2)

equation32 = lambda Cx,Cy,Cz,X,Y,Z,S1,S2,S3: ((X- (Cx-X)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))**2 + (Y- (Cy-Y)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))**2 + (Z- (Cz-Z)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))**2 )**(1/2)
equation2 = lambda Cx,Cy,Cz,X,Y,Z,S1,S2,S3: dist3D(X, Y, Z, Cx+ (X-Cx)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3) ,Cy+ (Y-Cy)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3), Cz+ (Z-Cz)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))
equation7 = lambda Cx,Cy,Cz,X,Y,Z,S1,S2,S3: dist3D(Cx, Cy, Cz, X, Y, Z) - dist3D(Cx, Cy, Cz, Cx+ (X-Cx)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3) ,Cy+ (Y-Cy)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3), Cz+ (Z-Cz)/dist13D(X, Y, Z, Cx, Cy, Cz, S1, S2, S3))


def printls(*args):
  return print(*args, sep='', end='\r')

def gradient_descent(_x, _y, _z, çbeta, state , çfig, àax, _learningrate):
    global precision, step_btw_frame

    x = np.array(_x)
    y = np.array(_y)
    z = np.array(_z)
    N = len(_x)

    print(çbeta)

    A = çbeta[0][1]
    B = çbeta[1][1]
    C = çbeta[2][1]

    D = çbeta[0][0]
    E = çbeta[1][0]
    F = çbeta[2][0]

    my_delta = np.array(equation7(D, E, F, x, y, z, A, B, C))
    my_delta2 = np.array(equation2(D, E, F, x, y, z, A, B, C))

    i=0
    arra = []
    for e in [i, A, B, C, D, E, F, my_delta.sum()/N, my_delta2.sum()/N]:
        arra.append([e])

    print("\nbegin: x\n", x, "\nend: x\n","\nbegin: y\n", y, "\nend: y\n","\nbegin: z\n", z, "\nend: z\n")

    lowest = [100000,0,0,0,0,0]

    displayResult2(A, B, C, D, E, F, my_delta, arra, x, y, z, çfig, àax, çbeta)
    # plt.show(block = True)
    plt.pause(0.1)
    bg = çfig.canvas.copy_from_bbox(çfig.bbox)
    displayResult(A, B, C, D, E, F, my_delta, arra, x, y, z, çfig, àax, çbeta)
    plt.show(block=False)

    çfig.canvas.blit(çfig.bbox)
    updateResult(A, B, C, D, E, F, my_delta, arra, x, y, z, çfig, àax, çbeta, bg)
    temp = [0,0,0,0,0,0,0,0,0]
    temp2= ((A-temp[1])**2 + (B-temp[2])**2 + (C-temp[3])**2 + (D-temp[4])**2 + (E-temp[5])**2 + (F-temp[6])**2 + ((my_delta.sum()/N)-temp[7])**2 + ((my_delta2.sum()/N)-temp[8])**2)

    while temp2 >= precision:
        if i %(step_btw_frame)==0:
            updateResult(A, B, C, D, E, F, my_delta, arra, x, y, z, çfig, àax, çbeta, bg)
        my_delta = np.array(equation7(D, E, F, x, y, z, A, B, C)) # reste de l'equation
        my_delta2 = np.array(equation2(D, E, F, x, y, z, A, B, C)) # le même sans le signe

        # print("\nbegin: state\n", i,"|",A,"|",B,"|",C,"|",D,"|",my_delta.sum(),"\n", ellipse,"\nend: state\n","\nbegin: my_delta\n", my_delta, "\nend: my_delta\n")
        if abs(my_delta.sum()) <= abs(lowest[0]): lowest=[my_delta.sum()/N,my_delta2.sum()/N,i,A,B,C,D,E,F]

        if state == 1:#just to make the calculation stand out
            A_temp = A
            B_temp = B
            C_temp = C
            D_temp = D
            E_temp = E
            F_temp = F

            # calcul de dérivée

            # my_delta = r = distance
            # my delta : signé, delta2 : pas signé.
            # faire delta2 + delta permet de pas faire partir les paramètres de position trop loin.
            # 
            # A,B,C sont des scales
            A = A -_learningrate * ( (-2*(1/N))* (1/(A_temp*A_temp*A_temp)) * (abs((x-D_temp)**2)).dot(my_delta).sum() )
            B = B -_learningrate * ( (-2*(1/N))* (1/(B_temp*B_temp*B_temp)) * (abs((y-E_temp)**2)).dot(my_delta).sum() )
            C = C -_learningrate * ( (-2*(1/N))* (1/(C_temp*C_temp*C_temp)) * (abs((z-F_temp)**2)).dot(my_delta).sum() )


            # D, E, F : position x,y,z
            D = D -_learningrate * ( (2*(1/N))* (1/(A_temp*A_temp)) * (((D_temp-x).dot(my_delta2 + my_delta)).sum()) )
            E = E -_learningrate * ( (2*(1/N))* (1/(B_temp*B_temp)) * (((E_temp-y).dot(my_delta2 + my_delta)).sum()) )
            F = F -_learningrate * ( (2*(1/N))* (1/(C_temp*C_temp)) * (((F_temp-z).dot(my_delta2 + my_delta)).sum()) )

        #condition d'arrêt
        temp2= ((A-temp[1])**2 + (B-temp[2])**2 + (C-temp[3])**2 + (D-temp[4])**2 + (E-temp[5])**2 + (F-temp[6])**2 + ((my_delta.sum()/N)-temp[7])**2 + ((my_delta2.sum()/N)-temp[8])**2)
        printls(" ",i," | ", temp2, '\t\t\t\t\t\t') #osef

        # H est un historique
        for H,e in enumerate([i, A, B, C, D, E, F, my_delta.sum()/N, my_delta2.sum()/N]):
            arra[H].append(e)
            temp[H] = e
        i += 1
    print('\n', lowest)
    updateResultfinal(A, B, C, D, E, F, my_delta, arra, x, y, z, çfig, àax, çbeta, bg)
    return A, B, C, D, E, F, my_delta.sum()/N, arra


def remove_plot(çax, y1, y2):
    for E,e in enumerate(çax):
        if y2>E and y1<=E:
            try:
                e.remove()
                # print("removed :")
            except:
                # print("tried to remove, but failed :")
                pass
        # print(E, e, y1, y2)

def graph_plot_more(çfig, çax, p1, P1, P2):
    remove_plot(çax, P1, P2)
    temp = çfig.add_subplot(çax[p1].get_gridspec()[p1,P1:P2])
    temp.cla()
    return temp

def graph_plot_ellipse(çfig, çax, çP = [0,0,0], çlabel= ["",""], çvalues = [], çelip = [[0,0,0,0,'',""]]):
    global t

    ax1 = graph_plot_more(çfig, çax[çP[0]], çP[0], çP[1], çP[2])
    ax1.grid(True)
    ax1.set_xlabel(çlabel[0])
    ax1.set_ylabel(çlabel[1])
    for e in çelip:
        plot_ellipse_2D(ax1, t, e[0], e[1], e[2], e[3], e[4], e[5], çvalues)

def plot_ellipse_2D(çax, çt, çX, çY, çS1, çS2, çcolor, çname, çvalues):
  global list_to_show, colors

  çEll = np.array([çS1*np.cos(çt) , çS2*np.sin(çt)])
  list_to_show.append([çname, çax.plot( çX + çEll[0,:] , çY + çEll[1,:], color = çcolor, animated=True), çax, çax.scatter(çvalues[0], çvalues[1], color = çvalues[2]), [çvalues[0], çvalues[1]] ])

def graph_plot_var(çfig, çax, çname_var, çarra, çname, pos = [0,0,0]):
    global list_to_show

    new_ax = graph_plot_more(çfig, çax[pos[0]], pos[0], pos[1], pos[2])
    new_ax.grid(True)
    new_ax.set_autoscale_on(True)
    new_ax.set_autoscaley_on(True)
    new_ax.set_autoscalex_on(True)
    list_to_show.append([çname, new_ax.plot(çarra[0], çarra[çname_var], 'r-', animated=True), new_ax, çname_var])

def update_ellipse_2D(some, çt, çX, çY, çS1, çS2):
    çEll = np.array([çS1*np.cos(çt) , çS2*np.sin(çt)])
    some[1][0].set_xdata(çX + çEll[0,:])
    some[1][0].set_ydata(çY + çEll[1,:])
    some[2].relim()
    some[2].autoscale_view()
    some[2].draw_artist(some[2].get_xaxis())
    some[2].draw_artist(some[2].get_yaxis())
    some[2].draw_artist(some[3])
    some[2].draw_artist(some[1][0])

def update_var(some, çX, çY):
    some[1][0].set_xdata(çX)
    some[1][0].set_ydata(çY)
    some[2].relim()
    some[2].autoscale()
    some[2].autoscale_view()
    some[2].draw_artist(some[2].get_xaxis())
    some[2].draw_artist(some[2].get_yaxis())
    some[2].draw_artist(some[1][0])

def updateResult(_A, _B, _C, _D, _E, _F, _my_delta, _arra, Xvalues, Yvalues, Zvalues, fig, ax, çbeta, bg):
    global list_to_show, t

    fig.canvas.restore_region(bg)
    for e in list_to_show:
        if e[0] == "ellipseXY":
            update_ellipse_2D(e, t, _D, _E, _A, _B)
        if e[0] == "ellipseYZ":
            update_ellipse_2D(e, t, _E, _F, _B, _C)
        if e[0] == "ellipseZX":
            update_ellipse_2D(e, t, _F, _D, _C, _A)
        # if e[0] == "ellipse":
        #     update_ellipse_2D(e, ax, t, _D, _E, _A, _B)
        if e[0] == "start ellipse":
            e[2].draw_artist(e[1][0])
        if e[0] == "var":
            update_var(e, _arra[0], _arra[e[3]])
    fig.canvas.blit(fig.bbox)
    fig.canvas.flush_events()

def updateResultfinal(_A, _B, _C, _D, _E, _F, _my_delta, _arra, Xvalues, Yvalues, Zvalues, fig, ax, çbeta, bg):
    global list_to_show, t

    fig.canvas.restore_region(bg)
    for e in list_to_show:
        if e[0] == "ellipseXY":
            update_ellipse_2D(e, t, _D, _E, _A, _B)
        if e[0] == "ellipseYZ":
            update_ellipse_2D(e, t, _E, _F, _B, _C)
        if e[0] == "ellipseZX":
            update_ellipse_2D(e, t, _F, _D, _C, _A)
        # if e[0] == "ellipse":
        #     update_ellipse_2D(e, ax, t, _D, _E, _A, _B)
        if e[0] == "start ellipse":
            e[2].draw_artist(e[1][0])
        if e[0] == "var":
            update_var(e, _arra[0], _arra[e[3]])
    fig.canvas.blit(fig.bbox)
    fig.canvas.flush_events()

def displayResult(_A, _B, _C, _D, _E, _F, _my_delta, _arra, Xvalues, Yvalues, Zvalues, fig, ax, çbeta):
    global colors

    graph_plot_ellipse(fig, ax, [0, 0, 2], ["", ""], [Xvalues, Yvalues, colors[0]], [[_D, _E, _A, _B, colors[2], "ellipseXY"], [çbeta[0][0], çbeta[1][0], çbeta[0][1], çbeta[1][1], colors[1], "start ellipse"]])
    graph_plot_ellipse(fig, ax, [0, 2, 4], ["", ""], [Yvalues, Zvalues, colors[0]], [[_E, _F, _B, _C, colors[2], "ellipseYZ"], [çbeta[1][0], çbeta[2][0], çbeta[1][1], çbeta[2][1], colors[1], "start ellipse"]])
    graph_plot_ellipse(fig, ax, [0, 4, 6], ["", ""], [Zvalues, Xvalues, colors[0]], [[_F, _D, _C, _A, colors[2], "ellipseZX"], [çbeta[2][0], çbeta[0][0], çbeta[2][1], çbeta[0][1], colors[1], "start ellipse"]])
    graph_plot_var(fig, ax, 1, _arra, "var", [1,0,1])
    graph_plot_var(fig, ax, 2, _arra, "var", [1,1,2])
    graph_plot_var(fig, ax, 3, _arra, "var", [1,2,3])
    graph_plot_var(fig, ax, 4, _arra, "var", [1,3,4])
    graph_plot_var(fig, ax, 5, _arra, "var", [1,4,5])
    graph_plot_var(fig, ax, 6, _arra, "var", [1,5,6])
    graph_plot_var(fig, ax, 7, _arra, "var", [2,0,3])
    graph_plot_var(fig, ax, 8, _arra, "var", [2,3,6])

def graph_plot_var2(çfig, çax, çname_var, çarra,çname, pos = [0,0,0]):
    global list_to_show

    new_ax = graph_plot_more(çfig, çax[pos[0]], pos[0], pos[1], pos[2])
    new_ax.grid(True)
    new_ax.set_title(params[çname_var])
    new_ax.set_xticks([])
    new_ax.set_yticks([])

def graph_plot_ellipse2(çfig, çax, çP = [0,0,0], çlabel= ["",""], çvalues = [], çelip = [[0,0,0,0,'',""]]):
    ax1 = graph_plot_more(çfig, çax[çP[0]], çP[0], çP[1], çP[2])
    ax1.grid(True)
    ax1.set_xticks([])
    ax1.set_yticks([])

def displayResult2(_A, _B, _C, _D, _E, _F, _my_delta, _arra, Xvalues, Yvalues, Zvalues, fig, ax, çbeta):
    global colors
    graph_plot_ellipse2(fig, ax, [0, 0, 2], ["", ""], [Xvalues, Yvalues], [[_D, _E, _A, _B, 'g', "ellipseXY"], [çbeta[0][0], çbeta[1][0], çbeta[0][1], çbeta[1][1], 'r', "start ellipse"]])
    graph_plot_ellipse2(fig, ax, [0, 2, 4], ["", ""], [Yvalues, Zvalues], [[_E, _F, _B, _C, 'g', "ellipseYZ"], [çbeta[1][0], çbeta[2][0], çbeta[1][1], çbeta[2][1], 'r', "start ellipse"]])
    graph_plot_ellipse2(fig, ax, [0, 4, 6], ["", ""], [Zvalues, Xvalues], [[_F, _D, _C, _A, 'g', "ellipseZX"], [çbeta[2][0], çbeta[0][0], çbeta[2][1], çbeta[0][1], 'r', "start ellipse"]])
    graph_plot_var2(fig, ax, 1, _arra, "var", [1,0,1])
    graph_plot_var2(fig, ax, 2, _arra, "var", [1,1,2])
    graph_plot_var2(fig, ax, 3, _arra, "var", [1,2,3])
    graph_plot_var2(fig, ax, 4, _arra, "var", [1,3,4])
    graph_plot_var2(fig, ax, 5, _arra, "var", [1,4,5])
    graph_plot_var2(fig, ax, 6, _arra, "var", [1,5,6])
    graph_plot_var2(fig, ax, 7, _arra, "var", [2,0,3])
    graph_plot_var2(fig, ax, 8, _arra, "var", [2,3,6])

def veleurs2D(çalpha, çbeta, çnum_var, çrandi, çstating):
    samples1 = 2*np.random.randn(çnum_var)
    ellipse = [ çbeta[0][0] + çalpha[0][0]*(np.random.randn()), çbeta[0][1] + çalpha[0][1]*(np.random.randn()),
                çbeta[1][0] + çalpha[1][0]*(np.random.randn()), çbeta[1][1] + çalpha[1][1]*(np.random.randn()),
                çbeta[2][0] + çalpha[2][0]*(np.random.randn()), çbeta[2][1] + çalpha[2][1]*(np.random.randn()) ]
    Xinit = çstating[0] * (ellipse[0] + ellipse[1]*np.cos(samples1) + çrandi*np.random.randn(num_var))
    Yinit = çstating[1] * (ellipse[2] + ellipse[3]*np.sin(samples1) + çrandi*np.random.randn(num_var))
    Zinit = 0 * (ellipse[4] + ellipse[5]*np.sin(samples1) + çrandi*np.random.randn(num_var))
    return Xinit, Yinit, Zinit, ellipse

#
def veleurs3D(çalpha, çbeta, çnum_var, çrandi, çstating):
    samples1 = 2*np.random.randn(çnum_var) # 2 angles
    samples2 = np.random.randn(çnum_var)
    ellipse = [ çbeta[0][0] + çalpha[0][0]*(np.random.randn()), çbeta[0][1] + çalpha[0][1]*(np.random.randn()),
                çbeta[1][0] + çalpha[1][0]*(np.random.randn()), çbeta[1][1] + çalpha[1][1]*(np.random.randn()),
                çbeta[2][0] + çalpha[2][0]*(np.random.randn()), çbeta[2][1] + çalpha[2][1]*(np.random.randn()) ]
    Xinit = çstating[0] * (ellipse[0] + ellipse[1]*np.sin(samples1)*np.cos(samples2) + çrandi*np.random.randn(num_var))
    Yinit = çstating[1] * (ellipse[2] + ellipse[3]*np.sin(samples1)*np.sin(samples2) + çrandi*np.random.randn(num_var))
    Zinit = çstating[2] * (ellipse[4] + ellipse[5]*np.cos(samples1)                  + çrandi*np.random.randn(num_var))
    return Xinit, Yinit, Zinit, ellipse

def do_show(çalpha, çbeta, çnum_var, çrandi, çstating, learningrate = 0.05, _state=1):
    global colors

    fig = plt. figure(num = 172)
    ax = fig.subplots(3,6)
    # plt.get_current_fig_manager().window.state('zoomed')

    _X, _Y ,_Z, _Elip = veleurs3D(çalpha, çbeta, çnum_var, çrandi, çstating)
    çA, çB, çC, çD, çE, çF, çmy_delta, çarra = gradient_descent(_X, _Y, _Z, çbeta, _state, fig, ax, _learningrate = learningrate)

    print("true ellipse: ", _Elip)
    sor_elli_val = [1, _Elip[1], _Elip[3], _Elip[5], _Elip[0], _Elip[2], _Elip[4], np.array(equation7(_Elip[0], _Elip[2], _Elip[4], _X, _Y, _Z, _Elip[1], _Elip[3], _Elip[5])).sum()/len(_X), np.array(equation2( _Elip[0], _Elip[2], _Elip[4], _X, _Y, _Z,_Elip[1], _Elip[3], _Elip[5])).sum()/len(_X), "", ""]
    for H,e in enumerate(çarra):
        print(params[H], ": | begin: ", e[0], "| result: ",e[-1], " | goal: ", sor_elli_val[H])

    plt.close()

    fig, ax = plt.subplots(num = 712, nrows = 1, ncols = 2, subplot_kw = dict(projection='3d'))
    # plt.get_current_fig_manager().window.state('zoomed')

    u = np.linspace(0, 2 * np.pi, 100)
    v = np.linspace(0, np.pi, 100)
    x = çD + çA * np.outer(np.cos(u), np.sin(v))
    y = çE + çB * np.outer(np.sin(u), np.sin(v))
    z = çF + çC * np.outer(np.ones(np.size(u)), np.cos(v))

    # xtrue = _Elip[0] + _Elip[1] * np.outer(np.cos(u), np.sin(v))
    # ytrue= _Elip[2] + _Elip[3] * np.outer(np.sin(u), np.sin(v))
    # ztrue = _Elip[4] + _Elip[5] * np.outer(np.ones(np.size(u)), np.cos(v))
    xinit = çbeta[0][0] + çbeta[0][1] * np.outer(np.cos(u), np.sin(v))
    yinit = çbeta[1][0] + çbeta[1][1] * np.outer(np.sin(u), np.sin(v))
    zinit = çbeta[2][0] + çbeta[2][1] * np.outer(np.ones(np.size(u)), np.cos(v))

    for e in ax:
        e.set_xlabel("X(A,D)")
        e.set_ylabel("Y(B,E)")
        e.set_zlabel("Z(C,F)")

    ax[0].scatter(_X, _Y, _Z, color = colors[0])
    ax[0].plot_surface(xinit, yinit, zinit, color = colors[1])

    ax[1].plot_surface(xinit, yinit, zinit, color = colors[1])
    ax[1].plot_surface(x, y, z, color = colors[2])
    ax[1].scatter(_X, _Y, _Z, color = colors[0])
    plt.show()

colors = [(0, 0, 1, 0.72), (1, 0, 0, 0.5), (0, 1, 0, 0.72), (0.72, 0, 0.72, 0.27)]

params = ["i", "A", "B", "C", "D", "E", "F", "my_delta", "my_delta2"]
precision = 1e-5
step_btw_frame = 500
num_var = 1000
stating = [1,1,1]#[scale dim,..] #pas toucher


alpha = [[1000, 100], [100, 10], [100, 10]]#[[scale alea pos, scale alea coef], ..]

beta = [[1000, 100, 1], [-1000, 100, 1], [100, 100, 1000]]#[[scale pos, scale coef, scale alea pos], ..]
randi = 1
with plt.style.context('dark_background'):
    do_show(alpha, beta, num_var, randi, stating, learningrate = 1)
# do_show(alpha, beta, num_step, num_var, randi, stating, learningrate = 1)

