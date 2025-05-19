{-- 他のプレリュード関数や演算子を使わずに、論理積&&に対する以下の定義を条件式を用いて形式化してください
 -  True && True = True
 -  _    && _    = False
--}

test1 :: Bool -> Bool -> Bool
test1 a b = if a then if b then True else False else False

{-- 以下についても同様のことをしてください。
 -  必要になる条件式の個数が異なることに注意しましょう
 -  True  && b = b
 -  False && _ = False
--}
--
test2 :: Bool -> Bool -> Bool
test2 a b = if a then b else False
