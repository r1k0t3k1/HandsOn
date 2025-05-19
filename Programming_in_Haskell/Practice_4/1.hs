{-- プレリュード関数を使って、長さが偶数のリストを半分ずつに分割する関数
 -  halve :: [a] -> ([a], [a])を定義してください。 --}

halve :: [a] -> ([a], [a])

halve xs | isEven = (take m xs, drop m xs)
  where n = length xs
        isEven = n `mod` 2 == 0
        m = n `div` 2
