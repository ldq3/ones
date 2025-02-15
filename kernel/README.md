设置 logger level，在 `mise run` 前环境变量 `LOG` 为（区分大小写）：
- error
- warn
- info
- debug
- trace

# 链接脚本
`linker.ld`

`src/runtime/entry.asm`

`src/virtualization/cpu/exception/handler.S`

符号 sbss 的位置不可更改 
bss.stack 节的位置也不能动

符号：
- 程序入口：_start