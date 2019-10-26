/*
 * teleinfo-parser
 * Copyright (c) 2018, 2019 Nicolas PENINGUY.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.

 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use chrono::*;
use frame::*;
use std::io;

/// All the usefull informations that can be extracted when in the Heures Creuses tariff option.
#[derive(Debug)]
pub struct HcInfos {

    /// The date and time at which the frame was received.
    pub date: DateTime<Local>,
    /// The current tariff period, HP or HC
    pub periode: String,
    /// The value of the HC meter, in Wh.
    pub hc: i32,
    /// The value of the HP meter, in Wh.
    pub hp: i32,
    /// The current intensity in A (informative).
    pub iinst: i32,
    /// Apparent power, in W (informative).
    pub papp: i32,
    /// True if maximum subscribed intensity is exceeded.
    pub alerte: bool
}

struct HcInfosBuilder {

    date: Option<DateTime<Local>>,
    periode: Option<String>,
    hc: Option<i32>,
    hp: Option<i32>,
    iinst: Option<i32>,
    papp: Option<i32>,
    alerte: bool
}

macro_rules! get {
    ($e:expr, $msg:expr) => (match $e { Some(e) => e, None => return Err(TeleinfoError::FrameError($msg.to_string())) })
}

impl HcInfos {

    /// Try to read informations from the next frame. Any lowlevel error in the frame
    /// (e.g. wrong checksum) will be returned as is. Additionnaly, the function will
    /// ensure that all the expected fields are indeed present. If not, a FrameError will be
    /// returned.
    pub fn read<T: io::Read>(mut input: &mut T) -> Result<HcInfos, TeleinfoError> {

        let frame = Frame::next_frame(&mut input)?;

        return HcInfos::from(frame);
    }

    fn from(frame: Frame) -> Result<HcInfos, TeleinfoError> {

        let mut builder = HcInfosBuilder::new();
        let now: DateTime<Local> = Local::now();

        builder.date(now);

        for tag in frame.tags {
            match tag {
                Tag::PTEC(p) => {
                    builder.periode(match p {
                        PeriodeTarifaire::HP => "HP",
                        PeriodeTarifaire::HC => "HC",
                        _ => panic!("PeriodeTarifaire does not match HC")
                    });
                },
                Tag::HCHC(v) => {
                    builder.hc(v);
                },
                Tag::HCHP(v) => {
                    builder.hp(v);
                },
                Tag::IINST(v) => {
                    builder.iinst(v);
                },
                Tag::PAPP(v) => {
                    builder.papp(v);
                },
                Tag::ADPS(_) => {
                    builder.alerte(true);
                },
                _ => ()
            };
        }

        builder.build()
    }
}

impl HcInfosBuilder {

    fn new() -> HcInfosBuilder {
        HcInfosBuilder {
            date: None,
            periode: None,
            hc: None,
            hp: None,
            iinst: None,
            papp: None,
            alerte: false
        }
    }

    fn date(&mut self, date: DateTime<Local>) -> &mut HcInfosBuilder {
        self.date = Some(date);
        self
    }

    fn periode(&mut self, periode: &str) -> &mut HcInfosBuilder {
        self.periode = Some(periode.to_string());
        self
    }

    fn hc(&mut self, hc: i32) -> &mut HcInfosBuilder {
        self.hc = Some(hc);
        self
    }

    fn hp(&mut self, hp: i32) -> &mut HcInfosBuilder {
        self.hp = Some(hp);
        self
    }

    fn iinst(&mut self, iinst: i32) -> &mut HcInfosBuilder {
        self.iinst = Some(iinst);
        self
    }

    fn papp(&mut self, papp: i32) -> &mut HcInfosBuilder {
        self.papp = Some(papp);
        self
    }

    fn alerte(&mut self, alerte: bool) -> &mut HcInfosBuilder {
        self.alerte = alerte;
        self
    }

    fn build(self) -> Result<HcInfos, TeleinfoError> {
        let infos = HcInfos {
            date: get!(self.date, "Missing date"),
            periode: get!(self.periode, "Missing periode"),
            hc: get!(self.hc, "Missing hc"),
            hp: get!(self.hp, "Missing hp"),
            iinst: get!(self.iinst, "Missing iinst"),
            papp: get!(self.papp, "Missing papp"),
            alerte: self.alerte
        };

        Ok(infos)
    }
}
