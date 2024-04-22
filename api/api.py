from flask import Flask, request, jsonify
import pandas as pd
from sqlalchemy import create_engine
from datetime import datetime
import traceback
import pickle
import os

app = Flask(__name__)

# Create a connection to the PostgreSQL database
connection_string = 'postgresql://postgres:password@localhost:5432/lokate_dev'
if os.environ.get('DB_CONNECTION_STRING') != None:
    connection_string = os.environ["DB_CONNECTION_STRING"]  

engine = create_engine(connection_string)

category_keys = [
    'seker_sakiz', 'cikolata_biskuvi', 'cips', 'gevrek', 'bebek',
    'sampuan_dusjeli', 'sabun', 'kisisel_bakim', 'camasir', 'bulasik',
    'ev_temizligi', 'makarna_pirinc_bakliyat', 'hazirgida_baharat',
    'sigara', 'pasta', 'peynir_tereyagi', 'dondurulmus', 'yumurta',
    'salam_sosis_sucuk', 'kahve', 'cay', 'alet', 'sos', 'ekmek',
    'sivi_yag', 'meyve_sebze', 'maden_suyu', 'icecek', 'kolonya',
    'konserve_salca', 'pecete', 'mangal', 'poset', 'recel_bal',
    'porselen', 'dondurma', 'kedi_kopek', 'kuruyemis', 'plastik',
    'su', 'sut', 'ayran_yogurt', 'pil'
]

@app.route('/probabilityToBuy/<user_id>', methods=['GET'])
def get_user_event_today(user_id):
    try:
        # Ensure user ID is provided
        if not user_id:
            return jsonify({'error': 'User ID is required'}), 400

        # Get today's date
        today_date = datetime.now().date()

        # Read and preprocess event data for the specified user
        query = f"SELECT * FROM events WHERE customer_id = '{user_id}'"
        event_df = pd.read_sql(query, engine)
        # display(event_df.head(3))

        event_df['enter_timestamp'] = pd.to_datetime(event_df['enter_timestamp'])
        event_df['possible_exit_timestamp'] = pd.to_datetime(event_df['possible_exit_timestamp'])
        event_df['date'] = event_df['enter_timestamp'].dt.date
        event_df = event_df[event_df['date'] == today_date]

        # Filter events for today's date
        print(f"Event data: {event_df}")

        # Calculate time in seconds
        event_df['time'] = (event_df['possible_exit_timestamp'] - event_df['enter_timestamp']).dt.total_seconds().astype(int)

        # Extract date from possible_exit_timestamp
        event_df['date'] = event_df['possible_exit_timestamp'].dt.date

        # Fetch product groups data
        query_product_groups = "SELECT id as group_id, group_name FROM product_groups"
        product_groups_df = pd.read_sql(query_product_groups, engine)

        # Read product_groups_to_campaigns data
        query_product_groups_to_campaigns = "SELECT * FROM product_groups_to_campaigns"
        product_groups_to_campaigns_df = pd.read_sql(query_product_groups_to_campaigns, engine)

        # Merge product_groups_to_campaigns data into productGroups dataframe based on product_group_id
        product_groups_to_campaigns_df = pd.merge(product_groups_df, product_groups_to_campaigns_df, left_on='group_id', right_on='product_group_id', how='right')

        # Group product_groups_df by campaign_id and join groupNames with a comma
        product_groups_to_campaigns_df['productGroups'] = product_groups_to_campaigns_df.groupby('campaign_id')['group_name'].transform(lambda x: ', '.join(x))

        # Drop duplicates
        product_groups_to_campaigns_df = product_groups_to_campaigns_df.drop_duplicates(subset='campaign_id')
        product_groups_to_campaigns_df = product_groups_to_campaigns_df[['campaign_id', 'productGroups']]

        # Merge productGroups data into events dataframe based on campaign_id
        event_df = pd.merge(event_df, product_groups_to_campaigns_df, left_on='campaign_id', right_on='campaign_id', how='left')

        # Rename columns
        event_df = event_df.rename(columns={'customer_id': 'userId'})

        # Select only required columns
        event_df = event_df[['userId', 'productGroups', 'date', 'time']]
        event_df = event_df.dropna(subset=['productGroups'])




        data = {f'G{i}': [0] for i in range(len(category_keys))}
        sample = pd.DataFrame(data)

        for _, row in event_df.iterrows():
            # Get the product groups for the current row
            product_groups = row['productGroups'].split(', ')
            time = row['time']
            # For each product group, increment its corresponding 'Gi' value in sample
            for product_group in product_groups:
                if product_group in category_keys:
                    sample[f'G{category_keys.index(product_group)}'] += time

        # Load the model from disk
        with open('model.pkl', 'rb') as file:
            loaded_model = pickle.load(file)

        # Use the loaded model to make predictions
        probabilities = loaded_model.predict_proba(sample)

        probabilities_list = []
        for i, prob in enumerate(probabilities):
            print(f"Probability for class {i}: {prob[0][1]}")
            # Append the probability to the list
            probabilities_list.append(prob[0][1])

        # Convert the list to a JSON array and return
        return jsonify(probabilities_list)

    except Exception as e:
        tb = traceback.format_exc()
        print(tb)
        return jsonify({'error': str(e)}), 500
if __name__ == '__main__':
    app.run(debug=True)
