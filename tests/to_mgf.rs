use std::fs::File;
use std::io;
use std::io::Write;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use chrono::Local;
use std::time::{Instant, Duration};


#[derive(Default)]
pub struct Precursor {
    pub mz: f32,
    pub intensity: Option<f32>,
    pub charge: Option<u8>,
    pub spectrum_ref: Option<String>,
}

pub struct RawSpectrum {
    pub file_id: usize,
    pub ms_level: u8,
    pub id: String,
    pub precursors: Vec<Precursor>,
    pub representation: Representation,
    pub scan_start_time: f32,
    pub ion_injection_time: f32,
    pub total_ion_current: f32,
    pub mz: Vec<f32>,
    pub intensity: Vec<f32>,
}

pub enum Representation {
    Profile,
    Centroid,
}

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
                intensity: dda_spectrum.intensities.iter().map(|&x| x as f32).collect(),
            };
            spectrum
        })
        .collect();
    Ok(spectra)
}

#[test]
fn test_mgf() -> anyhow::Result<()> {
    let start = Instant::now();

    let rootDir = "";
    let runName = "";

    let p = rootDir.to_string() + &runName + ".d";
    println!("Reading {:?}", p);

    let parse = parse(p, 1);

    println!("Time elapsed in parse is: {:?}", start.elapsed());

    let p2 = rootDir.to_string() + &runName + ".mgf";
    println!("Writing {:?}", p2);

    let outfile = File::create(p2)?;
    let mut w = io::BufWriter::new(outfile);
    parse.unwrap().iter().for_each(|x| {
        if x.precursors[0].mz==0.0 {
            return;
        }
        if x.mz.len() == 0 {
            return;
        }
        let l = format!("BEGIN IONS\nTITLE={}.{}.{}.{}\nRTINSECONDS={}\nPEPMASS={}\nCHARGE={}+\n{}\nEND IONS\n",
                        runName, x.id, x.id, x.precursors[0].charge.unwrap(),
                        x.scan_start_time, x.precursors[0].mz, x.precursors[0].charge.unwrap(),
                        std::iter::zip(&x.mz,&x.intensity).map(|(&mz, &intensity)| format!("{} {}", mz, intensity)).collect::<Vec<_>>().join("\n")
        );
        w.write_all(l.as_ref()).unwrap();
    });
    Ok(())
}

