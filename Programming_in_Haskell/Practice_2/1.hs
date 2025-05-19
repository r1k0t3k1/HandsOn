{-- この章の例題をGHCiを用いて実行してください --}

double x = x + x
quadruple x = double (double x)

factorial n = product [1..n]
average ns = sum ns `div` length ns
