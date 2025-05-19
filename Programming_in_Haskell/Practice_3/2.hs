{-- 以下の型を持つ定義を書き下してください。
 -  型が正しい限り、どのように実装してもかまいません。 
--}
bools :: [Bool]
nums :: [[Int]]
add :: Int -> Int -> Int -> Int
copy :: a -> (a,a)
apply :: (a -> b) -> a -> b

bools = [True,True,False,False]
nums = [[1,2],[3,4],[5,6],[7,8]]
add a b c = a + b + c 
copy a = (a,a)
apply f a = f a
