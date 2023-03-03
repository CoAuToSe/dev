from ctypes import *

rust4python = cdll.LoadLibrary(".\\target\\debug\\rust4python.dll")
rust4python.lib_test()
rust4python.wtf(10)
rust4python.sum_as_string(10, 10)
rust4python.sum_as_string_fail(10, 10)
temp = ''
some = rust4python.greet(temp)
print(some)

mydll = cdll.LoadLibrary(".\\target\\debug\\rust4python.dll")
mydll.lib_test()
mydll.wtf(10)
