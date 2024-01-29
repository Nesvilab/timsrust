use std::fs::File;
use std::io;
use std::io::Write;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use sage_core::spectrum::{Precursor, RawSpectrum, Representation};

pub fn parse(
    path_name: impl AsRef<str>,
    file_id: usize,
) -> Result<Vec<RawSpectrum>, timsrust::Error> {
    let dda_spectra: Vec<timsrust::Spectrum> =
        timsrust::FileReader::new(path_name.as_ref())?.read_all_spectra();
    let spectra: Vec<RawSpectrum> = (0..dda_spectra.len())
        .into_par_iter()
        .map(|index| {
            let dda_spectrum = &dda_spectra[index];
            let mut precursor: Precursor = Precursor::default();
            let dda_precursor: timsrust::Precursor =
                dda_spectrum.precursor.unwrap_as_precursor();
            precursor.mz = dda_precursor.mz as f32;
            precursor.charge = Option::from(dda_precursor.charge as u8);
            // precursor.ion_mobility = Option::from(dda_precursor.im as f32);
            precursor.intensity = Option::from(dda_precursor.intensity as f32);
            precursor.spectrum_ref = Option::from(dda_precursor.frame_index.to_string());
            let spectrum: RawSpectrum = RawSpectrum {
                file_id,
                precursors: vec![precursor],
                representation: Representation::Centroid,
                scan_start_time: dda_precursor.rt as f32 / 60.0,
                ion_injection_time: dda_precursor.rt as f32,
                total_ion_current: 0.0,
                mz: dda_spectrum.mz_values.iter().map(|&x| x as f32).collect(),
                ms_level: 2,
                id: dda_precursor.index.to_string(),
                // precursor_id: dda_precursor.index as u32,
                // frame_id: dda_precursor.frame_index as u32,
                intensity: dda_spectrum.intensities.iter().map(|&x| x as f32).collect(),
            };
            spectrum
        })
        .collect();
    Ok(spectra)
}

#[test]
fn test_mgf() -> anyhow::Result<()> {
    let parse = sage_cloudpath::tdf::TdfReader.parse("F:\\data\\PXD014777\\20180809_120min_200ms_WEHI25_brute20k_timsON_100ng_HYE124A_Slot1-7_1_890.d", 1);
    let outfile = File::create("F:\\data\\PXD014777\\20180809_120min_200ms_WEHI25_brute20k_timsON_100ng_HYE124A_Slot1-7_1_890.mgf")?;
    let mut w = io::BufWriter::new(outfile);
    parse.unwrap().iter().for_each(|x| {
        if x.precursors[0].mz==0.0 {
            return;
        }
        let l = format!("BEGIN IONS\nTITLE=\"index: {}, intensity: {}, average_mz: {}\"\nRTINSECONDS={}\nPEPMASS={}\nCHARGE={}+\n{}\nEND IONS\n",
                        x.id, x.precursors[0].intensity.unwrap(),
                        x.precursors[0].mz,
                        //x.mz.iter().sum::<f32>() / x.intensity.len() as f32,
                        x.scan_start_time, x.precursors[0].mz, x.precursors[0].charge.unwrap(),
                        // x.mz.iter().zip(&x.intensity).map(|(&mz, &intensity)| format!("{} {}", mz, intensity)).collect::<Vec<_>>().join("\n")
                        std::iter::zip(&x.mz,&x.intensity).map(|(&mz, &intensity)| format!("{} {}", mz, intensity)).collect::<Vec<_>>().join("\n")
        );
        w.write_all(l.as_ref()).unwrap();
    });
    Ok(())
}

