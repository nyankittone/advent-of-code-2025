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
badd +62 term://~/Programming/Advent_Of_Code_2025//289675:/usr/bin/bash
badd +1 src/main.rs
badd +1 src/day1.rs
badd +0 src/lib.rs
badd +0 LICENSE
badd +14 README.md
argglobal
%argdel
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/lib.rs
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
exe '2resize ' . ((&lines * 32 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 118 + 119) / 238)
exe '3resize ' . ((&lines * 22 + 28) / 57)
exe 'vert 3resize ' . ((&columns * 118 + 119) / 238)
argglobal
balt src/main.rs
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
let s:l = 1 - ((0 * winheight(0) + 27) / 55)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
argglobal
if bufexists(fnamemodify("~/Programming/Advent_Of_Code_2025/src/main.rs", ":p")) | buffer ~/Programming/Advent_Of_Code_2025/src/main.rs | else | edit ~/Programming/Advent_Of_Code_2025/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Programming/Advent_Of_Code_2025/src/main.rs
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
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 16) / 32)
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
let s:l = 1 - ((0 * winheight(0) + 11) / 22)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
exe 'vert 1resize ' . ((&columns * 119 + 119) / 238)
exe '2resize ' . ((&lines * 32 + 28) / 57)
exe 'vert 2resize ' . ((&columns * 118 + 119) / 238)
exe '3resize ' . ((&lines * 22 + 28) / 57)
exe 'vert 3resize ' . ((&columns * 118 + 119) / 238)
tabnext
edit ~/Programming/Advent_Of_Code_2025/src/day1.rs
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
let s:l = 1 - ((0 * winheight(0) + 27) / 54)
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
balt ~/Programming/Advent_Of_Code_2025/src/day1.rs
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
let s:l = 86 - ((53 * winheight(0) + 27) / 54)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 86
normal! 03|
lcd ~/Programming/Advent_Of_Code_2025
wincmd w
2wincmd w
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
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
