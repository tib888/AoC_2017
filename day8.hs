import System.Exit

import Data.Maybe

import Data.List
import qualified Data.List as List

import Data.Set
import qualified Data.Set as Set

import Data.Map
import qualified Data.Map as Map

import Text.ParserCombinators.Parsec

(|>) :: a -> (a -> b) -> b
(|>) x f = f x
infixl 0 |>

str2int :: String -> Int
str2int str = 
    read str :: Int

eol =   try (string "\n\r")
    <|> try (string "\r\n")
    <|> string "\n"
    <|> string "\r"
    <?> "end of line"

sign = 
    (string "inc" >> return (1 :: Int))
    <|> (string "dec" >> return ((-1) :: Int))

condition =    
    try (string ">=" >> return (>=))
    <|> try (string "<=" >> return (<=))
    <|> try (string ">" >> return (>))
    <|> try (string "<" >> return (<))
    <|> try (string "==" >> return (==))
    <|> (string "!=" >> return (/=))

inputFile = endBy line eol

line = do 
    target <- many (noneOf " \t")
    many (oneOf " \t")
    mul <- sign
    many (oneOf " \t")
    val <- many (noneOf " \t")
    many (oneOf " \t")
    (string "if")
    many (oneOf " \t")
    source <- many (noneOf " \t")
    many (oneOf " \t")
    predicate <- condition
    many (oneOf " \t")
    condval <- many (noneOf " \t\n\r")
    return (predicate, source, (str2int condval), target, (mul * (str2int val)))

exec k v m =
    (newval, newmax)
    where 
        (lastval, lastmax) = (Map.findWithDefault (0, 0) k m)
        newval = lastval + v 
        newmax = max lastmax newval
     
execute registers item =
    case predicate (fst (Map.findWithDefault (0, 0) source registers)) condval of
        False -> registers
        True -> Map.insert target newval registers
    where 
        (predicate, source, condval, target, diff) = item
        newval = exec target diff registers

cmp1 a b =
    compare (fst (snd a)) (fst (snd b))

cmp2 a b =
    compare (snd (snd a)) (snd (snd b))     

firsttask xs = 
    List.maximumBy cmp1 $ Map.toList $ List.foldl execute Map.empty xs

secondtask xs =     
    List.maximumBy cmp2 $ Map.toList $ List.foldl execute Map.empty xs

main = do
    result <- parseFromFile inputFile "day8.txt"    
    case result of
        Left err -> print err
        Right xs -> putStrLn $ (show (firsttask xs)) ++ (show (secondtask xs))
    
