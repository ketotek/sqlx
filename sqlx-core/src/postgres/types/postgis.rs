use postgis::ewkb::GeometryZ;
use postgis::ewkb::{AsEwkbGeometry, EwkbRead, EwkbWrite};
use std::io::{Cursor, Seek};
use std::ops::DerefMut;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres};
use crate::types::Type;

impl Type<Postgres> for GeometryZ {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::GEOGRAPHY
    }
}

impl Encode<'_, Postgres> for GeometryZ {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        let mut cursor = Cursor::new(buf.deref_mut());
        if cursor.seek(std::io::SeekFrom::End(0)).is_err() {
            return IsNull::Yes;
        }

        if self.as_ewkb().write_ewkb(&mut cursor).is_err() {
            return IsNull::Yes;
        }

        IsNull::No
    }
}

impl Decode<'_, Postgres> for GeometryZ {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let mut cursor = Cursor::new(value.as_bytes()?);
        Self::read_ewkb(&mut cursor).map_err(|e| e.to_string().into())
    }
}
