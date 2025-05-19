{-- Luhnアルゴリズムは、銀行のカード番号に対して単純な入力間違いを検出する方法であり、以下のように実行されます。
 -  - 其々を独立した番号だとみなす
 -  - 右から数えて偶数番目の数すべてを二倍にする
 -  - それぞれの数が9より大きいなら9を引く
 -  - すべての数を足し合わせる
 -  - 合計が10で割り切れるなら、カードの番号は正しい
 -
 -  数を2倍にして、もしその結果が9より大きいなら9を引く関数luhnDouble :: Int -> Int
 -  を定義してください。
 -
 -  luhnDoubleと整数の剰余を求める関数modを使って、4桁の銀行のカード番号がただし以下どうかを
 -  判定する関数luhn :: Int -> Int -> Int -> Int -> Boolを定義してください。
--}

luhnDouble :: Int -> Int
luhnDouble n = if n2 > 9 then n2 - 9 else n2 
  where n2 = n * 2

luhn :: Int -> Int -> Int -> Int -> Bool
luhn a b c d = ((luhnDouble a)+b+(luhnDouble c)+d) `mod` 10 == 0
