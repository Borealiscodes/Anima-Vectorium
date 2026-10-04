use crate::haptics::envelope::DeviceClass;
use crate::haptics::device_class::HapticEnvelope;

pub fn enforce_ndh_safety(envelope: &mut HapticEnvelope, class: DeviceClass) {
    let max_amp = class.max_amplitude();
    let max_freq = class.max_frequency();

    if envelope.amplitude > max_amp {
        envelope.amplitude = max_amp;
    }

    if envelope.frequency > max_freq {
        envelope.frequency = max_freq;
    }

    // Temporal curve normalization
    let max_curve = envelope.temporal_curve.iter().cloned().fold(0.0, f32::max);
    if max_curve > 1.0 {
        for i in 0..4 {
            envelope.temporal_curve[i] /= max_curve;
        }
    }
}
