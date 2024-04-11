import pandas as pd
import toml

def run(excel_file, row_ranges_file, output_file):
    df = pd.read_excel(excel_file, header=None, names=['BarcodeId', 'ProductName'])

    with open(row_ranges_file, 'r') as f:
        data = toml.load(f)

    for campaign, row_ranges in data.items():
        products = []
        for row_range in row_ranges:
            if ' ' in row_range:  # If row range contains spaces, split and handle as range
                start, end = map(int, row_range.split(' '))
                for row in range(start, end + 1):
                    barcode_id = df.at[row - 1, 'BarcodeId']
                    product_name = df.at[row - 1, 'ProductName']
                    products.append({'barcode_id': str(barcode_id), 'product_name': str(product_name)})
            else:  # If single row
                barcode_id = df.at[int(row_range) - 1, 'BarcodeId']
                product_name = df.at[int(row_range) - 1, 'ProductName']
                products.append({'barcode_id': str(barcode_id), 'product_name': str(product_name)})
        data[campaign] = products

    with open(output_file, 'w') as f:
        toml.dump(data, f)

excel_file = "barkod.xlsx"
row_ranges_file = "products2.toml"
output_file = "campaign_products_map.toml"
run(excel_file, row_ranges_file, output_file)
