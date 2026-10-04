import ctypes
import json
import time
import os

# Load the shared library
lib_path = os.path.abspath("./target/debug/libvectorium.so")
vectorium = ctypes.CDLL(lib_path)

# FFI signatures
vectorium.initialize_vectorium.restype = None
vectorium.tick_vectorium.restype = None

vectorium.get_expressive_state_json.restype = ctypes.c_char_p
vectorium.get_telemetry_json.restype = ctypes.c_char_p
vectorium.get_haptic_envelope_json.restype = ctypes.c_char_p

# Initialize runtime
vectorium.initialize_vectorium()

print("Vectorium runtime initialized.\n")

# Main polling loop
while True:
    vectorium.tick_vectorium()

    state_json = vectorium.get_expressive_state_json().decode("utf-8")
    telemetry_json = vectorium.get_telemetry_json().decode("utf-8")
    haptics_json = vectorium.get_haptic_envelope_json().decode("utf-8")

    print("Expressive State:", state_json)
    print("Telemetry:", telemetry_json)
    print("Haptics:", haptics_json)
    print("-" * 40)

    time.sleep(0.1)
