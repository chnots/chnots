import psycopg2
from psycopg2 import sql
from datetime import datetime


def copy_public_to_new_schema(db_params, new_schema_name):
    """
    Creates a new schema and copies all tables from public schema to it.

    Args:
        db_params: Dictionary containing database connection parameters
        new_schema_name: Name of the new schema to create
    """
    try:
        # Connect to the database
        conn = psycopg2.connect(**db_params)
        cursor = conn.cursor()

        # Create the new schema
        cursor.execute(
            sql.SQL("CREATE SCHEMA IF NOT EXISTS {}").format(
                sql.Identifier(new_schema_name)
            )
        )

        # Get all tables from public schema
        cursor.execute("""
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_type = 'BASE TABLE'
        """)
        tables = cursor.fetchall()

        # Copy each table to the new schema
        for table in tables:
            table_name = table[0]

            # Create the table in new schema with same structure
            create = sql.SQL("""
                CREATE TABLE {}.{} (LIKE public.{} INCLUDING ALL)
            """).format(
                sql.Identifier(new_schema_name),
                sql.Identifier(table_name),
                sql.Identifier(table_name),
            )
            print(create)
            cursor.execute(create)

            # Copy data from public to new schema
            cursor.execute(
                sql.SQL("""
                INSERT INTO {}.{} 
                SELECT * FROM public.{}
            """).format(
                    sql.Identifier(new_schema_name),
                    sql.Identifier(table_name),
                    sql.Identifier(table_name),
                )
            )

        conn.commit()
        print(
            f"Successfully created schema '{new_schema_name}' and copied all tables from public schema."
        )

    except Exception as e:
        conn.rollback()
        print(f"Error: {e}")
    finally:
        if conn:
            cursor.close()
            conn.close()


# Example usage
db_params = {
    "host": "localhost",
    "database": "chnotsprod",
    "user": "chnots",
    "password": "chnots",
    "port": "5432",
}

copy_public_to_new_schema(
    db_params, f"chnotsprod_{datetime.now().strftime('%Y%m%d%H%M%S')}"
)
