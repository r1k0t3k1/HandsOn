{-- 関数positionsを関数findを使って再定義してください --}

find :: Eq a => a -> [(a,b)] -> [b]
find k t = [v | (k',v) <- t, k' == k] 

positions :: Eq a => a -> [a] -> [Int]
positions x xs = find x (zip xs [0..])
