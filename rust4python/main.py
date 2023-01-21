from ctypes import *

mydll = cdll.LoadLibrary(".\\target\\debug\\rust4python.dll")
mydll.lib_test()
mydll.wtf(10)