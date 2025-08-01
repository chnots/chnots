import psycopg2

# Example usage
db_params = {
    "host": "localhost",
    "database": "chnotsprod",
    "user": "chnots",
    "password": "chnotsprod",
    "port": "5432",
}

def for_all_tables(fn):
    """
    Creates a new schema and copies all tables from public schema to it.

    Args:
        db_params: Dictionary containing database connection parameters
        new_schema_name: Name of the new schema to create
    """
    try:
        # Connect to the database
        print(db_params)
        conn = psycopg2.connect(**db_params)
        cursor = conn.cursor()

        # Get all tables from public schema
        cursor.execute("""
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
        """)
        tables = cursor.fetchall()
        print(tables)

        # Copy each table to the new schema
        for table in tables:
            table_name = table[0]
            fn(cursor, table_name)

        conn.commit()

    except Exception as e:
        conn.rollback()
        print(f"Error: {e}")
    finally:
        if conn:
            cursor.close()
            conn.close()








