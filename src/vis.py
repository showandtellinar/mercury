import json
import requests
import urllib
import pickle
import time
import sys

def tx_io(tx_json):
    """ Splits a tranaction into inputs and outputs """
    io = { 'inputs':[], 'out':[]}
    for i in tx_json['inputs']:
        if 'prev_out' in i:
            io['inputs'].append(i['prev_out']['addr'])
    for o in tx_json['out']:
        if 'addr' in o:
            io['out'].append(o['addr'])
    return io

def addr_tx(addr_json):
    """ Retuns a list of transaction hashes for an address """
    return [ tx['hash'] for tx in addr_json['txs'] ]

def request_addr(addr):
    """ Returns a request object for a bitcoin address """
    a_r = requests.get("http://blockchain.info/address/{}?format=json".format(addr))
    return a_r

def request_tx(tx):
    """ Returns a request object for a bitcoin transaction """
    tx_r = requests.get("http://blockchain.info/tx-index/{}?format=json&scripts=true".format(tx))
    return tx_r

def explore_addr(addr, depth, record):
    """ DFS from an address. Discovered addresses are added to record """
    time.sleep(0.2)
    if depth == 0: return None
    print("addr={}\tdepth={}".format(addr, depth))
    if addr not in record: record[addr] = [depth, {}]
    addr_r = request_addr(addr)
    try: addr_tx_ids = addr_tx(addr_r.json())
    except: return None
    for tx_id in addr_tx_ids:
        tx_r = request_tx(tx_id)
        try: tx_ios = tx_io(tx_r.json())
        except: continue
        # only interested in outputs for now
        for tx_o in tx_ios['out']:
            if tx_o in record[addr][1]:
                (record[addr][1])[tx_o] += 1
            else:
                (record[addr][1])[tx_o] = 1
            if tx_o not in record:
                explore_addr(tx_o, depth-1, record)

def from_file(filename):
    """ Returns the object from a pickle file """
    f = open(filename, 'rb')
    r = pickle.load(f)
    f.close()
    return r

depth = int(sys.argv[1]) # How far to go in the DFS. It expands pretty rapidly. 5 takes a while
record = {}
initial_address = "1FHKW7smZSsLPS7BCPuWwpMWSyRwsfabcp"


# DFS or read from pickled file?
if True:
    explore_addr(initial_address, depth, record)
    print("\n\nWriting record to file\n")
    f = open("record_str{}.txt".format(depth), 'wb')
    pickle.dump(record, f)
    f.close()
else:
    record = from_file("record_str{}.txt".format(depth))
    print(len(record))

def export_gexf(record):
    """ Create a gexf file from a record """
    f = open("record{}.gexf".format(depth), 'w')
    f.write('<?xml version="1.0" encoding="UTF-8"?>\n')
    f.write('<gexf xmlns="http://www.gexf.net/1.2draft" version="1.2">\n')
    f.write('\t<graph mode="static" defaultedgetype="directed">\n')
    # write attributes
    f.write('\t\t<attributes class="node" type="static">\n')
    f.write('\t\t\t<attribute id="0" title="depth" type="integer"/>\n')
    f.write('\t\t</attributes>\n')
    # write nodes
    f.write('\t\t<nodes>\n')
    nodes = set()
    for key,values in record.items():
        nodes.add(key)
        for value in values[1]:
            nodes.add(value)
    node_map = {}
    ncount = 0
    for node in nodes:
        f.write('\t\t\t<node id="{}">\n'.format(ncount))
        f.write('\t\t\t\t<attvalues>\n')
        if node in record:
            f.write('\t\t\t\t\t<attvalue id="0" value="{}"/>\n'.format(depth-record[node][0]))
        else:
            f.write('\t\t\t\t\t<attvalue id="0" value="{}"/>\n'.format(depth))
        f.write('\t\t\t\t</attvalues>\n')
        f.write('\t\t\t</node>\n')
        node_map[node] = ncount
        ncount += 1
    f.write('\t\t</nodes>\n')
    f.write('\t\t<edges>\n')
    ecount = 0
    for key,values in record.items():
        vdict = values[1]
        keyi = node_map[key]
        for value in vdict:
            keyv = node_map[value]
            f.write('\t\t\t<edge id="{}" source="{}" target="{}" />\n'.format(ecount, keyi, keyv))
            ecount += 1
    f.write('\t\t</edges>\n')
    f.write('\t</graph>\n')
    f.write('</gexf>\n')

def construct_graph(record):
    G = nx.Graph()
    nodes = set()
    for key,values in record.items():
        nodes.add(key)
        for value in values:
            nodes.add(value)
    for node in nodes:
        G.add_node(node)
    for key,values in record.items():
        for node,times in values.items():
            for _ in range(times):
                G.add_edge(key, node)
    return G

print("\n\nCreating gexf\n")
export_gexf(record)

#print("\n\nCreating graph\n")
#G = construct_graph(record)
#print("\n\nDrawing graph\n")
#nx.draw_spring(G)
#nx.draw(G)
#print("\n\nShowing graph\n")
#plt.show()
