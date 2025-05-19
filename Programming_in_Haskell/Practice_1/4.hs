{-- リストを降順に整列するように関数qsortの定義を変えるにはどうすればよいでしょうか？ --}

qsort [] = []
qsort (n:ns) = qsort larger ++ [n] ++ qsort smaller
               where
                 larger = [l | l <- ns, l >= n]
                 smaller = [s | s <- ns, s <= n]

main = print (qsort [5,3,7,2,9])
