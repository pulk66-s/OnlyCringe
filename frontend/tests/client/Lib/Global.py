def parse_field(row, field):
    try:
        return row[field]
    except:
        return None
