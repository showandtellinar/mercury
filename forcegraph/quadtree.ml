open List;;

type point = { x : float; y : float };;
type box = { p0 : point; p1 : point};;

type quadtree =
    | Leaf of point list * box
    | Node of quadnode
and quadnode = {
    mass : float;
    level : int;
    bounds : box;
    tl : quadtree;
    tr : quadtree;
    bl : quadtree;
    br : quadtree;
};;

let capacity = 3;;

let box_center (b:box) =
    { x = (b.p1.x -. b.p0.x) /. 2.0;
      y = (b.p0.y -. b.p1.y) /. 2.0 }
;;


let quad_center l = match l with
    | Leaf (plist,_) -> 
            let xsum, ysum = List.fold_left 
                (fun (xc, yc) p -> (xc +. p.x, yc +. p.y) )
                (0.0,0.0) plist in
            let xavg = xsum /. (float_of_int (List.length plist)) in
            let yavg = ysum /. (float_of_int (List.length plist)) in
            { x=xavg; y=yavg }
    | Node n -> box_center n.bounds
;;

let empty_leaf b = Leaf ([], b);;

let rec split_leaf l =  match l with
    | Leaf (plist, b) ->
        let lc = box_center b in
        let tlb = { p0 = b.p0; p1 = lc } in
        let trb = { p0 = {b.p0 with x=b.p1.x}; p1 = {lc with x=b.p1.x} } in
        let blb = { p0 = {tlb.p0 with y=lc.y}; p1 = {lc with y=b.p1.y} } in
        let brb = { p0 = lc; p1 = b.p1 } in
        let tll = Leaf ([], tlb) in
        let trl = Leaf ([], trb) in
        let bll = Leaf ([], blb) in
        let brl = Leaf ([], brb) in
        let node = Node { mass = 0.0; 
                          level = 0; 
                          bounds = b; 
                          tl = tll; 
                          tr = trl; 
                          bl = bll; 
                          br = brl; } in
        List.fold_left (fun acc p -> add acc p) node plist
    | _ -> failwith "not a leaf"
and add q p = match q with
    | Leaf (l,b) -> 
        if List.length l >= capacity then
            split_leaf (Leaf ((p :: l),b))
        else
            Leaf ((p :: l),b)
    | Node n -> 
        let center = box_center n.bounds in
        let xside = center.x < p.x in
        let yside = center.y < p.y in
        match (xside, yside) with
        | (true, true) -> Node { n with tr=(add n.tr p) }
        | (true, false) -> Node { n with br=(add n.br p) }
        | (false, false) -> Node { n with bl=(add n.bl p) }
        | (false, true) -> Node {n with tl=(add n.tl p)}
;;

let print_point p = 
    print_string "(";
    print_float p.x;
    print_string ", ";
    print_float p.y;
    print_string ") "
;;

let rec print_quadtree q = match q with
    | Leaf (plist, _) -> 
            List.iter (fun p -> print_point p) plist;
            print_endline "";
    | Node n -> 
            print_endline "in quadnode";
            print_endline "printing tl";
            print_quadtree n.tl;
            print_endline "printing tr";
            print_quadtree n.tr;
            print_endline "printing bl";
            print_quadtree n.bl;
            print_endline "printing br";
            print_quadtree n.br;
;;

let qtree = empty_leaf { p0 = {x=0.0; y=10.0}; p1 = {x=10.0; y=0.0} };;
let points = [ {x=1.0; y=1.0}; 
               {x=2.0; y=2.0}; 
               {x=1.0; y=2.0}; 
               {x=9.0; y=2.0}; 
               {x=9.0; y=9.0}; ];;
let qtree = List.fold_left (fun qtree p -> add qtree p) qtree points;;
print_quadtree qtree;;
