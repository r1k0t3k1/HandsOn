{-- プレリュード関数tailのように振る舞うsafetail :: [a] -> [a]関数を考えてください。
 -  ただし、tailは空リストを与えるとエラーになりますが、safetailは空リストをエラーとせず、
 -  空リストを返すものとします。関数tail、空リストかどうかを判定する関数null :: [a] -> Bool、
 -  及び以下の其々の使ってsafetailを定義してください。
 -  a 条件式
 -  b ガード付きの等式
 -  c パターンマッチ --}

isNull :: [a] -> Bool
isNull xs = length xs == 0

safetail_a :: [a] -> [a]
safetail_b :: [a] -> [a]
safetail_c :: [a] -> [a]

-- a
safetail_a xs = if isNull xs then [] else tail xs

-- b
safetail_b xs | not (isNull xs) = tail xs 
              | otherwise       = []

-- c
safetail_c []     = []
safetail_c (_:xs) = xs 
