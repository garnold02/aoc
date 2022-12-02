import Data.Char (ord)

solve = do
  input <- readFile "input.txt"
  print $ sum $ map roundScore (lines input)
  print $ sum $ map roundScore2 (lines input)

roundScore round
  | head round == 'A' && last round == 'X' = 3 + 1
  | head round == 'A' && last round == 'Y' = 6 + 2
  | head round == 'A' && last round == 'Z' = 0 + 3
  | head round == 'B' && last round == 'X' = 0 + 1
  | head round == 'B' && last round == 'Y' = 3 + 2
  | head round == 'B' && last round == 'Z' = 6 + 3
  | head round == 'C' && last round == 'X' = 6 + 1
  | head round == 'C' && last round == 'Y' = 0 + 2
  | head round == 'C' && last round == 'Z' = 3 + 3

roundScore2 round
  | head round == 'A' && last round == 'X' = 0 + 3
  | head round == 'A' && last round == 'Y' = 3 + 1
  | head round == 'A' && last round == 'Z' = 6 + 2
  | head round == 'B' && last round == 'X' = 0 + 1
  | head round == 'B' && last round == 'Y' = 3 + 2
  | head round == 'B' && last round == 'Z' = 6 + 3
  | head round == 'C' && last round == 'X' = 0 + 2
  | head round == 'C' && last round == 'Y' = 3 + 3
  | head round == 'C' && last round == 'Z' = 6 + 1

ords :: String -> [Int]
ords = map ord