
use ::std::io::{self, Read};


const STX: char = '\u{0002}';
const ETX: char = '\u{0003}';
const EOT: char = '\u{0004}';

const LF: char = '\u{000A}';
const CR: char = '\u{000D}';

const SP: char = '\u{0020}';
// const HT: char = '\u{0009}';

const SEPARATOR: char = SP;

/// The subscribed tariff option
#[derive(Debug)]
pub enum OptionTarifaire {
    Base,
    HC,
    EJP,
    UNKNOWN(String)
}

/// The current period for the frame. Valid values depends on the subscribed plan.
#[derive(Debug)]
pub enum PeriodeTarifaire {
    /// Toutes Heures
    TH,
    /// Heures Creuses
    HC,
    /// Heures Pleines
    HP,
    /// Autre / non géré
    UNKNOWN(String)
}

/// The different tags that can compose a frame.
#[derive(Debug)]
pub enum Tag {
    /// Adresse du compteur
    ADCO(String),
    /// Option tarifaire choisie
    OPTARIF(OptionTarifaire),
    /// Intensité souscrite
    ISOUSC(i32),
    /// Index option Base
    BASE(i32),
    /// Index option Heures Creuses
    /// Heures Creuses
    HCHC(i32),
    /// Heures Pleines
    HCHP(i32),
    /// Période tarifaire en cours
    PTEC(PeriodeTarifaire),
    /// Intensité Instantanée
    IINST(i32),
    /// Avertissement de Dépassement de Puissance Souscrite
    ADPS(i32),
    /// Intensité maximale appelée
    IMAX(i32),
    /// Puissance apparente
    PAPP(i32),
    /// Horaire Heures Pleines Heures Creuses
    HHPHC(char),
    /// Mot d'état du compteur
    MOTDETAT(String),
    /// Groupe d'information inconnu ou non géré
    UNKNOWN(String, String)
}

/// A parsed and valid frame received from the TeleInfo line.
pub struct Frame {
    pub tags: Vec<Tag>
}

impl Frame {
    fn new() -> Frame {
        Frame {
            tags: Vec::new()
        }
    }

    pub fn next_frame(mut input: &mut Read) -> Result<Frame, TeleinfoError> {

        skip_to(&mut input, STX)?;

        return read_frame(&mut input);
    }
}

/// Possible fatal or non-fatal errors when reading frames from the serial port.
#[derive(Debug)]
pub enum TeleinfoError {
    /// Channel is closed.
    EndOfFile,
    /// A input/output error was encountered.
    IoError(io::Error),
    /// The transmission was interupted, for example because of external reading of the meter
    /// current value. Reading a new frame can be tried.
    EndOfTransmission,
    /// One tag had unexpected form, maybe because of transmission error. Try reading a new frame.
    FrameError(String),
    /// One tag had wrong checksum, maybe because of transmission error. Try reading a new frame.
    ChecksumError
}

impl From<io::Error> for TeleinfoError {

    fn from(err: io::Error) -> TeleinfoError {
        TeleinfoError::IoError(err)
    }
}

fn read_frame(mut input: &mut Read) -> Result<Frame, TeleinfoError> {

    let mut frame = Frame::new();

    loop {
        let c = read_char(&mut input)?;

        if c == ETX {
            return Ok(frame);
        }

        if c != LF {
            return Err(TeleinfoError::FrameError(format!("Expected LF but found {}", c)));
        }

        let lbl = read_to_sep(&mut input)?;
        let val = read_to_sep(&mut input)?;
        let checksum = read_char(&mut input)?;

        let mut sum = 0u8;

        for c in lbl.chars() {
            sum = sum.wrapping_add(c as u8);
        }

        sum = sum.wrapping_add(SEPARATOR as u8);

        for c in val.chars() {
            sum = sum.wrapping_add(c as u8);
        }

        // TODO: mode de calcul 2, rajouter le séparateur après valeur
        // sum = sum.wrapping_add(SEPARATOR as u8);

        let sum = ((sum & 0x3F) + 0x20) as char;

        if sum != checksum {
            return Err(TeleinfoError::ChecksumError);
        }

        let tag = parse_tag(&lbl, &val)?;

        frame.tags.push(tag);


        expect_char(&mut input, CR)?;
    }
}

fn parse_tag(lbl: &str, val: &str) -> Result<Tag, TeleinfoError> {

    let tag = match lbl {

        "ADCO" => Tag::ADCO(val.to_string()),

        "OPTARIF" => {
            Tag::OPTARIF(match val {
                "Base" => OptionTarifaire::Base,
                "HC.." => OptionTarifaire::HC,
                "EJP." => OptionTarifaire::EJP,
                _ => OptionTarifaire::UNKNOWN(val.to_string())
            })
        },

        "ISOUSC" => {
            let p = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::ISOUSC(p)
        },

        "BASE" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::BASE(v)
        },

        "HCHC" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::HCHC(v)
        },

        "HCHP" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::HCHP(v)
        },

        "PTEC" => {
            Tag::PTEC(match val {
                "TH.." => PeriodeTarifaire::TH,
                "HC.." => PeriodeTarifaire::HC,
                "HP.." => PeriodeTarifaire::HP,
                _ => PeriodeTarifaire::UNKNOWN(val.to_string())
            })
        },

        "IINST" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::IINST(v)
        },

        "IMAX" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::IMAX(v)
        },

        "ADPS" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::ADPS(v)
        },

        "PAPP" => {
            let v = val.parse::<i32>()
                .map_err(|_| TeleinfoError::FrameError(format!("Number parse error on {}", val)))?;
            Tag::PAPP(v)
        },

        "HHPHC" => {
            let c = match val.chars().next() {
                Some(c) => c,
                None => return Err(TeleinfoError::FrameError("HHPHC should be one char long".to_string()))
            };
            Tag::HHPHC(c)
        },

        "MOTDETAT" => Tag::MOTDETAT(val.to_string()),

        _ => Tag::UNKNOWN(lbl.to_string(), val.to_string())
    };

    Ok(tag)
}

fn skip_to(mut input: &mut Read, stop_char: char) -> Result<(), TeleinfoError> {

    loop {
        let c = read_char(&mut input)?;

        if c == stop_char {
            break;
        }
    }

    Ok(())
}

fn read_to_sep(mut input: &mut Read) -> Result<String, TeleinfoError> {

    let mut result: String = String::new();

    loop {
        let c = read_char(&mut input)?;

        if c ==  SEPARATOR {
            break;
        }

        result.push(c);
    }

    Ok(result)
}

fn expect_char(mut input: &mut Read, expected: char) -> Result<(), TeleinfoError> {

    let c = read_char(&mut input)?;

    if c != expected {
        return Err(TeleinfoError::FrameError(format!("Expected {} but found {}", expected, c)));
    }

    Ok(())
}

fn read_char(input: &mut Read) -> Result<char, TeleinfoError> {

    let mut buf = [0u8; 1];
    let count = input.read(&mut buf)?;

    if count == 0 {
        return Err(TeleinfoError::EndOfFile);
    }

    let c = buf[0] as char;

    if c == EOT {
        return Err(TeleinfoError::EndOfTransmission);
    }

    return Ok(c);
}
