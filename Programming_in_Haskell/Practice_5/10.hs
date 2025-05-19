{-- シーザー暗号のプログラムを変更して大文字も扱えるようにしてください。 --}
import Data.Char

find :: Eq a => a -> [(a,b)] -> [b]
find k t = [v | (k',v) <- t, k' == k]

positions :: Eq a => a -> [a] -> [Int]
positions x xs = find x (zip xs [0..])

let2int :: Char -> Int
let2int c | isLower c = ord c - ord 'a'
          | isUpper c = ord c - ord 'A' + 26
          | otherwise = ord c

int2let :: Int -> Char
int2let n | 0 <= n && n <= 25 = chr (ord 'a' + n)
          | 26 <= n && n <= 51 = chr (ord ('A') + n - 26)
          | otherwise = chr n

shift :: Int -> Char -> Char
shift n c | isAlpha c = int2let ((let2int c + n) `mod` 52)
          | otherwise = c

encode :: Int -> String -> String
encode n s = [shift n c | c <- s]

table :: [Float]
table = [
  8.1, 1.5, 2.8, 4.2, 12.7, 
  2.2, 2.0, 6.1, 7.0, 0.2,
  0.8, 4.0, 2.4, 6.7, 7.5,
  1.9, 0.1, 6.0, 6.3, 9.0,
  2.8, 1.0, 2.4, 0.2, 2.0,
  0.1]

lowers :: String -> Int
lowers xs = length [x | x <- xs, x >= 'a' && x <= 'z']

count :: Char -> String -> Int
count x xs = length [x' | x' <- xs, x == x']

percent :: Int -> Int -> Float
percent n m = (fromIntegral n / fromIntegral m) * 100

freqs :: String -> [Float]
freqs xs = [percent (count x xs) n | x <- ['a'..'z']]
           where n = lowers xs

chisqr :: [Float] -> [Float] -> Float
chisqr os es = sum [((o-e)^2)/e | (o,e) <- zip os es]

rotate :: Int -> [a] -> [a]
rotate n xs = drop n xs ++ take n xs

crack :: String -> String
crack xs = encode (-factor) xs
           where
             factor = head (positions (minimum chitab) chitab)
             chitab = [chisqr (rotate n table') table | n <- [0..25]]
             table' = freqs xs
