function Node(name, x, y) {
	this.name = name;
	this.x = x;
	this.y = y;
}
Node.prototype.type = function() { return "Node"; };

function Edge(a, b) {
	this.a = a;
	this.b = b;
}
Edge.prototype.type = function() { return "Edge"; };

function Graph(name) {
	this.name = name;
	this.nodes = [];
	this.edges = [];
}
Graph.prototype.type = function() { return "Graph"; };
Graph.prototype.addEdge = function(edge) { 
	this.edges.push(edge);
};
Graph.prototype.addNode = function(node) { 
	this.nodes.push(node);
};

function parse(str) {
	var tokens = str.split(/\n+/); //| +/);
	for (i=0; i < tokens.length; ++i) {
		tokens[i] = tokens[i].trim();
	}
	if (tokens.length == 0) return null;

	var graphname = tokens[0].split(" ")[1];

	var rbb = /^graph \[bb=.*\];$/;
	var rnode = /^.[ \t]*\[height.*$/;
	var redge = /^. --.*$/;

	var graph = new Graph(graphname);

	for (i = 1; i < tokens.length; ++i) {
		if (rbb.test(tokens[i])) {
			var rcoors = /(\d*\.?\d*,){3}(\d*\.\d*)/;
			var coors = rcoors.exec(tokens[i])[0].split(",");
			graph.x0 = parseFloat(coors[0]);
			graph.y0 = parseFloat(coors[1]);
			graph.x1 = parseFloat(coors[2]);
			graph.y1 = parseFloat(coors[3]);
		} else if (rnode.test(tokens[i])) {
			var line = tokens[i] + tokens[i+1] + tokens[i+2];

			var name = line[0];
			var rpos = /pos=".*"/;
			var pos = rpos.exec(line)[0];
			var coors = pos.substr(5);
			coors = coors.substr(0, coors.length-1);
			coors = coors.split(",");
			
			var node = new Node(name, parseFloat(coors[0]), parseFloat(coors[1]));
			graph.addNode(node);

			// Skip the next two lines
			i += 2;
			continue;
		} else if (redge.test(tokens[i])) {
			var redge = /^. -- ./;
			var ab = redge.exec(tokens[i])[0].split(" -- ");
			var edge = new Edge(ab[0], ab[1]);
			graph.addEdge(edge);
		}
		
	}
	return graph;
}
