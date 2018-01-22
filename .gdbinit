target remote localhost:1234
symbol-file build/kernel-x86_64.bin
break kmain
python
try:
    gdb.execute("continue")
except:
    pass
end
disconnect
set architecture i386:x86-64
target remote localhost:1234
