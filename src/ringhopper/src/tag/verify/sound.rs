use std::usize;

use primitives::{dynamic::DynamicTagDataArray, primitive::TagPath, tag::PrimaryTagStructDyn};
use ringhopper_structs::{Sound, SoundChannelCount, SoundFormat};

use crate::tag::{sound::{sample_rate_to_u32, SoundPermutationMetadata}, tree::TagTree};

use super::{VerifyContext, VerifyResult};

pub fn verify_sound<T: TagTree + Send + Sync>(tag: &dyn PrimaryTagStructDyn, _path: &TagPath, _context: &VerifyContext<T>, result: &mut VerifyResult) {
    let sound: &Sound = tag.as_any().downcast_ref().unwrap();
    verify_sound_permutation_indices(sound, result);
    verify_sound_formats(sound, result);
    verify_sound_buffer_size(sound, result);

    for (p, pitch_range) in ziperator!(sound.pitch_ranges) {
        let actual_natural_pitch = if pitch_range.natural_pitch <= 0.0 { 1.0 } else { pitch_range.natural_pitch };

        let pitch_range_ok = if pitch_range.bend_bounds.lower > actual_natural_pitch && pitch_range.bend_bounds.lower != 0.0 {
            false
        }
        else if pitch_range.bend_bounds.upper < actual_natural_pitch && pitch_range.bend_bounds.upper != 0.0 {
            false
        }
        else {
            true
        };

        if !pitch_range_ok {
            result.warnings.push(format!("Pitch range #{p}'s bend bounds does not fit the natural pitch value ({actual_natural_pitch}) and will be adjusted."));
        }
    }
}

pub fn sound_is_playable(sound: &Sound) -> bool {
    let mut result = VerifyResult::default();
    verify_sound_permutation_indices(sound, &mut result);
    verify_sound_formats(sound, &mut result);
    verify_sound_buffer_size(sound, &mut result);
    result.is_ok()
}

fn verify_sound_buffer_size(sound: &Sound, result: &mut VerifyResult) {
    let channel_count = match sound.channel_count {
        SoundChannelCount::Mono => 1,
        SoundChannelCount::Stereo => 2
    };
    for (pr, pitch_range) in ziperator!(sound.pitch_ranges) {
        for (pe, permutation) in ziperator!(pitch_range.permutations) {
            match permutation.format {
                SoundFormat::PCM => {
                    let data = permutation.samples.bytes.len();
                    let sample_size = 2 * channel_count;
                    if (data % sample_size) != 0 {
                        result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has an incorrect size (not divisible by {sample_size})", permutation.name.as_str()));
                        continue;
                    }
                },
                SoundFormat::XboxADPCM => {
                    let block_size = 36 * channel_count;
                    let data = permutation.samples.bytes.len();
                    if (data % block_size) != 0 {
                        result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has an incorrect size (not divisible by {block_size})", permutation.name.as_str()));
                        continue;
                    }
                }
                _ => ()
            }

            let metadata = match SoundPermutationMetadata::read_from_sound_permutation(sound, permutation) {
                Ok(n) => n,
                Err(e) => {
                    result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} had an error while querying sound metadata: {e}", permutation.name.as_str()));
                    continue;
                }
            };

            if sound.channel_count != metadata.channel_count {
                result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has a mismatched channel count (permutation is {}, where sound tag is {})", permutation.name.as_str(), metadata.channel_count, sound.channel_count));
            }

            if sound.sample_rate != metadata.sample_rate {
                result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has a mismatched sample rate (permutation is {} Hz, where sound tag is {} Hz)", permutation.name.as_str(), sample_rate_to_u32(metadata.sample_rate), sample_rate_to_u32(sound.sample_rate)));
            }

            if permutation.buffer_size != metadata.buffer_size {
                result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has a mismatched buffer size (permutation is {}, where sound tag is {})", permutation.name.as_str(), metadata.buffer_size, permutation.buffer_size));
            }
        }
    }
}

fn verify_sound_formats(sound: &Sound, result: &mut VerifyResult) {
    let expected = sound.format;
    for (pr, pitch_range) in ziperator!(sound.pitch_ranges) {
        for (pe, permutation) in ziperator!(pitch_range.permutations) {
            let actual = permutation.format;
            if expected != actual {
                result.errors.push(format!("Permutation #{pe} (`{}`) of pitch range #{pr} has a mismatched sound format (expected {expected}, got {actual} instead)", permutation.name.as_str()));
            }
        }
    }
}

fn verify_sound_permutation_indices(sound: &Sound, result: &mut VerifyResult) {
    let split_permutations = sound.flags.split_long_sound_into_permutations;
    if !split_permutations {
        return
    }

    for (pr, pitch_range) in ziperator!(sound.pitch_ranges) {
        let subpermutation_count = pitch_range.permutations.len();

        let actual_permutation_count = if split_permutations {
            pitch_range.actual_permutation_count as usize
        }
        else {
            subpermutation_count
        };

        let actual_permutations = match pitch_range.permutations.items.get(0..actual_permutation_count) {
            Some(n) => n,
            None => {
                result.errors.push(format!("Pitch range #{pr} has {actual_permutation_count} actual permutations, but only {} permutations", pitch_range.permutations.items.len()));
                continue
            }
        };

        let mut used: Vec<bool> = vec![false; subpermutation_count];

        'permutation_loop:
        for (ap, permutation) in (0..actual_permutations.len()).zip(actual_permutations.iter()) {
            let mut traversed = 0usize;

            let mut permutation_index = ap;
            loop {
                if permutation_index > subpermutation_count {
                    result.errors.push(format!("Permutation #{ap} (`{}`) of pitch range #{pr} has out-of-bounds permutations", permutation.name.as_str()));
                    continue 'permutation_loop;
                }

                traversed += 1;
                if traversed > subpermutation_count {
                    result.errors.push(format!("Permutation #{ap} (`{}`) of pitch range #{pr} infinitely loops", permutation.name.as_str()));
                    continue 'permutation_loop;
                }

                used[permutation_index] = true;
                permutation_index = match pitch_range.permutations.items[permutation_index].next_permutation_index {
                    Some(n) => n as usize,
                    None => break
                };
            }
        }

        let unused_permutations_total = used.into_iter().filter(|&p| p == false).count();
        if unused_permutations_total > 0 {
            result.warnings.push(format!("Pitch range #{pr} contains {unused_permutations_total} unused subpermutation(s)"));
        }
    }
}
