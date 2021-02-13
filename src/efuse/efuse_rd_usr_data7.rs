#[doc = "Reader of register EFUSE_RD_USR_DATA7"]
pub type R = crate::R<u32, super::EFUSE_RD_USR_DATA7>;
#[doc = "Reader of field `EFUSE_USR_DATA7`"]
pub type EFUSE_USR_DATA7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_usr_data7(&self) -> EFUSE_USR_DATA7_R {
        EFUSE_USR_DATA7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}