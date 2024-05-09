import json
import numpy as np
from tslearn.clustering import TimeSeriesKMeans
from tslearn.metrics import dtw, dtw_path
from tslearn.utils import to_time_series_dataset

def encode_route(route, encoding):
    encoded_route = []
    for stop in route:
        encoded_stop = encoding[stop]
        encoded_route.append(encoded_stop)
    return encoded_route

# next stop is in the form of beacon name
def get_next_stop(current_route):
    print('Current route:', current_route)

    if not current_route:
        return ""

    with open('models/visit-analytics/encoded_names.json', 'r') as f:
        encoded_names = json.load(f)
    with open('models/visit-analytics/cluster_routes.json', 'r') as f:
        cluster_routes = json.load(f)

    encoded_route = encode_route(current_route, encoded_names)

    model = TimeSeriesKMeans.from_pickle('models/visit-analytics/model.pkl')
    cluster_no = str(model.predict(to_time_series_dataset([encoded_route]))[0])
    print('Cluster no:', cluster_no)

    similar_routes = cluster_routes[cluster_no]
    last_stop = current_route[-1]

    next_stop_counts = {}
    for similar_route in similar_routes:
        if last_stop in similar_route:
            next_stop_idx = similar_route.index(last_stop) + 1
            if next_stop_idx < len(similar_route):
                next_stop = similar_route[next_stop_idx]
                if next_stop not in next_stop_counts:
                    next_stop_counts[next_stop] = 0
                next_stop_counts[next_stop] += 1

    print('Next stop counts: ', next_stop_counts)
    if next_stop_counts:
        return max(next_stop_counts, key=next_stop_counts.get)
    else:
        return ""

# current_route = [
#       "Yellow",
#       "Pseudo20",
#       "Pseudo14",
#       "Pseudo13",
#       "Pseudo17",
#       "Pseudo5",
#       "Pseudo3",
# ]
# print(get_next_route(current_route))