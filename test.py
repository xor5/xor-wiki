import ctypes
lh = ctypes.CDLL("./libasm.so")
print(lh.init())