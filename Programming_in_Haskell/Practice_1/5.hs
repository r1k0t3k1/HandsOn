{-- qsortの定義で、<=を<に置き換えるとどのような影響があるでしょうか？
 -  ヒント: 例として[2,2,3,1,1]を考えてみましょう --}

qsort [] = []
qsort (n:ns) = qsort larger ++ [n] ++ qsort smaller
               where
                 larger = [l | l <- ns, l > n] -- nと同じ値がリスト内に複数あった場合にその値がふるい落とされる
                 smaller = [s | s <- ns, s < n]

main = print (qsort [2,2,3,1,1])
