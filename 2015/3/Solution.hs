import Data.List (nub)

solve = do
  input <- readFile "input.txt"
  print $ length $ nub $ journey input
  print $ length $ nub $ journey2 $ sep input

journey = foldl move [(0, 0)]

journey2 (a, b) = journey a ++ journey b

move (a : b) c = go a c : a : b

go (x, y) d
  | d == '^' = (x, y + 1)
  | d == 'v' = (x, y - 1)
  | d == '<' = (x - 1, y)
  | d == '>' = (x + 1, y)

sep [] = ([], [])
sep (a : b : c) = (a : d, b : e) where (d, e) = sep c