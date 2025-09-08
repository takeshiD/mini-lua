# Source code
```lua
local a = 1000000000
local b = a + 2
print("hello world")
print(b)
```

## luac
```
main <bytecodes/local_assign.lua:0,0> (9 instructions, 36 bytes at 0x425880)
0+ params, 4 slots, 0 upvalues, 2 locals, 4 constants, 0 functions
        1       [1]     LOADK           0 -1    ; 1000000000
        2       [2]     ADD             1 0 -2  ; - 2
        3       [3]     GETGLOBAL       2 -3    ; print
        4       [3]     LOADK           3 -4    ; "hello world"
        5       [3]     CALL            2 2 1
        6       [4]     GETGLOBAL       2 -3    ; print
        7       [4]     MOVE            3 1
        8       [4]     CALL            2 2 1
        9       [4]     RETURN          0 1
constants (4) for 0x425880:
        1       1000000000
        2       2
        3       "print"
        4       "hello world"
locals (2) for 0x425880:
        0       a       2       9
        1       b       3       9
upvalues (0) for 0x425880:
```

## undump
```
Instructions:
  [0]  0x00000001     line1
  [1]  0x0040404C     line2
  [2]  0x00008085     line3
  [3]  0x0000C0C1     line3
  [4]  0x0100409C     line3
  [5]  0x00008085     line4
  [6]  0x008000C0     line4
  [7]  0x0100409C     line4
  [8]  0x0080001E     line4
ConstantTable:
  [00]  Number(1000000000)
  [01]  Number(2)
  [02]  String("print")
  [03]  String("hello world")
Locals:
  [00]   "a" (line 1-8)
  [01]   "b" (line 2-8)
```

## Comparison

| luac               | umdump       | 備考                                                                             |
| ---------------    | ---------    | --------                                                                         |
| `LOADK     0 -1`   | `0x00000001` | 定数テーブルの1番目(100000000)をレジスタ0にmoveする                              |
| `ADD       1 0 -2` | `0x0040404C` | レジスタ0と定数テーブル2番目(2)の加算結果をレジスタ1にmoveする                   |
| `GETGLOBAL 2 -3`   | `0x00008085` | 定数テーブル3番目(print)をグローバルとしてレジスタ2にmvoe                        |
| `LOADK     3 -4`   | `0x0000C0C1` | 定数テーブル4番目(hello world)をレジスタ3にmove                                  |
| `CALL      2 2 1`  | `0x0100409C` | R(2),R(1) := R(2)(R(3),R(3)) => print,100000000 := print(helloworld, helloworld) |
| `GETGLOBAL 2 -3`   | `0x00008085` | 定数テーブル3番目(print)をグローバルとしてレジスタ2にmvoe                        |
| `MOVE      3 1`    | `0x008000C0` | R(3) := R(1) => R(3) := 1000002|
| `CALL      2 2 1`  | `0x0100409C` | R(2),R(1) := R(2)(R(3),R(3)) => print,100000002 := print(10000002, 10000002) |
  `RETURN    0 1`  | `0x0080001E` | return R(0), R(-1) |

| PC | 命令             | R\[0]      | R\[1]      | R\[2]    | R\[3]         | 説明                                                                                                                       |
| -- | ---------------- | ---------- | ---------- | -----    | ------------- | -----------------------------------------------------------------------------------                                        |
| 1  | `LOADK 0 -1`     | 1000000000 |            |          |               | a = 1000000000                                                                                                             |
| 2  | `ADD 1 0 -2`     | 1000000000 | 1000000002 |          |               | b = a + 2                                                                                                                  |
| 3  | `GETGLOBAL 2 -3` | 1000000000 | 1000000002 | print    |               | グローバルから関数 print を取る                                                                                            |
| 4  | `LOADK 3 -4`     | 1000000000 | 1000000002 | print    | "hello world" | 引数準備                                                                                                                   |
| 5  | `CALL 2 2 1`     | 1000000000 | 1000000002 | *使用中* | "hello world" | CALL: R\[2] が関数、R\[3] が引数。この範囲を使って print(R\[3]) を実行。終了後、戻り値なしなので R\[2],R\[3] は未定義扱い |
| 6  | `GETGLOBAL 2 -3` | 1000000000 | 1000000002 | print    |               | print を再取得                                                                                                             |
| 7  | `MOVE 3 1`       | 1000000000 | 1000000002 | print    | 1000000002    | b を引数レジスタへ移動                                                                                                     |
| 8  | `CALL 2 2 1`     | 1000000000 | 1000000002 | *使用中* | 1000000002    | CALL: R\[2] が関数、R\[3] が引数。print(R\[3]) 実行。戻り値なし                                                            |
| 9  | `RETURN 0 1`     | …          | …          | …        | …             | 関数終了                                                                                                                   |

