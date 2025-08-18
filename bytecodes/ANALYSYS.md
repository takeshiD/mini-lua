# Lua51

```lua
local a = 1
local b = a + 2
print("hello world")
print(b)
```

をluacでコンパイルしたあとに`luac -l`で閲覧

```bin
main <local_assign.lua:0,0> (9 instructions, 36 bytes at 0x425880)
0+ params, 4 slots, 0 upvalues, 2 locals, 4 constants, 0 functions
        1       [1]     LOADK           0 -1    ; 1
        2       [2]     ADD             1 0 -2  ; - 2
        3       [3]     GETGLOBAL       2 -3    ; print
        4       [3]     LOADK           3 -4    ; "hello world"
        5       [3]     CALL            2 2 1
        6       [4]     GETGLOBAL       2 -3    ; print
        7       [4]     MOVE            3 1
        8       [4]     CALL            2 2 1
        9       [4]     RETURN          0 1
```

# Register Machine
> Reference: [The Implementation of Lua5.0](https://www.lua.org/doc/jucs05.pdf)


