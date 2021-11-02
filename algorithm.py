import queue


def isNumber(x: str) -> bool:
    return (x.isdigit() and (x == '0' or x[0] != '0'))


def isNumbers(graph: dict) -> bool:
    return all([isNumber(i) for i in graph.keys()]) and all([isNumber(j) for i in graph.values() for j in i])

# initialize list of edges in graph from user input


def initialization(graph: dict, used: set, undirected_edges: str, directed_edges: str, all_undirected = False):
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
            if all_undirected:
                graph[tline[1]] = graph.get(tline[1], []) + [tline[0]]
        else:
            if len(tline) > 0:
                graph[tline[0]] = graph.get(tline[0], [])

    if isNumbers(graph):
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


def component(graph: dict(), used: set(), undirected_edges: str, directed_edges: str) -> int:
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


def circuit_rank(graph: dict(), used: set(), undirected_edges: str, directed_edges: str) -> int:
    # Circuit rank = (number of edges) - (number of vertices) + (number of components)
    # Add (number of edges)
    answer = len(list(filter(lambda x: (x is not None and len(
        x.split()) >= 2), (undirected_edges + '\n' + directed_edges).split('\n'))))

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
        return dict(sorted(answer.items(), key=lambda i: int(i[0])))
    return dict(sorted(answer.items()))


# initialize and set start vertex for dfs
def depth_first_search(start_vertex: str, undirected_edges: str, directed_edges: str) -> list:
    graph = dict()
    used = set()
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
    # if all vertices is numbers then start vertex must be number
    if len(graph) > 0 and isinstance(list(graph.keys())[0], int):
        start_vertex = int(start_vertex)
        dfs(start_vertex)
        # convert to strings for join output
        search_answer = [str(i) for i in search_answer]
    else:
        dfs(start_vertex)

    return search_answer


def breadth_first_search(start_vertex: str, undirected_edges: str, directed_edges: str) -> list:
    graph = dict()
    used = set()
    initialization(graph, used, undirected_edges, directed_edges)
    # if all vertices is numbers then start vertex must be number
    if len(graph) > 0 and isinstance(list(graph.keys())[0], int):
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

# auxiliary class for bridges and cutpoints search


class node():
    def __init__(self, time_in: int, up: int):
        self.time_in = time_in  # time that reach vertex
        self.up = up  # auxiliary variable on the basics of which we will find the answer


def cutpoints(undirected_edges: str, directed_edges: str) -> list:
    graph = dict()
    used = set()
    initialization(graph, used, undirected_edges, directed_edges, True)

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
                    search_answer.add(v)
                children += 1
        if ancestor == -1 and children > 1:
            search_answer.append(v)

    dp, search_answer = dict(), set()
    # go through all components
    for i in graph.keys():
        if i not in used:
            dfs_cutpoints(i, -1, 0)

    return search_answer


def bridges(undirected_edges: str, directed_edges: str) -> list:
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

    graph = dict()
    used = set()
    initialization(graph, used, undirected_edges, directed_edges, True)
    dp, search_answer = dict(), []
    # go through all components
    for i in graph.keys():
        if i not in used:
            dfs_bridges(i, -1, 0)

    return search_answer
