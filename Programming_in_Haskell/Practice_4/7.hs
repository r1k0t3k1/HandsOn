{-- 以下のカリー化された関数の定義の意味をラムダ式を用いて形式化してください。
 -  mult :: Int -> Int -> Int -> Int
 -  mult x y z = x*y*z
 --}

mult :: Int -> Int -> Int -> Int
mult = \x -> (\y -> (\z -> x*y*z))
