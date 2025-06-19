import sys
import socket
import json
import pandas as pd
from PyQt5.QtCore import Qt, QTimer, QPointF
from PyQt5.QtWidgets import (
    QApplication, QMainWindow, QVBoxLayout, QWidget, QTableWidget,
    QTableWidgetItem, QHeaderView, QPushButton, QHBoxLayout
)
from PyQt5.QtChart import QChart, QChartView, QLineSeries, QValueAxis
from PyQt5.QtGui import QPainter

TCP_IP = '127.0.0.1'
TCP_PORT = 8080

class SensorChart(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Sensor Chart + Transaction Viewer")
        self.resize(1000, 700)

        self.counter = 0
        self.history = []

        self.init_ui()
        self.start_timer()

    def init_ui(self):
        self.main_widget = QWidget()
        self.setCentralWidget(self.main_widget)
        main_layout = QVBoxLayout(self.main_widget)

        # Chart
        self.series_temp = QLineSeries(name="Temperature (°C)")
        self.series_hum = QLineSeries(name="Humidity (%)")

        self.chart = QChart()
        self.chart.addSeries(self.series_temp)
        self.chart.addSeries(self.series_hum)
        self.chart.setTitle("Sensor Data Chart")

        self.axis_x = QValueAxis()
        self.axis_x.setLabelFormat("%d")
        self.axis_x.setTitleText("Sample")
        self.axis_x.setRange(0, 10)
        self.chart.addAxis(self.axis_x, Qt.AlignBottom)
        self.series_temp.attachAxis(self.axis_x)
        self.series_hum.attachAxis(self.axis_x)

        self.axis_y = QValueAxis()
        self.axis_y.setRange(0, 100)
        self.axis_y.setTitleText("Value")
        self.chart.addAxis(self.axis_y, Qt.AlignLeft)
        self.series_temp.attachAxis(self.axis_y)
        self.series_hum.attachAxis(self.axis_y)

        chart_view = QChartView(self.chart)
        chart_view.setRenderHint(QPainter.Antialiasing)
        main_layout.addWidget(chart_view)

        # Table
        self.table = QTableWidget(0, 3)
        self.table.setHorizontalHeaderLabels(["Temperature", "Humidity", "Tx Hash"])
        self.table.horizontalHeader().setSectionResizeMode(QHeaderView.Stretch)
        main_layout.addWidget(self.table)

        # Export Button
        button_layout = QHBoxLayout()
        self.export_btn = QPushButton("Export to Excel")
        self.export_btn.clicked.connect(self.export_excel)
        button_layout.addWidget(self.export_btn)
        main_layout.addLayout(button_layout)

    def start_timer(self):
        self.timer = QTimer()
        self.timer.timeout.connect(self.read_socket)
        self.timer.start(2000)  # Baca tiap 2 detik

    def read_socket(self):
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.connect((TCP_IP, TCP_PORT))
                data = s.recv(1024)
                if data:
                    obj = json.loads(data.decode())
                    temp = float(obj.get("temperature", 0))
                    hum = float(obj.get("humidity", 0))
                    tx_hash = obj.get("tx_hash", "N/A")
                    self.update_data(temp, hum, tx_hash)
        except Exception as e:
            print("Socket error:", e)

    def update_data(self, temp, hum, tx_hash):
        self.counter += 1

        self.series_temp.append(QPointF(self.counter, temp))
        self.series_hum.append(QPointF(self.counter, hum))

        # Grafik menumpuk dari kiri, semua data terlihat
        self.axis_x.setRange(0, self.counter)

        row = self.table.rowCount()
        self.table.insertRow(row)
        self.table.setItem(row, 0, QTableWidgetItem(str(temp)))
        self.table.setItem(row, 1, QTableWidgetItem(str(hum)))
        self.table.setItem(row, 2, QTableWidgetItem(tx_hash))

        self.history.append({
            "Temperature": temp,
            "Humidity": hum,
            "Tx Hash": tx_hash
        })

    def export_excel(self):
        df = pd.DataFrame(self.history)
        df.to_excel("sensor_data.xlsx", index=False)
        print("Data exported to sensor_data.xlsx")

if __name__ == "__main__":
    app = QApplication(sys.argv)
    win = SensorChart()
    win.show()
    sys.exit(app.exec_())
