module Force where
import Dataparser

data Point = Point { xcoor,ycoor :: Float} deriving (Show, Eq)
data Quadtree = Quadtree { 	tl :: Quadtree,
							tr :: Quadtree,	
							bl :: Quadtree,	
							br :: Quadtree,
							xline :: Float,
							yline :: Float,
						    center :: Point,
							mass :: Float,
							numPoints :: Int,
							maxDepth :: Int }
				| Leaf { leafPoints :: [Point],
						 center :: Point,
						 mass :: Float }
				 | Nil

emptyQuad :: Quadtree
emptyQuad = Quadtree Nil Nil Nil Nil zero 0.0 0 3
	where zero = Point 0.0 0.0

add :: Quadtree -> Point -> Quadtree
add Nil p = add emptyQuad p
add (Leaf leafPoints center mass) p = 
	Leaf (p : leafPoints) (center') (mass+1)
	where 
          x = (l*(xcoor center) + (xcoor p)) / (1.0+l)
          y = (l*(ycoor center) + (ycoor p)) / (1.0+l)
          l = fromIntegral(length leafPoints) :: Float
          center' = Point x y
add (Quadtree tl tr bl br center mass numPoints maxDepth) p =
