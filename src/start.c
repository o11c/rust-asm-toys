asm (
".globl      _start\n\t"
"\n\t"
".text\n\t"
"_start:\n\t"
"    movq %rsp, %rdi\n\t"
"    jmp _start2\n\t"
);
