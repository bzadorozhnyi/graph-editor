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

# find number of components in graph
def component(graph: dict, used: set, undirected_edges: str, directed_edges: str) -> int:
    initialization(graph, used, undirected_edges, directed_edges)
    
    # check all unvisited vertices
    def dfs_component(v):
        used.add(v)
        for i in graph.get(v, []):
            if i not in used:
                dfs_component(i)

    answer = 0
    for i in graph.keys():
        if i not in used:
            answer += 1
            dfs_component(i)

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


# initialize and set start vertex for dfs
def depth_first_search(graph: dict, used: set, start_vertex: str, undirected_edges: str, directed_edges: str) -> list:
    initialization(graph, used, undirected_edges, directed_edges)
    
    # dfs recursion
    def dfs(v):
        used.add(v)
        search_answer.append(v)
        for i in graph.get(v, []):
            if i not in used:
                dfs(i)

    # list that save order of traversing the graph
    search_answer = []
    global isNumbers
    # if all vertices is numbers then start vertex must be number
    if isNumbers == True:
        start_vertex = int(start_vertex)

    dfs(start_vertex)
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


# def dfs_Point_Bridge(v):
#     global used, GTemp
#     used.add(v)
#     for j in GTemp[v]:
#         if j not in used:
#             dfs_Point_Bridge(j)

# def ArticulationPoint(undirected_edges, directed_edges):
#     global graph, used, GTemp
#     CNT_Components = component(undirected_edges, directed_edges)
    
#     answer = set()
#     for i in graph.keys():
#         GTemp = copy.deepcopy(graph)
#         used = set()
#         GTemp[i] = []
#         T = 0
#         for j in GTemp.keys():
#             if j not in used and i != j:
#                 dfs_Point_Bridge(j)
#                 T += 1

#         if CNT_Components < T:
#             answer.add(i)
    
#     global isNumbers
#     if isNumbers:
#         return sorted(answer, key=lambda x: int(x))
#     return sorted(answer)


# auxiliary class for bridges and cutpoints search
class node():
    def __init__(self, time_in: int, up: int):
        self.time_in = time_in # time that reach vertex
        self.up = up # auxiliary variable on the basics of which we will find the answer

def cutpoints(graph: dict, used: set, undirected_edges: str, directed_edges: str) -> list:
    initialization(graph, used, undirected_edges, directed_edges)
    def dfs_cutpoints(v, ancestor: int, time_counter: int):
        used.add(v)
        dp[v] = dp.get(v, node(time_counter, time_counter))
        children = 0
        for to in graph.get(v, []):
            if to == ancestor:
                continue
            if to in used:
                dp[v].up = min(dp[v].up, dp[to].time_in)
            else:
                dfs_cutpoints(to, v, time_counter + 1)
                dp[v].up = min(dp[v].up, dp[to].up)
                if dp[to].up >= dp[v].time_in and ancestor != -1:
                    search_answer.append(v)
                children += 1
        if ancestor == -1 and children > 1:
            search_answer.append(v)

    dp, search_answer = dict(), []
    # go through all components
    for i in graph.keys():
        if i not in used:
            dfs_cutpoints(i, -1, 0)
    
    if isNumber:
        return sorted(search_answer, key=lambda x: int(x))
    return sorted(search_answer)

def bridges(graph: dict, used: set, undirected_edges: str, directed_edges: str) -> list:
    def dfs_bridges(v, ancestor: int, time_counter: int):
        used.add(v)
        dp[v] = dp.get(v, node(time_counter, time_counter))
        for to in graph.get(v, []):
            if to == ancestor:
                continue
            if to in used:
                dp[v].up = min(dp[v].up, dp[to].time_in)
            else:
                dfs_bridges(to, v, time_counter + 1)
                dp[v].up = min(dp[v].up, dp[to].up)
                if dp[to].up > dp[v].time_in:
                    search_answer.append((v, to))

    initialization(graph, used, undirected_edges, directed_edges)
    dp, search_answer = dict(), []
    # go through all components
    for i in graph.keys():
        if i not in used:
            dfs_bridges(i, -1, 0)

    return search_answer