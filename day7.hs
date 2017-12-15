import System.Exit

import Data.Maybe

import Data.List
import qualified Data.List as List

import Data.Set
import qualified Data.Set as Set

import Data.Map (Map)
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

inputFile = endBy line eol

child = do
    many (oneOf " \t")
    name <- many (noneOf " \t,\r\n")
    return name

children = do
    (string "->")
    list <- sepBy child (char ',')
    return list

nochild = do
    return []

disk = try children
        <|> nochild

line = do 
    name <- many (noneOf " \t\n\r,()")
    many (oneOf " \t")
    (string "(")
    weight <- many (noneOf " \t\n\r,()")
    (string ")")
    many (oneOf " \t")    
    children <- disk
    return (name, (str2int weight, children))

getchildren mydata =
    snd (snd mydata)    

findroot input =
    Data.Maybe.fromJust $ List.find (\x -> not $ Set.member x useds) (Prelude.map fst input)
    where 
        useds = Set.fromList $ List.concat $ Prelude.map getchildren input

sumweights input root = 
    case Map.lookup root input of
        Just (weight, children) -> weight + (sum $ Prelude.map (sumweights input) children)
        Nothing -> 0

decorated input root =
    (root, sumweights input root)

findunballanced input root = 
    childweights
    where
        (weight, children) = Data.Maybe.fromJust $ Map.lookup root input
        childweights = List.map (decorated input) children


main = do
    result <- parseFromFile inputFile "day7.txt"    
    case result of
        Left err -> print err
        Right xs -> print $ show $ findunballanced (Map.fromList xs) "sphbbz" --(findroot xs)
    
    