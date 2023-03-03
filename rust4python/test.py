# import pyo3

# rust_lib = pyo3.PyRustLibrary(path_to_rust_lib)
# rust_lib.greet("world")

from ctypes import *
# give location of dll
mydll = cdll.LoadLibrary(".\\target\\debug\\rust4python.dll")
# result1= mydll.add(10,1)
# result2= mydll.sub(10,1)
# print("Addition value:-"+result1)
# print("Substraction:-"+result2)
mydll.lib_test()