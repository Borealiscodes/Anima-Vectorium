import ctypes
import json
import os

class VectoriumController:
    def __init__(self, lib_path: str):
        self.lib = ctypes.CDLL(lib_path)

        # FFI signatures
        self.lib.initialize_vectorium.restype = ctypes.c_void_p
        self.lib.tick_vectorium.argtypes = [ctypes.c_void_p]
        self.lib.free_vectorium.argtypes = [ctypes.c_void_p]

        self.lib.get_expressive_state_json.argtypes = [ctypes.c_void_p]
        self.lib.get_expressive_state_json.restype = ctypes.c_char_p

        self.lib.get_telemetry_json.argtypes = [ctypes.c_void_p]
        self.lib.get_telemetry_json.restype = ctypes.c_char_p

        self.lib.get_haptic_envelope_json.argtypes = [ctypes.c_void_p]
        self.lib.get_haptic_envelope_json.restype = ctypes.c_char_p

        # Initialize runtime
        self.state_ptr = self.lib.initialize_vectorium()

    def tick(self):
        self.lib.tick_vectorium(self.state_ptr)

    def get_expressive_packet(self):
        raw = self.lib.get_expressive_state_json(self.state_ptr)
        return json.loads(ctypes.string_at(raw).decode("utf-8"))

    def get_telemetry_packet(self):
        raw = self.lib.get_telemetry_json(self.state_ptr)
        return json.loads(ctypes.string_at(raw).decode("utf-8"))

    def get_haptic_packet(self):
        raw = self.lib.get_haptic_envelope_json(self.state_ptr)
        return json.loads(ctypes.string_at(raw).decode("utf-8"))

    def shutdown(self):
        if self.state_ptr:
            self.lib.free_vectorium(self.state_ptr)
            self.state_ptr = None
