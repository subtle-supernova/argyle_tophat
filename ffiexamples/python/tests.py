import ctypes as ct
from sys import platform, exit

libtype = "rlib"

if "linux" == platform or "linux2" == platform:
    libtype = "so"
elif "win" == platform:
    print("WARNING: This might not work. Add DLL to compile types.")
    libtype = "dll"
elif platform == "darwin":
    libtype = "dylib"

lib = ct.cdll.LoadLibrary("../target/debug/libffiexamples.%s" % libtype)

lib.print_hi()

class CStructExample(ct.Structure):
    _fields_ = [("words", ct.c_char_p), ("number", ct.c_int)]

lib.print_cstruct_example.argtypes = (CStructExample, )

lib.print_cstruct_example(CStructExample('hi there'.encode('utf-8'), 1))
