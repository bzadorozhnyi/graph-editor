import algorithm
import dash
from dash.dependencies import Input, Output, State
import dash_cytoscape as cyto
import dash_html_components as html
import dash_core_components as dcc

app = dash.Dash(__name__)
server = app.server

# ----------------- App layout -----------------

app.layout = html.Div([
    html.Div(className='graph_and_command', children=[
        html.Div(className='part_network', children=[
            cyto.Cytoscape(
                id='cytoscape-elements-callbacks',
                elements=[],
                autoRefreshLayout=False,
                stylesheet=[
                    {
                        'selector': 'node',
                        'style': {
                            'background-color': 'white',
                            'border-color': 'black',
                            'border-width': '2px',
                            'color': 'black',
                            'font-family': ['sans-serif'],
                            'font-size': 16,
                            'font-weight': 700,
                            'label': 'data(label)',
                            'shape': 'circle',
                            'text-halign':'center',
                            'text-valign':'center'
                        }
                    },
                    {
                        'selector': ':selected',
                        'style': {
                            'background-color': '#80bfff',
                            'color': 'black',
                            'font-family': ['sans-serif'],
                            'font-size': 16,
                            'font-weight': 700,
                            'label': 'data(label)',
                            'shape': 'circle',
                            'text-halign': 'center',
                            'text-valign': 'center'
                        }
                    },
                    {
                        'selector': 'edge',
                        'style': {
                            'curve-style': 'bezier',
                            'font-family': ['sans-serif'],
                            'font-size': 16,
                            'font-weight': 700,
                            'label': 'data(weight)',
                            'line-color': 'black',
                            'text-rotation': 'autorotate',
                            'text-margin-y': '20px'
                        }
                    },
                    {
                        'selector': '.directed-edges',
                        'style': {
                            'mid-target-arrow-color': 'black',
                            'mid-target-arrow-shape': 'triangle',
                            'line-color': 'black'
                        }
                    }
                ],
                layout={
                    'name': 'grid'
                }
            )
        ]),
        html.Div(className='Command_Info', children=[
            html.Div(className='CommandSelector', children=[
                dcc.Dropdown(
                    id='command_dropdown',
                    options=[
                        {'label': 'Number of components in graph',
                            'value': 'COMPONENT'},
                        {'label': 'Circuit rank',
                            'value': 'CIRCUIT RANK'},
                        {'label': 'Degree of vertices',
                            'value': 'Degree of vertices'},
                        {'label': 'Depth-first search', 'value': 'DFS'},
                        {'label': 'Обхід в ширину', 'value': 'BFS'},
                        {'label': 'Точки зчеплення', 'value': 'Articulation Point'},
                        {'label': 'Мости', 'value': 'Bridge'}
                    ],
                    value=''
                ),
                html.Button('Submit', className='Submit_button',
                            id='submit_button', n_clicks=0),
            ]),
            html.Div(className='InfoOutput', id='dd-output-container', children=[
                html.Pre(id='output-container')
            ])
        ]),
        html.Div(className='two-columns', children=[
            dcc.Tabs(id='tabs', children=[
                dcc.Tab(label='Undirecred edges', className='custom-tab', selected_className='custom-tab--selected', children=[
                    html.Div(className='tab', children=[
                        dcc.Textarea(
                            id='textarea_undirected_edges',
                            value='',
                            className='textarea'
                        )
                    ])
                ]),
                dcc.Tab(label='Direcred edges', className='custom-tab', selected_className='custom-tab--selected', children=[
                    html.Div(className='tab', children=[
                        dcc.Textarea(
                            id='textarea_directed_edges',
                            value='',
                            className='textarea'
                        )
                    ])
                ])
            ])
        ])
    ]),
    html.Div(id='placeholder')
])

# ----------------------------------------------

# ----------------- Callback -----------------


@app.callback(
    Output('cytoscape-elements-callbacks', 'elements'),
    Input('textarea_undirected_edges', 'value'),
    Input('textarea_directed_edges', 'value')
)
def update_graph(textarea_undirected_edges, textarea_directed_edges):
    nodes, edges = [], []
    all_nodes = set()

    all_value = str(textarea_undirected_edges) + \
        '\n' + str(textarea_directed_edges)
    for line in all_value.split('\n'):
        tline = line.split()
        for j in range(min(len(tline), 2)):
            if tline[j] not in all_nodes:
                x = abs(hash(tline[j])) % 300
                y = abs(hash(tline[j]) + hash(tline[j][0])) % 300
                nodes.append(
                    {
                        'data': {'id': 'n' + tline[j], 'label': tline[j]},
                        'position': {'x': x, 'y': y}
                    }
                )
                all_nodes.add(tline[j])

    for line in textarea_undirected_edges.split('\n'):
        tline = line.split()
        if len(tline) <= 1:
            continue
        else:
            if len(tline) == 2:
                edges.append(
                    {'data': {'source': 'n' +
                              tline[0], 'target': 'n' + tline[1]}}
                )
            else:
                edges.append(
                    {'data': {'source': 'n' +
                              tline[0], 'target': 'n' + tline[1], 'weight': tline[2]}}
                )

    for line in textarea_directed_edges.split('\n'):
        tline = line.split()
        if len(tline) <= 1:
            continue
        else:
            if len(tline) == 2:
                edges.append(
                    {'data': {
                        'source': 'n' + tline[0], 'target': 'n' + tline[1]}, 'classes': 'directed-edges'}
                )
            else:
                edges.append(
                    {'data': {'source': 'n' + tline[0], 'target': 'n' +
                              tline[1], 'weight': tline[2]}, 'classes': 'directed-edges'}
                )

    return nodes + edges


@app.callback(
    Output('output-container', 'children'),
    Input('submit_button', 'n_clicks'),
    Input('command_dropdown', 'value'),
    Input('cytoscape-elements-callbacks', 'tapNodeData'),
    State('textarea_undirected_edges', 'value'),
    State('textarea_directed_edges', 'value')
)
def update_message(n_clicks, value, data, undirected_edges, directed_edges):
    if n_clicks > 0:
        n_clicks = 0
        try:
            if value == 'COMPONENT':
                return 'Number of components in graph = {}'.format(algorithm.component(dict(), set(), undirected_edges, directed_edges))
            elif value == 'CIRCUIT RANK':
                return 'Circuit rank = {}'.format(algorithm.circuit_rank(dict(), set(), undirected_edges, directed_edges))
            elif value == 'Degree of vertices':
                return 'Degree of vertices :\n' + '\n'.join('{} : {}'.format(k, str(v)) for k, v in algorithm.degree(undirected_edges, directed_edges).items())
            elif value == 'DFS':
                if data is None:
                    return 'Choose start vertex'
                return 'Depth-first search : \n' + '\n'.join(algorithm.depth_first_search(dict(), set(), data['label'], undirected_edges, directed_edges))
            elif value == 'BFS':
                if data is None:
                    return 'Оберіть вершину для початку обходу'
                return 'Обхід в ширину : \n' + '\n'.join(algorithm.BFS(data['label'], undirected_edges, directed_edges))
            elif value == 'Articulation Point':
                return 'Точки зчеплення :\n' + '\n'.join('{}'.format(i) for i in algorithm.ArticulationPoint(undirected_edges, directed_edges))
            elif value == 'Bridge':
                return 'Мости :\n' + '\n'.join('{}'.format(i) for i in algorithm.Bridges(undirected_edges, directed_edges))
            else:
                return 'Команда не була обрана'
        except BaseException:
            return 'Виникла помилка, радимо обробити запит вручну'

# --------------------------------------------


if __name__ == '__main__':
    app.run_server(debug=True)
