# import numpy as np

# from PIL import Image
# img = Image.open('ellipsedented.png').convert('L')
# A = np.array(img)

# x = []; y=[]; I=[]
# for i in  range(A.shape[1]): 
#      for j in range(A.shape[0]):
#          x.append(float(i)) 
#          y.append(A.shape[0]-float(j)) # images have (0,0) on upper left
#          I.append(A[j,i])
# x = np.array(x); y=np.array(y); I=np.array(I)

# h = 50; k = 20; rx = 20; ry = 20; mu=150; eta = 0.000001
# itera = 1000
# C = ((I-mu)**2) + 0.01
# for i in range(itera):
#      tmp = -2*C*(x-h) / rx**2
#      h_step = tmp.sum() * eta 
#      tmp = -2*C*(y-k) / ry**2
#      k_step = tmp.sum() * eta 
#      tmp = -2*C*(x-h)**2 / np.power(rx,3)
#      rx_step = tmp.sum() * eta
#      tmp = -2*C*(y-k)**2 / np.power(ry,3)
#      ry_step = tmp.sum() * eta
#      h = h - h_step
#      k = k - k_step
#      rx = rx - rx_step
#      ry = ry - ry_step
#      #break
# print(h,k,rx,ry)

# # for plots
# import matplotlib.pyplot as plt
# from matplotlib.patches import Ellipse
# ell = Ellipse(xy=[h,A.shape[0]-k], width=rx, height=ry)
# fig, ax = plt.subplots()
# ax.add_patch(ell)
# # plt.hold(True)
# plt.imshow(img)
# plt.show()

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.patches import Ellipse
from PIL import Image

img = Image.open('ellipsedented.png').convert('L')
A = np.array(img)

x = []; y=[]; I=[]
for i in  range(A.shape[1]): 
     for j in range(A.shape[0]):
         x.append(float(i)) 
         y.append(A.shape[0]-float(j)) # imalarin (0,0) noktasi ust solda
         I.append(A[j,i])
x = np.array(x); y=np.array(y); I=np.array(I)

h = 20; k = 20; rx = 30; ry = 30; mu=150; eta = 0.01
M = 50; iter = 1000

C =  -1*np.log((np.abs(I-mu) / M) + 0.001)

def R(xx,yy,hh,kk,rxx,ryy):
    return ((xx-hh)**2 / rxx**2) + ((yy-kk)**2 / ryy**2)  

def plot_ellipse_image(h,k,rx,ry,i):
     ell = Ellipse(xy=[h,A.shape[0]-k], width=rx, height=ry, alpha=0.2)
     fig, ax = plt.subplots()
     ax.add_patch(ell)
    #  plt.hold(True)
     plt.imshow(img)
     plt.savefig('ellipse_fitting_%d.png' % i) 

delay=1

for i in range(iter):
     old = np.array([h,k,rx,ry])
     r = R(x,y,h,k,rx,ry)
     tmp = (-2*C*(x-h) / rx**2) / r
     h_step = pd.Series(tmp).fillna(0).sum() * eta 
     tmp = (-2*C*(y-k) / ry**2) / r
     k_step = pd.Series(tmp).fillna(0).sum() * eta 
     tmp = (-2*C*(x-h)**2 / np.power(rx,3)) / r
     rx_step = pd.Series(tmp).fillna(0).sum() * eta / delay
     tmp = (-2*C*(y-k)**2 / np.power(ry,3)) / r
     ry_step = pd.Series(tmp).fillna(0).sum() * eta / delay
     h = h - h_step
     k = k - k_step
     rx = rx - rx_step
     ry = ry - ry_step     
     new = np.array([h,k,rx,ry])
     if i % 100 == 0:
         plot_ellipse_image(h,k,rx,ry,i)
         print(h,k,rx,ry,np.abs((new-old).sum()), i)
     if np.abs((new-old).sum()) < 0.40: break