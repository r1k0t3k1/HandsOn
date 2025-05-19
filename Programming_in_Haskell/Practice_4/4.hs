{-- 論理積演算子 &&と同様に、パターンマッチを使って論理和演算子 ||を四通りの方法で定義してください。 --}

(||) :: Bool -> Bool -> Bool
False || False = False 
False || True  = True
True  || False = True 
True  || True = True 
