{-- プレリュード関数initは、空でないリストから最後の要素を取り除きます。
 - 例えば、init [1,2,3,4,5] = [1,2,3,4]です。
 - 関数initの定義を 2通り示してください。
 - --}

init2 xs = take (length xs - 1) xs
init3 xs = reverse (drop 1 (reverse xs))
