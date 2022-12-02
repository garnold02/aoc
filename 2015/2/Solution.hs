import Data.List.Extra (splitOn)

solve = do
  input <- readFile "input.txt"
  print $ sum $ map (paper . present) (lines input)
  print $ sum $ map (ribbon . present) (lines input)

area = (2 *) . sum . sides

paper a = area a + minimum (sides a)

sides [a, b, c] = [a * b, a * c, b * c]

present = map read . splitOn "x"

ribbon p = 2 * (sum p - maximum p) + product p