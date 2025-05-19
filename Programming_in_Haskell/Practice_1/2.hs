{-- xの値によらずsum [x] = xであることを示してください --}
sumfunction [] = 0
sumfunction (n:ns) = n + sumfunction ns

main = print (sumfunction [1, 2, 3])
