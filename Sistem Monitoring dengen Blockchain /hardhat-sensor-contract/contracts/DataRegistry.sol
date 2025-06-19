// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract DataRegistry {
    // Struct untuk menyimpan satu data pembacaan sensor
    struct SensorReading {
        int64 temperature; // Suhu dikali 100 (misal: 25.50°C disimpan sbg 2550)
        int64 humidity;    // Kelembapan dikali 100 (misal: 60.75% disimpan sbg 6075)
        uint256 timestamp; // Waktu data dicatat
    }

    // Mapping untuk menyimpan riwayat data berdasarkan ID perangkat
    mapping(string => SensorReading[]) public readingsByDevice;

    // Event yang akan dipancarkan setiap kali data baru dicatat
    event DataRecorded(string deviceId, int64 temperature, int64 humidity);

    /**
     * @dev Mencatat data sensor baru ke blockchain.
     * @param deviceId ID unik dari perangkat/sensor.
     * @param _temperature Suhu yang akan dicatat (sudah dikali 100).
     * @param _humidity Kelembapan yang akan dicatat (sudah dikali 100).
     */
    function recordData(string memory deviceId, int64 _temperature, int64 _humidity) public {
        readingsByDevice[deviceId].push(
            SensorReading({
                temperature: _temperature,
                humidity: _humidity,
                timestamp: block.timestamp
            })
        );
        emit DataRecorded(deviceId, _temperature, _humidity);
    }
}