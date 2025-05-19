{-- 以下の関数の型はなんでしょう？
 -  second xs = head (tail xs)
 -  swap (x,y) = (y,x)
 -  pair x y = (x,y)
 -  double x = x*2
 -  palindrome xs = reverse xs == xs
 -  twice f x = f (f x)
 -  --}

second :: [a] -> a
swap :: (a,b) -> (b,a)
pair :: a -> b -> (a,b)
double :: Num a -> a -> a
palindrome :: Eq a -> [a] -> Bool
twice :: (a -> a) -> a -> a 
