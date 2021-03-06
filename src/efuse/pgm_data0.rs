#[doc = "Reader of register PGM_DATA0"]
pub type R = crate::R<u32, super::PGM_DATA0>;
#[doc = "Writer for register PGM_DATA0"]
pub type W = crate::W<u32, super::PGM_DATA0>;
#[doc = "Register PGM_DATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PGM_DATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WR_DIS`"]
pub type WR_DIS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WR_DIS`"]
pub struct WR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wr_dis(&mut self) -> WR_DIS_W {
        WR_DIS_W { w: self }
    }
}
