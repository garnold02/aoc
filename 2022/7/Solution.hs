import Data.Char (isDigit)
import Data.List (concat, isPrefixOf)
import Data.List.Split (splitOn)

solve = do
  input <- readFile "input.txt"
  print $ filteredSize $ root input
  print $ dirToDelete $ root input
  where
    root = snd . parseDir . parseCmds . lines

data Cmd = Cd CdPath | Ls [LsEntry]

data CdPath = Up | Down String

data LsEntry = LsDir String | LsFile Int String

data Dir = DirDir String [File] [Dir]

data File = DirFile Int String

parseCmds [] = []
parseCmds input = cmd : parseCmds rest
  where
    (rest, cmd) = parseCmd input

    parseCmd input = case cmd of
      "cd" -> parseCd input
      "ls" -> parseLs input
      where
        cmd = head $ tail $ splitOn " " (head input)

    parseCd input = do
      (tail input, Cd cdPath)
      where
        cdPath = case last $ splitOn " " (head input) of
          ".." -> Up
          name -> Down name

    parseLs input = (rest, Ls (foldl lsEntry [] relevant))
      where
        relevant = reverse $ takeWhile isLsEntry (tail input)
        rest = drop (length relevant) (tail input)

        isLsEntry line = isDirLine line || isFileLine line
        isDirLine = isPrefixOf "dir"
        isFileLine = isDigit . head

        lsEntry entries line
          | first == "dir" = LsDir last : entries
          | otherwise = LsFile (read first) last : entries
          where
            [first, last] = splitOn " " line

parseDir cmds = (finalRest, DirDir name (dirFiles entries) (reverse dirs))
  where
    (Cd (Down name)) : (Ls entries) : rest = cmds
    (finalRest, dirs) = dirDirs rest []

    dirFiles = foldl dirFile []

    dirFile files (LsFile size name) = DirFile size name : files
    dirFile files _ = files

    dirDirs cmds@(Cd (Down _) : _) dirs = (nextRest, nextDirs)
      where
        (rest, dir) = parseDir cmds
        (nextRest, nextDirs) = dirDirs rest (dir : dirs)
    dirDirs (Cd Up : rest) dirs = (rest, dirs)
    dirDirs [] dirs = ([], dirs)

filteredSize dir@(DirDir _ _ dirs)
  | mySize <= 100000 = sizeOfSubdirs + mySize
  | otherwise = sizeOfSubdirs
  where
    mySize = dirSize dir
    sizeOfSubdirs = sum $ map filteredSize dirs

dirToDelete dir = minimum $ atLeast (neededSpace dir) dir
  where
    neededSpace dir = 30000000 - freeSpace
      where
        freeSpace = 70000000 - dirSize dir

    atLeast size dir@(DirDir _ _ dirs)
      | mySize >= size = mySize : subdirs
      | otherwise = subdirs
      where
        mySize = dirSize dir
        subdirs = concatMap (atLeast size) dirs

dirSize (DirDir _ files dirs) = sizeOfFiles + sizeOfDirs
  where
    sizeOfFiles = sum $ map fileSize files
    sizeOfDirs = sum $ map dirSize dirs
    fileSize (DirFile size _) = size