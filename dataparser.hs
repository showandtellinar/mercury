module Dataparser where
import System.IO
import Control.Monad
import Data.Maybe

type Id = Int
data Item = Node Id Int
			| Edge Id Id Id deriving (Show, Eq)

parseitem :: String -> Maybe Item
parseitem s 
	| length sp == 2 = Just $ Node (read(head sp)::Id) (read(last sp)::Int) -- unsafe
	| length sp == 3 = Just $ Edge (read(head sp)::Id) (read(sp!!1)::Id) (read(last sp)::Id) -- unsafe
	| otherwise = Nothing
	where sp = words s

parsefile :: String -> IO [Maybe Item]
parsefile file = do 
	handle <- openFile file ReadMode
	content <- hGetContents handle
	return $ filter (/=Nothing) $ map parseitem $ lines content
