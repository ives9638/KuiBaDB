/*
Copyright 2020 <盏一 w@hidva.com>
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum OidEnum {
    Template0Db = 1,
    KuiBaDb = 2,
    KB_CATALOG_NAMESPACE = 11,
    BOOLOID = 16,
    BoolInProc = 1242,
    BoolOutProc = 1243,
    BYTEAOID = 17,
    ByteaInProc = 1244,
    ByteaOutProc = 31,
    INT8OID = 20,
    Int8InProc = 460,
    Int8OutProc = 461,
    INT2OID = 21,
    Int2InProc = 38,
    Int2OutProc = 39,
    INT4OID = 23,
    Int4InProc = 42,
    Int4OutProc = 43,
    FLOAT4OID = 700,
    Float4InProc = 200,
    Float4OutProc = 201,
    FLOAT8OID = 701,
    Float8InProc = 214,
    Float8OutProc = 215,
    VARCHAROID = 1043,
    VarcharInProc = 1046,
    VarcharOutProc = 1047,
    TypeRelationId = 1247,
    AttributeRelationId = 1249,
    ProcedureRelationId = 1255,
    RelationRelationId = 1259,
    DatabaseRelationId = 1262,
    KB_PUBLIC_NAMESPACE = 2200,
    NamespaceRelationId = 2615,
    OperatorRelationId = 2617,
    MAX_OID = 16384, // The oid of system catalogs should be less than MAX_OID.
}

pub type Oid = std::num::NonZeroU32;
#[derive(Copy, Clone)]
pub struct OptOid(pub Option<Oid>);

impl std::convert::From<u32> for OptOid {
    fn from(val: u32) -> Self {
        Self(Oid::new(val))
    }
}

impl std::convert::From<OptOid> for u32 {
    fn from(val: OptOid) -> Self {
        match val.0 {
            None => 0,
            Some(v) => v.get(),
        }
    }
}

impl std::convert::From<OidEnum> for Oid {
    fn from(val: OidEnum) -> Oid {
        Oid::new(val as u32).unwrap()
    }
}