use crate::serialize::buffer::*;
use serde::{ser, Serialize, Deserialize};
use std::io::{Read, Write};

use std::fmt::{self, Display};

use crate::serialize::string::{ReadString, WriteString};
use crate::serialize::var::{DeserializeError, ReadVarInt, WriteVarInt};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct MCProtoSerializer<W: Write> {
    pub writer: W,
}

impl<W: Write> MCProtoSerializer<W> {
    /// Creates a new Serializer with the given `Write`r.
    pub fn new(w: W) -> MCProtoSerializer<W> {
        MCProtoSerializer { writer: w }
    }
}

impl<'a, W: Write> serde::Serializer for &'a mut MCProtoSerializer<W> {
    type Ok = ();
    type Error = crate::serialize::error::Error;
    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        //self.writer.write_bool(v);
        //Ok(())
        unimplemented!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_i8(v);
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_i16::<BigEndian>(v);
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_i32::<BigEndian>(v);
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_i64::<BigEndian>(v);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_u8(v);
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_u16::<BigEndian>(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_u32::<BigEndian>(v);
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_u64::<BigEndian>(v);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_f32::<BigEndian>(v);
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_f64::<BigEndian>(v);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, val: &str) -> Result<Self::Ok, Self::Error> {
        //write_String(&val, &mut self.writer)
        //unimplemented!()
        self.writer.write_string(val);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write(value);
        Ok(()) //TODO handle
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&[0xff]);
        Ok(()) //TODO handle
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        //write_varint(&(variant_index as i32), &mut self.writer)
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound { ser: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        //write_varint(&(variant_index as i32), &mut self.writer);
        Ok(Compound { ser: self })
    }

    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        unimplemented!()
    }
}

pub struct Compound<'a, W: 'a + Write> {
    ser: &'a mut MCProtoSerializer<W>,
}

impl<'a, W> serde::ser::SerializeStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = crate::serialize::error::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = crate::serialize::error::Error;

    #[inline]
    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    #[inline]
    fn end(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn to_buffer<T>(value: &T) -> Result<ByteBuf, ()>
where
    T: ser::Serialize,
{
    let mut serializer = MCProtoSerializer::new(ByteBuf::new());
    value.serialize(&mut serializer).unwrap();
    Ok(serializer.writer)
}

pub struct VarInt {
    value: i32,
}

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut out: Vec<u8> = vec![];
        let mut value: i32 = self.value;
        if value == 0 {
            out.push(0_u8);
        }

        while value != 0 {
            let mut current_byte: u8 = (value & 0b01111111) as u8;
            // unsigned right shift
            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                current_byte |= 0b10000000;
            }
            out.push(current_byte);
        }

        serializer.serialize_bytes(out.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize::de::{deserialize, MCProtoDeserializer};
    use serde::Deserializer;

    #[derive(Serialize, Deserialize)]
    pub struct StructTest {
        id: i32,
        s: String,
        v: i16,
        varint: VarInt,
    }

    #[test]
    fn basic_serializer_test() {
        let v = StructTest {
            id: 55 as i32,
            s: "hello".to_string(),
            v: 555 as i16,
            varint: VarInt{ value: 1337},
        };
        let mut buf = to_buffer(&v).unwrap();
        let s = buf.as_slice();
        assert_eq!(14, buf.len());
        //assert_eq!(v.id, buf.read_i32::<BigEndian>().unwrap());
        //assert_eq!(v.s, buf.read_string().unwrap());
        //assert_eq!(v.v, buf.read_i16::<BigEndian>().unwrap());
        //assert_eq!(v.varint.value, buf.read_var_int().unwrap());
        let mut d = MCProtoDeserializer::new(buf);
        deserialize::<StructTest>(&d);
    }
}
