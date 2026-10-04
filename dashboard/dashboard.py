import time
import json
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

            # === Artifact 14: User Input Layer ===
            user = input("Enter operator JSON (or press Enter to skip): ").strip()

            if user:
                try:
                    op = json.loads(user)
                    controller.inject_operator(op)
                    print("Operator injected.")
                except Exception as e:
                    print("Invalid operator JSON:", e)

            time.sleep(0.1)

    except KeyboardInterrupt:
        print("Shutting down Vectorium...")
        controller.shutdown()
