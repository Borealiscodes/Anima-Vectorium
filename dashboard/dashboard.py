import time
from dashboard_controller import VectoriumController

def run_dashboard(lib_path: str):
    controller = VectoriumController(lib_path)

    try:
        while True:
            controller.tick()

            expressive = controller.get_expressive_packet()
            telemetry = controller.get_telemetry_packet()
            haptic = controller.get_haptic_packet()

            print("\n=== Vectorium Dashboard ===")
            print("Timestamp:", expressive["timestamp_ms"])
            print("Stability:", expressive["stability_score"])
            print("Fog Density:", expressive["fog_density"])
            print("Glyph Hint:", expressive["glyph_hint"])

            print("Power (mW):", telemetry["power_mw"])
            print("Spectral Variance:", telemetry["spectral_variance"])

            print("Haptic Amp:", haptic["amplitude"])
            print("Haptic Freq:", haptic["frequency"])

            time.sleep(0.1)

    except KeyboardInterrupt:
        print("Shutting down Vectorium...")
        controller.shutdown()
