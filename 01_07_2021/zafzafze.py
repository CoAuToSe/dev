import matplotlib
matplotlib.use('WXAgg')

# import wx
import pylab as p
import numpy as npy
from time import sleep

ax = p.subplot(111)
canvas = ax.figure.canvas
x = npy.arange(0,2*npy.pi,0.01)
line, = p.plot(x, npy.sin(x), animated=True)

ax.get_yaxis().set_animated(True)

def update_line(*args):
    if update_line.background is None:
        update_line.background = canvas.copy_from_bbox(ax.bbox)

    for i in range(20):
        canvas.restore_region(update_line.background)

        line.set_ydata((i/10.0)*npy.sin(x))
        ax.set_ylim(-1*i/5.0-0.5,i/5.0+0.5)

        ax.draw_artist(ax.get_yaxis())

        ax.draw_artist(line)

        canvas.blit(ax.bbox)

        sleep(0.1)
    print('end')


# update_line.cnt = 0
# update_line.background = None
# wx.EVT_IDLE(wx.GetApp(), update_line)
p.show()