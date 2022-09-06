def parse_field(row, field):
    try:
        return row[field]
    except:
        return None

def row_to_user(row):
    obj = {
        "uuid": parse_field(row, "uuid"),
        "name": parse_field(row, "name"),
        "description": parse_field(row, "description"),
        "password": parse_field(row, "password"),
        "email": parse_field(row, "email"),
        "archived": parse_field(row, "archived"),
        "creation_date": parse_field(row, "creation_date"),
        "verified": parse_field(row, "verified"),
        "role": parse_field(row, "role"),
    }
    if obj["role"] == "":
        obj["role"] = "USER"
    obj = {k: v for k, v in obj.items() if v is not None and v != ""}
    return obj
