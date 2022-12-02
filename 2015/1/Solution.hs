solve = do
  input <- readFile "input.txt"
  print $ up input - down input
  print $ neg input

up = length . filter (== '(')

down = length . filter (== ')')

neg list = length list - acc list 0
  where
    acc a (-1) = length a
    acc (a : b) floor
      | a == '(' = acc b (floor + 1)
      | a == ')' = acc b (floor - 1)
