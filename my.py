import algorithm
import dash
from dash.dependencies import Input, Output, State
import dash_cytoscape as cyto
from dash import html
from dash import dcc

app = dash.Dash(__name__)
server = app.server

# ----------------- App layout -----------------

app.layout = html.Div([
    html.Div(children=[
        dcc.Dropdown(
            clearable=False,
            id='styling-nodes',
            options=[
                {'label': 'Circle', 'value': 'circle'},
                {'label': 'Rectangle', 'value': 'rectangle'},
                {'label': 'Pentagon', 'value': 'pentagon'},
                {'label': 'Hexagon', 'value': 'hexagon'}
            ],
            value='circle'
        ),
        dcc.Slider(
            id='node-size',
            min=50,
            max=100,
            step=None,
            marks={
                50: '50',
                60: '60',
                70: '70',
                80: '80',
                90: '90',
                100: '100'
            },
            value=50
        )
    ]),
    html.Div(className='graph-command', children=[
        html.Div(className='part-network', children=[
            cyto.Cytoscape(
                id='cytoscape-elements-callbacks',
                elements=[],
                autoRefreshLayout=False,
                stylesheet=[
                    {
                        'selector': 'node',
                        'style': {
                            'background-color': 'grey',
                            'color': 'black',
                            'font-family': ['sans-serif'],
                            'font-size': 16,
                            'font-weight': 700,
                            'label': 'data(label)',
                            'text-halign': 'center',
                        }
                    },
                    {
                        'selector': '.size_50',
                        'style': {
                            'width': '50px',
                            'height': '50px',
                            'text-margin-y': '35px'
                        }
                    },
                    {
                        'selector': '.size_60',
                        'style': {
                            'width': '60px',
                            'height': '60px',
                            'text-margin-y': '40px'
                        }
                    },
                    {
                        'selector': '.size_70',
                        'style': {
                            'width': '70px',
                            'height': '70px',
                            'text-margin-y': '45px'
                        }
                    },
                    {
                        'selector': '.size_80',
                        'style': {
                            'width': '80px',
                            'height': '80px',
                            'text-margin-y': '50px'
                        }
                    },
                    {
                        'selector': '.size_90',
                        'style': {
                            'width': '90px',
                            'height': '90px',
                            'text-margin-y': '55px'
                        }
                    },
                    {
                        'selector': '.size_100',
                        'style': {
                            'width': '100px',
                            'height': '100px',
                            'text-margin-y': '60px'
                        }
                    },
                    {
                        'selector': '.circle',
                        'style': {
                            'shape': 'circle',
                        }
                    },
                    {
                        'selector': '.rectangle',
                        'style': {
                            'shape': 'rectangle',
                        }
                    },
                    {
                        'selector': '.pentagon',
                        'style': {
                            'shape': 'pentagon',
                        }
                    },
                    {
                        'selector': '.hexagon',
                        'style': {
                            'shape': 'hexagon',
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
                            'line-color': 'grey',
                            'text-rotation': 'autorotate',
                            'text-margin-y': '20px'
                        }
                    },
                    {
                        'selector': '.directed-edges',
                        'style': {
                            'mid-target-arrow-color': 'grey',
                            'mid-target-arrow-shape': 'triangle',
                            'line-color': 'grey'
                        }
                    }
                ],
                layout={
                    'name': 'grid'
                }
            )
        ]),
        html.Div(className='command-info', children=[   
            html.Div(className='command-selector', children=[
                dcc.Dropdown(
                    clearable=False,
                    id='command-dropdown',
                    options=[
                        {'label': 'Number of components in graph',
                            'value': 'components'},
                        {'label': 'Circuit rank',
                            'value': 'circuit rank'},
                        {'label': 'Degree of vertices',
                            'value': 'degree of vertices'},
                        {'label': 'Depth-first search', 'value': 'dfs'},
                        {'label': 'Breadth-first search', 'value': 'bfs'},
                        {'label': 'Cupoints', 'value': 'cupoints'},
                        {'label': 'Bridges', 'value': 'bridges'}
                    ],
                    value=''
                ),
                html.Button('Submit', className='submit-button',
                            id='submit-button', n_clicks=0),
            ]),
            html.Div(className='info-output', id='dd-output-container', children=[
                html.Pre(id='output-container')
            ])
        ]),
        html.Div(className='user-input', children=[
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
    Input('textarea_directed_edges', 'value'),
    Input('styling-nodes', 'value'),
    Input('node-size', 'value')
)
def update_graph(textarea_undirected_edges, textarea_directed_edges, shape, node_size):
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
                        'position': {'x': x, 'y': y},
                        'classes': shape + ' size_' + str(node_size)
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
    Input('submit-button', 'n_clicks'),
    Input('command-dropdown', 'value'),
    Input('cytoscape-elements-callbacks', 'tapNodeData'),
    State('textarea_undirected_edges', 'value'),
    State('textarea_directed_edges', 'value')
)
def update_message(n_clicks, value, data, undirected_edges, directed_edges):
    if n_clicks > 0:
        n_clicks = 0
        try:
            if value == 'components':
                return 'Number of components in graph = {}'.format(algorithm.component(dict(), set(), undirected_edges, directed_edges))
            elif value == 'circuit rank':
                return 'Circuit rank = {}'.format(algorithm.circuit_rank(dict(), set(), undirected_edges, directed_edges))
            elif value == 'degree of vertices':
                return 'Degree of vertices :\n' + '\n'.join('{} : {}'.format(k, str(v)) for k, v in algorithm.degree(undirected_edges, directed_edges).items())
            elif value == 'dfs':
                if data is None:
                    return 'Choose start vertex'
                return 'Depth-first search : \n' + '\n'.join(algorithm.depth_first_search(data['label'], undirected_edges, directed_edges))
            elif value == 'bfs':
                if data is None:
                    return 'Choose start vertex'
                return 'Breadth-first search : \n' + '\n'.join(algorithm.breadth_first_search(data['label'], undirected_edges, directed_edges))
            elif value == 'cupoints':
                return 'Cupoints :\n' + '\n'.join('{}'.format(i) for i in algorithm.cutpoints(undirected_edges, directed_edges))
            elif value == 'bridges':
                return 'Bridges :\n' + '\n'.join('{}'.format(i) for i in algorithm.bridges(undirected_edges, directed_edges))
            else:
                return 'Command hasn\'t chosen'
        except BaseException:
            return 'Oops, something went wrong.'

# --------------------------------------------


if __name__ == '__main__':
    app.run_server(debug=True)
