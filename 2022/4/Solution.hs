import Data.List.Extra (splitOn)

solve = do
  input <- readFile "input.txt"
  select contains input
  select overlaps input

select a b = print $ length $ filter a $ map ranges $ lines b

ranges a = map (map (read :: String -> Int) . splitOn "-") $ splitOn "," a

contains [[a, b], [c, d]]
  | c >= a && d <= b = True
  | a >= c && b <= d = True
  | otherwise = False

overlaps [[a, b], [c, d]]
  | d >= a && c <= b = True
  | b >= c && a <= d = True
  | otherwise = False
