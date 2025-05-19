{-- 自分自身を除く約数の和が自分自身と等しいとき、
 -  その整数を完全数と呼びます。与えられた上限までに含まれる完全数すべてを
 -  算出する関数perfects :: Int -> [Int]を、リスト内包表記をと関数factorsを使って定義してください
 --}

factors :: Int -> [Int]
factors  a = [x | x <- [1..a], a `mod` x == 0]

perfects :: Int -> [Int]
perfects a = [x | x <- [1..a], sum (factors x) == x * 2]
