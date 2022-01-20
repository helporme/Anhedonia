let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd /mnt/f/Anhedonia/iteration0
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
badd +15 ecs_mvp/src/main.rs
badd +209 ecs_mvp/src/archetype.rs
badd +0 term:///mnt/f/Anhedonia/iteration0/ecs_mvp//384:/bin/bash
argglobal
%argdel
$argadd ecs_mvp
edit ecs_mvp/src/archetype.rs
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
exe 'vert 1resize ' . ((&columns * 141 + 106) / 213)
exe 'vert 2resize ' . ((&columns * 71 + 106) / 213)
argglobal
balt ecs_mvp/src/main.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 210 - ((11 * winheight(0) + 23) / 47)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 210
normal! 015|
lcd /mnt/f/Anhedonia/iteration0/ecs_mvp
wincmd w
argglobal
if bufexists("term:///mnt/f/Anhedonia/iteration0/ecs_mvp//384:/bin/bash") | buffer term:///mnt/f/Anhedonia/iteration0/ecs_mvp//384:/bin/bash | else | edit term:///mnt/f/Anhedonia/iteration0/ecs_mvp//384:/bin/bash | endif
if &buftype ==# 'terminal'
  silent file term:///mnt/f/Anhedonia/iteration0/ecs_mvp//384:/bin/bash
endif
balt /mnt/f/Anhedonia/iteration0/ecs_mvp/src/main.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 908 - ((46 * winheight(0) + 23) / 47)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 908
normal! 0
lcd /mnt/f/Anhedonia/iteration0/ecs_mvp
wincmd w
exe 'vert 1resize ' . ((&columns * 141 + 106) / 213)
exe 'vert 2resize ' . ((&columns * 71 + 106) / 213)
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0&& getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOFI
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
