import ctypes
import json
import time
import os

class VectoriumController:
    def __init__(self):
        lib_path = os.path.abspath("./target/debug/libvectorium.so")
        self.vectorium = ctypes.CDLL(lib_path)

        # FFI signatures
        self.vectorium.initialize_vectorium.restype = None
        self.vectorium.tick_vectorium.restype = None

        self.vectorium.get_expressive_state_json.restype = ctypes.c_char_p
        self.vectorium.get_telemetry_json.restype = ctypes.c_char_p
        self.vectorium.get_haptic_envelope_json.restype = ctypes.c_char_p

        self.vectorium.inject_operator_json.argtypes = [ctypes.c_char_p]
        self.vectorium.inject_operator_json.restype = None

        # Initialize runtime
        self.vectorium.initialize_vectorium()

        # Cached state
        self.state = {}
        self.telemetry = {}
        self.haptics = {}

    def tick(self):
        self.vectorium.tick_vectorium()

    def poll(self):
        self.state = json.loads(
            self.vectorium.get_expressive_state_json().decode("utf-8")
        )
        self.telemetry = json.loads(
            self.vectorium.get_telemetry_json().decode("utf-8")
        )
        self.haptics = json.loads(
            self.vectorium.get_haptic_envelope_json().decode("utf-8")
        )

    def inject_operator(self, operator_dict):
        json_str = json.dumps(operator_dict)
        self.vectorium.inject_operator_json(json_str.encode("utf-8"))

    def loop(self, dt=0.1):
        while True:
            self.tick()
            self.poll()

            print("State:", self.state)
            print("Telemetry:", self.telemetry)
            print("Haptics:", self.haptics)
            print("-" * 40)

            time.sleep(dt)
