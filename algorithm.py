import copy
import queue

isNumbers = False

def isNumber(x: str) -> bool:
	return (x.isdigit() and (x == '0' or x[0] != '0'))

# initialize list of edges in graph from user input
def initialization(graph: dict, used: set, undirected_edges: str, directed_edges: str):
    global isNumbers
    graph.clear()
    used.clear()

    for line in undirected_edges.split('\n'):
        tline = line.split()
        if len(tline) > 1:
            graph[tline[0]] = graph.get(tline[0], []) + [tline[1]]
            graph[tline[1]] = graph.get(tline[1], []) + [tline[0]]
        else:
            if len(tline) > 0:
                graph[tline[0]] = graph.get(tline[0], [])

    for line in directed_edges.split('\n'):
        tline = line.split()
        if len(tline) > 1:
            graph[tline[0]] = graph.get(tline[0], []) + [tline[1]]
        else:
            if len(tline) > 0:
                graph[tline[0]] = graph.get(tline[0], [])

    isNumbers = all([isNumber(i) for i in graph.keys()]) and all([isNumber(j) for i in graph.values() for j in i])
    if isNumbers:
        temp_graph = dict()
        for k, v in graph.items():
            temp_graph[int(k)] = list(map(int, v))
        
        for k, v in temp_graph.items():
            graph.pop(str(k), None)
            graph[k] = v
    
    graph = dict(sorted(graph.items()))
    for i in graph.keys():
        graph[i].sort()


# check all unvisited vertices
def dfs_component(graph: dict, used:set, v):
    used.add(v)
    for i in graph.get(v, []):
        if i not in used:
            dfs_component(graph, used, i)

# find number of components in graph
def component(graph: dict, used: set, undirected_edges: str, directed_edges: str) -> int:
    initialization(graph, used, undirected_edges, directed_edges)
    answer = 0

    for i in graph.keys():
        if i not in used:
            answer += 1
            dfs_component(graph, used, i)

    return answer


def circuit_rank(graph: dict, used: dict, undirected_edges: str, directed_edges: str) -> int:
    # Circuit rank = (number of edges) - (number of vertices) + (number of components)
    # Add (number of edges)
    answer = len(list(filter(lambda x: (x is not None and len(x.split()) >= 2), (undirected_edges + '\n' + directed_edges).split('\n'))))

    # Add (number of components)
    answer += component(graph, used, undirected_edges, directed_edges)

    # Subtract (number of vertices)
    answer -= len(graph.keys())

    return answer

# Degree of vertices
def degree(undirected_edges: str, directed_edges: str) -> dict:
    all_edges = undirected_edges + '\n' + directed_edges
    answer = dict()

    if len(all_edges) > 0:
        # count number of edges that connect to verticel
        for line in all_edges.split('\n'):
            tline = line.split()
            if len(tline) > 1:
                answer[tline[0]] = answer.get(tline[0], 0) + 1
                answer[tline[1]] = answer.get(tline[1], 0) + 1
            else:
                if len(tline) > 0:
                    answer[tline[0]] = answer.get(tline[0], 0)

    # if all vertices is numbers that sort it as numbers
    if all([isNumber(i) for i in answer.keys()]):
        return dict(sorted(answer.items(), key = lambda i: int(i[0])))
    return dict(sorted(answer.items()))


# dfs recursion
def dfs(graph: dict, used: set, search_answer: list, v):
    used.add(v)
    search_answer.append(v)
    for i in graph.get(v, []):
        if i not in used:
            dfs(graph, used, search_answer, i)

# initialize and set start vertex for dfs
def depth_first_search(graph: dict, used: set, start_vertex: str, undirected_edges: str, directed_edges: str) -> list:
    initialization(graph, used, undirected_edges, directed_edges)
    # list that save order of traversing the graph
    search_answer = []
    global isNumbers
    # if all vertices is numbers then start vertex must be number
    if isNumbers == True:
        start_vertex = int(start_vertex)

    dfs(graph, used, search_answer, start_vertex)
    # convert to strings for join output
    if isNumbers == True:
        search_answer = [str(i) for i in search_answer]
    
    return search_answer


def breadth_first_search(graph: dict, used: set, start_vertex: str, undirected_edges: str, directed_edges: str) -> list:
    initialization(graph, used, undirected_edges, directed_edges)
    global isNumbers
    # if all vertices is numbers then start vertex must be number
    if isNumbers == True:
        start_vertex = int(start_vertex)

    search_answer, q = [], queue.Queue()
    q.put(start_vertex)
    used.add(start_vertex)
    while q.qsize() > 0:
        v = q.get()
        # convert to string for join output
        search_answer.append(str(v))
        for i in graph.get(v, []):
            if i not in used:
                q.put(i)
                used.add(i)

    return search_answer


def dfs_Point_Bridge(v):
    global used, GTemp
    used.add(v)
    for j in GTemp[v]:
        if j not in used:
            dfs_Point_Bridge(j)

def ArticulationPoint(undirected_edges, directed_edges):
    global graph, used, GTemp
    CNT_Components = component(undirected_edges, directed_edges)
    
    answer = set()
    for i in graph.keys():
        GTemp = copy.deepcopy(graph)
        used = set()
        GTemp[i] = []
        T = 0
        for j in GTemp.keys():
            if j not in used and i != j:
                dfs_Point_Bridge(j)
                T += 1

        if CNT_Components < T:
            answer.add(i)
    
    global isNumbers
    if isNumbers:
        return sorted(answer, key=lambda x: int(x))
    return sorted(answer)

def Bridges(undirected_edges, directed_edges):
    global graph, used, GTemp, isNumbers
    CNT_Components = component(undirected_edges, directed_edges)

    all_edges = str(undirected_edges) + '\n' + str(directed_edges)
    answer = set()

    if len(all_edges) > 0:
        for line in all_edges.split('\n'):
            tline = line.split()
            if isNumbers:
                tline = list(map(int, tline))
            
            GTemp = copy.deepcopy(graph)
            used = set()
            if len(tline) > 1:
                GTemp[tline[0]].remove(tline[1])
                GTemp[tline[1]].remove(tline[0])

            T = 0
            for j in GTemp.keys():
                if j not in used:
                    dfs_Point_Bridge(j)
                    T += 1

            if CNT_Components < T:
                answer.add(line)
            
    return answer