import copy

G, used, SearchAnswer = dict(), set(), set()
isNumbers = False
GTemp = dict()

def isNumber(x: str) -> bool:
	return (x.isdigit() and (x == '0' or x[0] != '0'))

def initialization(undirected_edges, directed_edges):
    global G, used, isNumbers
    G, used = dict(), set()
    all_edges = str(undirected_edges) + '\n' + str(directed_edges)

    if len(all_edges) > 0:
        for line in all_edges.split('\n'):
            tline = line.split()
            if len(tline) > 1:
                G[tline[0]] = G.get(tline[0], []) + [tline[1]]
                G[tline[1]] = G.get(tline[1], []) + [tline[0]]
            else:
                if len(tline) > 0:
                    G[tline[0]] = G.get(tline[0], [])

    isNumbers = all([isNumber(i) for i in G.keys()]) and all([isNumber(j) for i in G.values() for j in i])
    if isNumbers:
        GTemp1 = dict()
        for k, v in G.items():
            GTemp1[int(k)] = list(map(int, v))
        G = GTemp1
    
    G = dict(sorted(G.items()))
    for i in G.keys():
        G[i] = sorted(G.get(i, []))


def dfs_Component(v):
    global G, used

    used.add(v)
    for i in G[v]:
        if i not in used:
            dfs_Component(i)


def Component(undirected_edges, directed_edges):
    answer = 0

    initialization(undirected_edges, directed_edges)

    global G, used

    for i in G.keys():
        if i not in used:
            answer += 1
            dfs_Component(i)

    return answer


def CircuitRank(undirected_edges, directed_edges):
    global G, used
    answer = len(list(filter(lambda x: (x is not None and len(x.split()) >= 2), (undirected_edges + '\n' + directed_edges).split('\n'))))
    answer += Component(undirected_edges, directed_edges) - len(G.keys())

    return answer


def Degree(undirected_edges, directed_edges):
    all_edges = str(undirected_edges) + '\n' + str(directed_edges)
    answer = dict()

    if len(all_edges) > 0:
        for line in all_edges.split('\n'):
            tline = line.split()
            if len(tline) > 1:
                answer[tline[0]] = answer.get(tline[0], 0) + 1
                answer[tline[1]] = answer.get(tline[1], 0) + 1
            else:
                if len(tline) > 0:
                    answer[tline[0]] = answer.get(tline[0], 0)

    if all([isNumber(i) for i in answer.keys()]):
        return dict(sorted(answer.items(), key = lambda i: int(i[0])))
    return dict(sorted(answer.items()))


def dfs(v):
    global G, used, SearchAnswer
    SearchAnswer.append(str(v))
    used.add(v)
    for i in G[v]:
        if i not in used:
            dfs(i)


def DFS(start_vertex, undirected_edges, directed_edges):
    global G, used, SearchAnswer
    initialization(undirected_edges, directed_edges)
    SearchAnswer = []

    global isNumbers
    if isNumbers == True:
        start_vertex = int(start_vertex)
    
    dfs(start_vertex)

    for i in G.keys():
        if i not in used:
            dfs(i)

    return SearchAnswer


def BFS(start_vertex, undirected_edges, directed_edges):
    global G, used, SearchAnswer
    initialization(undirected_edges, directed_edges)

    global isNumbers
    if isNumbers == True:
        start_vertex = int(start_vertex)

    SearchAnswer, q = [], [start_vertex]
    used.add(start_vertex)

    while len(q) > 0:
        v = q[0]
        SearchAnswer.append(str(v))
        q.pop(0)
        for i in G[v]:
            if i not in used:
                q.append(i)
                used.add(i)

    return SearchAnswer


def dfs_Point_Bridge(v):
    global used, GTemp
    used.add(v)
    for j in GTemp[v]:
        if j not in used:
            dfs_Point_Bridge(j)

def ArticulationPoint(undirected_edges, directed_edges):
    global G, used, GTemp
    CNT_Components = Component(undirected_edges, directed_edges)
    
    answer = set()
    for i in G.keys():
        GTemp = copy.deepcopy(G)
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
    global G, used, GTemp, isNumbers
    CNT_Components = Component(undirected_edges, directed_edges)

    all_edges = str(undirected_edges) + '\n' + str(directed_edges)
    answer = set()

    if len(all_edges) > 0:
        for line in all_edges.split('\n'):
            tline = line.split()
            if isNumbers:
                tline = list(map(int, tline))
            
            GTemp = copy.deepcopy(G)
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