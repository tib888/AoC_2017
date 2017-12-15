import Data.Vector
import qualified Data.Vector as Vector

import Data.Map (Map)
import qualified Data.Map as Map

(|>) :: a -> (a -> b) -> b
(|>) x f = f x
infixl 0 |>

str2int :: String -> Int
str2int str = 
    read str :: Int

execUpdates :: Vector Int -> Int -> Int -> Vector Int
execUpdates array startindex remaining =
    case remaining of
        0 -> array
        _ -> execUpdates (array // [(index, array!index + 1)]) (startindex + 1) (remaining - 1)
    where 
        index = startindex `mod` (Vector.length array)

spread :: Vector Int -> Vector Int
spread numbers = 
    execUpdates maxless (maxindex + 1) max
    where 
        maxindex = Vector.maxIndex numbers
        max = numbers!maxindex
        maxless = numbers // [(maxindex, 0)]        

solver :: Map (Vector Int) Int -> Vector Int -> Int -> (Int, Int)
solver already numbers n =
    case (Map.lookup numbers already) of
        Just last -> (n, n - last)
        Nothing -> solver (Map.insert numbers n already) (spread numbers) (n + 1)             

solve5b :: [Int] -> (Int, Int)
solve5b numbers = 
    solver Map.empty (Vector.fromList numbers) 0

main = 
    "4 1 15 12 0 9 9 5 5 8 7 3 14 5 12 3" 
    |> words 
    |> Prelude.map str2int
    |> solve5b 
    |> show 
    |> putStrLn