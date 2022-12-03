import Data.Char (ord)
import Data.List (intersect, nub)

solve = do
  input <- readFile "input.txt"
  print $ sum $ map (sum . map val . common) (lines input)
  print $ sum $ map (sum . map val . gcommon) (groups $ lines input)

common a = nub $ take b a `intersect` drop b a
  where
    b = length a `div` 2

gcommon [a, b, c] = nub $ (a `intersect` b) `intersect` (b `intersect` c)

val a
  | a < '[' = ord a - ord 'A' + 27
  | otherwise = ord a - ord 'a' + 1

groups [] = []
groups (a : b : c : d) = [a, b, c] : groups d