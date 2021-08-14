// Generated by Molecule 0.7.0

use super::super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct AnyoneShutdownSidechainWitness(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for AnyoneShutdownSidechainWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for AnyoneShutdownSidechainWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for AnyoneShutdownSidechainWitness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "challenge_times", self.challenge_times())?;
        write!(f, ", {}: {}", "check_data_size", self.check_data_size())?;
        write!(f, ", {}: {}", "jailed_checkers", self.jailed_checkers())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for AnyoneShutdownSidechainWitness {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            40, 0, 0, 0, 16, 0, 0, 0, 20, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        AnyoneShutdownSidechainWitness::new_unchecked(v.into())
    }
}
impl AnyoneShutdownSidechainWitness {
    pub const FIELD_COUNT: usize = 3;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn challenge_times(&self) -> Uint32 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Uint32::new_unchecked(self.0.slice(start..end))
    }

    pub fn check_data_size(&self) -> Uint128 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Uint128::new_unchecked(self.0.slice(start..end))
    }

    pub fn jailed_checkers(&self) -> PubKeyHashList {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            PubKeyHashList::new_unchecked(self.0.slice(start..end))
        } else {
            PubKeyHashList::new_unchecked(self.0.slice(start..))
        }
    }

    pub fn as_reader<'r>(&'r self) -> AnyoneShutdownSidechainWitnessReader<'r> {
        AnyoneShutdownSidechainWitnessReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for AnyoneShutdownSidechainWitness {
    type Builder = AnyoneShutdownSidechainWitnessBuilder;

    const NAME: &'static str = "AnyoneShutdownSidechainWitness";

    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        AnyoneShutdownSidechainWitness(data)
    }

    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }

    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        AnyoneShutdownSidechainWitnessReader::from_slice(slice).map(|reader| reader.to_entity())
    }

    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        AnyoneShutdownSidechainWitnessReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }

    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }

    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .challenge_times(self.challenge_times())
            .check_data_size(self.check_data_size())
            .jailed_checkers(self.jailed_checkers())
    }
}
#[derive(Clone, Copy)]
pub struct AnyoneShutdownSidechainWitnessReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for AnyoneShutdownSidechainWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for AnyoneShutdownSidechainWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for AnyoneShutdownSidechainWitnessReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "challenge_times", self.challenge_times())?;
        write!(f, ", {}: {}", "check_data_size", self.check_data_size())?;
        write!(f, ", {}: {}", "jailed_checkers", self.jailed_checkers())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> AnyoneShutdownSidechainWitnessReader<'r> {
    pub const FIELD_COUNT: usize = 3;

    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }

    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }

    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }

    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }

    pub fn challenge_times(&self) -> Uint32Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Uint32Reader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn check_data_size(&self) -> Uint128Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Uint128Reader::new_unchecked(&self.as_slice()[start..end])
    }

    pub fn jailed_checkers(&self) -> PubKeyHashListReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            PubKeyHashListReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            PubKeyHashListReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for AnyoneShutdownSidechainWitnessReader<'r> {
    type Entity = AnyoneShutdownSidechainWitness;

    const NAME: &'static str = "AnyoneShutdownSidechainWitnessReader";

    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }

    fn new_unchecked(slice: &'r [u8]) -> Self {
        AnyoneShutdownSidechainWitnessReader(slice)
    }

    fn as_slice(&self) -> &'r [u8] {
        self.0
    }

    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        Uint32Reader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Uint128Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        PubKeyHashListReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct AnyoneShutdownSidechainWitnessBuilder {
    pub(crate) challenge_times: Uint32,
    pub(crate) check_data_size: Uint128,
    pub(crate) jailed_checkers: PubKeyHashList,
}
impl AnyoneShutdownSidechainWitnessBuilder {
    pub const FIELD_COUNT: usize = 3;

    pub fn challenge_times(mut self, v: Uint32) -> Self {
        self.challenge_times = v;
        self
    }

    pub fn check_data_size(mut self, v: Uint128) -> Self {
        self.check_data_size = v;
        self
    }

    pub fn jailed_checkers(mut self, v: PubKeyHashList) -> Self {
        self.jailed_checkers = v;
        self
    }
}
impl molecule::prelude::Builder for AnyoneShutdownSidechainWitnessBuilder {
    type Entity = AnyoneShutdownSidechainWitness;

    const NAME: &'static str = "AnyoneShutdownSidechainWitnessBuilder";

    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.challenge_times.as_slice().len()
            + self.check_data_size.as_slice().len()
            + self.jailed_checkers.as_slice().len()
    }

    fn write<W: ::molecule::io::Write>(&self, writer: &mut W) -> ::molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.challenge_times.as_slice().len();
        offsets.push(total_size);
        total_size += self.check_data_size.as_slice().len();
        offsets.push(total_size);
        total_size += self.jailed_checkers.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.challenge_times.as_slice())?;
        writer.write_all(self.check_data_size.as_slice())?;
        writer.write_all(self.jailed_checkers.as_slice())?;
        Ok(())
    }

    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        AnyoneShutdownSidechainWitness::new_unchecked(inner.into())
    }
}