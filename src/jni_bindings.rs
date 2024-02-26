// Import JNI dependencies
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring, jlong, jbyteArray};
use std::path::PathBuf;

// Import local module
// use crate::file_readers::file_formats::{FileFormat, FileFormatError};
use crate::{FileReader, Spectrum, QuadrupoleEvent};

// #[no_mangle]
// pub extern "system" fn Java_com_rogerli_timsrust_TimsRustDataReader_parseFileFormat<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, input: JString<'local>) -> jstring {
//     // Convert JString to Rust String
//     let input_path: String = env.get_string(&input).expect("Couldn't get java string!").into();
//
//     // Call the parse function from Rust, handling potential errors
//     let result = FileFormat::parse(PathBuf::from(input_path));
//
//     // Match on the result to handle both Ok and Err cases
//     let output = match result {
//         Ok(format) => match format {
//             FileFormat::DFolder(_) => "DFolder",
//             FileFormat::MS2Folder(_) => "MS2Folder",
//         },
//         Err(e) => match e {
//             FileFormatError::DirectoryDoesNotExist => "DirectoryDoesNotExist",
//             FileFormatError::BinaryFilesAreMissing => "BinaryFilesAreMissing",
//             FileFormatError::MetadataFilesAreMissing => "MetadataFilesAreMissing",
//             _ => "UnknownError",
//         },
//     };
//
//     // Convert the Rust String back into a JString and return it
//     env.new_string(output).expect("Couldn't create java string!").into_raw()
// }

// Create FileReader instance
#[no_mangle]
pub extern "system" fn Java_com_rogerli_timsrust_TimsRustDataReader_newFileReader<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, input: JString<'local>) -> jlong {
    let input_path: String = env.get_string(&input).expect("Couldn't get java string!").into();
    let reader = match FileReader::new(input_path) {
        Ok(reader) => Box::new(reader),
        Err(_) => return 0, // Simplified error handling
    };
    Box::into_raw(reader) as jlong
}

// Free FileReader instance
#[no_mangle]
pub extern "system" fn Java_com_rogerli_timsrust_TimsRustDataReader_freeFileReader<'local>(_env: JNIEnv<'local>, _class: JClass<'local>, reader_ptr: jlong) {
    unsafe {
        let _ = Box::from_raw(reader_ptr as *mut FileReader);
    }
}


#[no_mangle]
pub extern "system" fn Java_com_rogerli_timsrust_TimsRustDataReader_readAllSpectraAsBytes(env: JNIEnv, _class: JClass, file_reader_ptr: jlong) -> jbyteArray {
    let file_reader = unsafe { &*(file_reader_ptr as *const FileReader) };
    let spectra = file_reader.read_all_spectra(); // Call your existing Rust function

    let serialized_spectra = serialize_spectra(spectra); // Serialize the spectra

    // Convert the Rust Vec<u8> to a Java byte array
    let byte_array = env.byte_array_from_slice(&serialized_spectra).expect("Couldn't create java byte array");
    byte_array.into_raw()
}


use prost::Message;

fn serialize_spectra(spectra: Vec<Spectrum>) -> Vec<u8> {
    let proto_spectra = spectra.into_iter().map(|s| convert_spectrum_to_proto(&s)).collect::<Vec<_>>();
    let spectra_message = crate::spectra_proto::Spectra { spectra: proto_spectra };
    let mut buf = Vec::new();
    spectra_message.encode(&mut buf).expect("Failed to serialize spectra");
    buf
}

// Assuming this function is part of your FileReader implementation or somewhere appropriate
fn convert_spectrum_to_proto(spectrum: &Spectrum) -> crate::spectra_proto::Spectrum {
    let precursor = match &spectrum.precursor {
        QuadrupoleEvent::Precursor(p) => Some(crate::spectra_proto::QuadrupoleEvent {
            event: Some(crate::spectra_proto::quadrupole_event::Event::Precursor(
                crate::spectra_proto::Precursor {
                    mz: p.mz,
                    rt: p.rt,
                    im: p.im,
                    charge: p.charge as u32, // usize to u32
                    intensity: p.intensity,
                    index: p.index as u32, // usize to u32
                    frame_index: p.frame_index as u32, // usize to u32
                    collision_energy: p.collision_energy,
                },
            )),
        }),
        QuadrupoleEvent::None => None,
    };

    crate::spectra_proto::Spectrum {
        mz_values: spectrum.mz_values.clone(),
        intensities: spectrum.intensities.clone(),
        precursor,
        index: spectrum.index as u32, // usize to u32
    }
}




