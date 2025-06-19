import sqlite3
import os

# Nama file database
DB_FILE = "sensor_data.db"

def initialize_database():
    # Hapus file lama jika ingin mulai dari awal (opsional)
    if not os.path.exists(DB_FILE):
        print(f"Membuat database baru: {DB_FILE}")
    else:
        print(f"Database sudah ada: {DB_FILE}")

    # Koneksi ke SQLite
    conn = sqlite3.connect(DB_FILE)
    cursor = conn.cursor()

    # Buat tabel jika belum ada
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS sensor_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            temperature REAL NOT NULL,
            humidity REAL NOT NULL,
            tx_hash TEXT
        )
    ''')

    # Simpan dan tutup koneksi
    conn.commit()
    conn.close()
    print("Tabel 'sensor_log' siap digunakan.")

if __name__ == "__main__":
    initialize_database()
