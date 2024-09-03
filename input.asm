.text
ldi A 42
sta 1
lda 1
ldi B 5
add
sta 2
lda 1
mov B M 2
cmp
jnz %else_branch
lda 2
ldi B 1
add
sta 2
jmp %endif
else_branch:
endif:
hlt
