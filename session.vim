let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Programming/Advent_Of_Code_2025
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash
badd +29 src/main.rs
badd +51 src/day1.rs
badd +1 src/lib.rs
badd +1 LICENSE
badd +14 README.md
badd +196 ~/Programming/Eggsh/main.c
badd +19 ~/Programming/Eggsh/todo.md
badd +1 ~/Programming/Eggsh/Makefile
badd +208 ~/Programming/Eggsh/src/command_builder_and_tokenizer.c
badd +84 ~/Programming/Eggsh/include/tokenizer.h
badd +9 ~/Programming/Eggsh/include/testing/tokenizer_runner.h
badd +1 ~/Programming/Eggsh/tests_src/tokenizer_runner.c
badd +194 ~/Programming/Eggsh/src/hash_map.c
badd +97 ~/Programming/Eggsh/src/command_runner.c
badd +135 term://~/Programming/Eggsh//478306:/usr/bin/bash
badd +75 term://~/Programming/Eggsh//523606:/usr/bin/bash
badd +1 ~/Source_Code/dash/src/alias.c
badd +329 ~/Source_Code/dash/src/trap.c
badd +45 ~/Source_Code/dash/src/main.h
badd +53 ~/Source_Code/dash/src/options.h
badd +108 ~/Source_Code/dash/src/options.c
badd +21 man://sigsetops(3)
badd +95 ~/Source_Code/dash/src/jobs.h
badd +61 ~/Source_Code/dash/src/error.h
badd +92 ~/Source_Code/dash/src/error.c
badd +1 man://setjmp(3)
badd +174 ~/Source_Code/dash/src/main.c
badd +94 ~/Source_Code/dash/src/shell.h
badd +22 man://sigqueue(3)
badd +14 man://sigaction(3p)
badd +0 man://sigaction(2)
badd +0 man://signal(7)
badd +4 src/day2.cpp
argglobal
%argdel
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/main.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 119 + 119) / 238)
exe '2resize ' . ((&lines * 31 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 118 + 119) / 238)
exe '3resize ' . ((&lines * 22 + 28) / 57)
exe 'vert 3resize ' . ((&columns * 118 + 119) / 238)
argglobal
balt src/lib.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 64 - ((53 * winheight(0) + 27) / 54)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 64
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
argglobal
if bufexists(fnamemodify("~/Programming/Advent_Of_Code_2025/src/lib.rs", ":p")) | buffer ~/Programming/Advent_Of_Code_2025/src/lib.rs | else | edit ~/Programming/Advent_Of_Code_2025/src/lib.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Programming/Advent_Of_Code_2025/src/lib.rs
endif
balt ~/Programming/Advent_Of_Code_2025/src/main.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 15) / 31)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
argglobal
if bufexists(fnamemodify("term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash", ":p")) | buffer term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash | else | edit term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash | endif
if &buftype ==# 'terminal'
  silent file term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash
endif
balt term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
let s:l = 9055 - ((21 * winheight(0) + 11) / 22)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 9055
normal! 03|
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
exe 'vert 1resize ' . ((&columns * 119 + 119) / 238)
exe '2resize ' . ((&lines * 31 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 118 + 119) / 238)
exe '3resize ' . ((&lines * 22 + 28) / 57)
exe 'vert 3resize ' . ((&columns * 118 + 119) / 238)
tabnext
edit ~/Programming/Advent_Of_Code_2025/src/day2.cpp
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe 'vert 1resize ' . ((&columns * 118 + 119) / 238)
exe 'vert 2resize ' . ((&columns * 119 + 119) / 238)
argglobal
balt ~/Programming/Advent_Of_Code_2025/src/day1.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 6 - ((5 * winheight(0) + 27) / 54)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 6
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
argglobal
if bufexists(fnamemodify("term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash", ":p")) | buffer term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash | else | edit term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash | endif
if &buftype ==# 'terminal'
  silent file term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash
endif
balt ~/Programming/Advent_Of_Code_2025/src/day1.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
let s:l = 9055 - ((53 * winheight(0) + 27) / 54)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 9055
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
exe 'vert 1resize ' . ((&columns * 118 + 119) / 238)
exe 'vert 2resize ' . ((&columns * 119 + 119) / 238)
tabnext 2
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
