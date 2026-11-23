# Audio Stream Metadata
## Codec Parameters
Retrieve the following properties from the FLAC audio stream's codec 
parameters: number of audio channels, sample rate, bits per sample.

## Duration
duration = total number of samples / sample rate
duration = tick count * time base

## Errors
Flac supports between 1 and 8 channels:
0 - Invalid
1 - mono
2 - stereo
6 - 5.1 surround
8 - 7.1 surround
9 - Invalid

Flac supports sample rates from 1 Hz to 655,350 Hz:
0 Hz = Invalid, 0.5 Hz = Invalid, 1 Hz = Valid, 192,000 Hz = Valid, 
655,350 Hz = Valid, 655, 351 Hz = Invalid

Flac supports 4 to 32 bits per sample, inclusive, in 1-bit increments for 
linear integer PCM audio:
0 bits = Invalid, 3 bits = Invalid, 4 bits = Valid, 7 bits = Valid, 
32 bits = Valid, 33 bits = Invalid

## API
```
pub fn extract_audio_metadata(
    track: &Track,
) -> Result<AudioStreamMetadata, AudioStreamError> {}

pub fn validate_channels(
    classification: &AudioStreamClassification,
    channels: Option<usize>,
) -> Result<(), AudioStreamError> {}

pub fn validate_sample_rate(
    classification: &AudioStreamClassification,
    sample_rate: Option<u32>,
) -> Result<(), AudioStreamError> {}

pub fn validate_bits_per_sample(
    classification: &AudioStreamClassification,
    bits_per_sample: Option<u32>,
) -> Result<(), AudioStreamError> {}
```
