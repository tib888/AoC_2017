import System.IO

data State = Group | Trash | Escape

process_ch :: ([State], Int, Int) -> Char -> ([State], Int, Int)
process_ch (state, sum, ntrash) ch =
    case head state of
        Group -> case ch of
                    '!' -> (Escape:state, sum, ntrash)
                    '{' -> (Group:state, sum, ntrash)
                    '}' -> (tail state, sum + length state - 1, ntrash)
                    '<' -> (Trash:state, sum, ntrash)
                    _ -> (state, sum, ntrash)
        Trash -> case ch of 
                    '!' -> (Escape:state, sum, ntrash)
                    '>' -> (tail state, sum, ntrash)
                    _ -> (state, sum, ntrash + 1)
        Escape -> (tail state, sum, ntrash)

process_stream :: String -> ([State], Int, Int)
process_stream stream = 
    foldl process_ch ([Group], 0 :: Int, 0 :: Int) stream
        
main = do 
    x <- readFile "day9.txt"
    let (a,b,c) = process_stream x
    putStrLn $ show $ (b,c)
