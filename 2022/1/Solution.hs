import Data.List.Extra (sort, splitOn)

solve = do
  input <- readFile "input.txt"
  print $ partOne input
  print $ partTwo input

partOne = maximum . parse

partTwo = sum . take 3 . revSort

revSort = reverse . sort . parse

parse = map toInts . toElves

toInts = sum . map read . splitOn "\n"

toElves = splitOn "\n\n"