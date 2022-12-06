import Data.List (nub, transpose)

solve = do
  input <- readFile "input.txt"
  print $ 4 + fst (head $ filter unique $ zip [0 ..] $ chunk 4 input)
  print $ 14 + fst (head $ filter unique $ zip [0 ..] $ chunk 14 input)

unique a = nub (snd a) == snd a

chunk n = takeWhile ((== n) . length) . transpose . take n . iterate tail