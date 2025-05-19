{-- リスト内包表記一つと上記の関数gridを用いて、大きさnの正方形座標を返す関数
 -  square :: Int -> [(Int, Int)]を定義してください。
 -  ただし、(0,0)から(n,n)の体格の格子は含みません。 --}

grid :: Int -> Int -> [(Int, Int)]
grid a b = [(x,y) | x <- [0..a], y <- [0..b]]

square :: Int -> [(Int, Int)]
square a = [(x,y) | (x,y) <- grid a a, x /= y]
