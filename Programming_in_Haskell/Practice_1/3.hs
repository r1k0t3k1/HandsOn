{-- 数値のリストに対し席を計算する関数productを定義し、
 -  product [2,3,4] = 24 となることを示してください --}

productfunction [] = 1
productfunction (n:ns) = n * productfunction ns

main = print (productfunction [2,3,4])
