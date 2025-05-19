{-- 長さがnである整数のリストxsとysの内積は、対応する要素の積の和として計算できます。
 -  ２つのリストから内積を計算する関数scalarproduct :: [Int] -> [Int] -> Intを、
 -  関数chisqrと同じようにリスト内包表記を使って定義できることを示してください。
--}

scalarproduct :: [Int] -> [Int] -> Int
scalarproduct xs ys = sum [x*y | (x,y) <- zip xs ys] 
