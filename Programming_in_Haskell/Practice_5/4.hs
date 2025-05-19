{-- ある要素のみからなるリストを生成するプレリュード関数
 -  replicate :: Int -> a -> [a]を、関数lengthと同じ要領でリスト内包表記を用いて定義してください--}

replicate :: Int -> a -> [a]
replicate i a = [a | _ <- [1..i]]
