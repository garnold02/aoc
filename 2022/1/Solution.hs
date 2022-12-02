import Data.List.Extra (sort, splitOn)

solve = do
  input <- readFile "input.txt"
  print $ maximum $ parse input
  print $ sum $ take 3 $ revSort input

revSort = reverse . sort . parse

parse = map toInts . toElves

toInts = sum . map read . splitOn "\n"

toElves = splitOn "\n\n"