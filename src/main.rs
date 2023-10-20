use rodio::{OutputStream, Source};
use std::time::Duration;

// Variables de un wavetable
struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    // Constructor que inicia el oscilador
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOscillator {
        return WaveTableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        };
    }

    // Frequencia del oscilador.
    // Calculando el incremento del índice en función de la frecuencia
    // y el tamaño de la wavetable
    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    // Obtiene un sample del oscilador, realizando interpolación lineal
    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    // Realiza la interpolación lineal entre dos valores en la tabla de ondas
    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

// Permite tratar el oscilador como un iterador que produce muestras de audio
impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

// Define métodos requeridos para que WaveTableOscillator sea una fuente de audio
// para que pueda ser reproducida por Rodio
impl Source for WaveTableOscillator {
    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    // Crea una wavetable con 64 muestras que representan una forma de onda senoidal
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin())
    }

    // Crea una instancia de WaveTableOscillator con una frecuencia de muestreo
    // de 44100Hz y la wavetable
    let mut oscillator = WaveTableOscillator::new(44100, wave_table);

    // Configura la nota que se va a tocar
    oscillator.set_frequency(30.0);

    // Crea un flujo de audio (OutputStream) y un manejador de flujo (stream_handle)
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Se reproduce el flujo de audio
    let _result = stream_handle.play_raw(oscillator.convert_samples());

    // La duración del audio dura 5 segundos
    std::thread::sleep(Duration::from_secs(5));
}
