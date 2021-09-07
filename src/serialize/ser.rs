use crate::serialize::buffer::*;
use serde::{ser, Serialize};
use std::io::{Read, Write};

use std::fmt::{self, Display};

use crate::serialize::var::{DeserializeError, VarIntReader, VarIntWriter};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::{IntPriv, Integer, Value};

pub struct BufSerializer {
    buf: ByteBuf,
}

pub fn to_buffer<T>(value: &T) -> Result<ByteBuf, ()>
where
    T: ser::Serialize,
{
    let mut serializer = BufSerializer {
        buf: ByteBuf::new(),
    };
    value.serialize(&mut serializer).unwrap();
    Ok(serializer.buf)
}

#[derive(Clone, Debug, PartialEq)]
pub enum SerializeError {
    Message(String),
    Unimplemented,
}

impl ser::Error for SerializeError {
    fn custom<T: Display>(msg: T) -> Self {
        SerializeError::Message(msg.to_string())
    }
}

impl Display for SerializeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SerializeError::Message(msg) => formatter.write_str(msg),
            SerializeError::Unimplemented => formatter.write_str("none"),
        }
    }
}

impl std::error::Error for SerializeError {}

impl<'a> ser::SerializeStruct for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;
    fn serialize_field<T>(&mut self, name: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        println!("CALLED");
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeMap for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _: &T) -> Result<(), Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;
    fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;
    fn serialize_field<T: ?Sized>(&mut self, _: &'static str, _: &T) -> Result<(), Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTuple for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut BufSerializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> 
where
        T: Serialize,
    {
        println!("called serialize_element!");
        value.serialize(&mut *self);
        
        Err(SerializeError::Unimplemented)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

impl serde::ser::SerializeSeq for BufSerializer {
    type Ok = ();
    type Error = SerializeError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        println!("SerializeSeq called");
        serde::ser::SerializeSeq::serialize_element(self, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        println!("SerializeSeq end called");
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::Serializer for &'a mut BufSerializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = SerializeError;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.buf.write_i8(v as i8).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.buf.write_i8(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.buf.write_i16::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.buf.write_u64::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.buf.write_i32::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.buf.write_u32::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.buf.write_i64::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.buf.write_u8(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.buf.write_u16::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.buf.write_f32::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.buf.write_f64::<BigEndian>(v).unwrap();
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.buf.write_var_int(v.len() as i32);
        self.buf.write(v.as_bytes());
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.buf.write(v);
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializeError::Unimplemented)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(&mut *self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerializeError::Unimplemented)
    }
}

#[derive(Serialize)]
pub struct StructTest {
    id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_serialize_struct() {
        let v = StructTest { id: 55 as i32 };
        let buf = to_buffer(&v).unwrap();
        let s = buf.as_slice();
        assert_eq!(1, buf.len());
    }
}
