{-- リストの３つ目の要素を返す関数third :: [a] -> a を、以下を使って其々定義してください。
 -  ただし、リストには３つ以上の要素が格納されているとします。
 -  a headとtail
 -  b リストのインデックス演算子 !!
 -  c パターンマッチ
--}

-- a
third_a :: [a] -> a
third_a xs | length xs == 3 = head (tail (tail xs))

-- b
third_b :: [a] -> a
third_b xs | length xs == 3 = xs !! 2

-- c
third_c :: [a] -> a
third_c (_:_:x:_) = x
