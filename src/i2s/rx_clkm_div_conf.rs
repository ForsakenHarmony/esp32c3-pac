#[doc = "Reader of register RX_CLKM_DIV_CONF"]
pub type R = crate::R<u32, super::RX_CLKM_DIV_CONF>;
#[doc = "Writer for register RX_CLKM_DIV_CONF"]
pub type W = crate::W<u32, super::RX_CLKM_DIV_CONF>;
#[doc = "Register RX_CLKM_DIV_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::RX_CLKM_DIV_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_CLKM_DIV_YN1`"]
pub type RX_CLKM_DIV_YN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CLKM_DIV_YN1`"]
pub struct RX_CLKM_DIV_YN1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_YN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RX_CLKM_DIV_X`"]
pub type RX_CLKM_DIV_X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_CLKM_DIV_X`"]
pub struct RX_CLKM_DIV_X_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 18)) | (((value as u32) & 0x01ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `RX_CLKM_DIV_Y`"]
pub type RX_CLKM_DIV_Y_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_CLKM_DIV_Y`"]
pub struct RX_CLKM_DIV_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | (((value as u32) & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Reader of field `RX_CLKM_DIV_Z`"]
pub type RX_CLKM_DIV_Z_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RX_CLKM_DIV_Z`"]
pub struct RX_CLKM_DIV_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CLKM_DIV_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rx_clkm_div_yn1(&self) -> RX_CLKM_DIV_YN1_R {
        RX_CLKM_DIV_YN1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 18:26"]
    #[inline(always)]
    pub fn rx_clkm_div_x(&self) -> RX_CLKM_DIV_X_R {
        RX_CLKM_DIV_X_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn rx_clkm_div_y(&self) -> RX_CLKM_DIV_Y_R {
        RX_CLKM_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rx_clkm_div_z(&self) -> RX_CLKM_DIV_Z_R {
        RX_CLKM_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rx_clkm_div_yn1(&mut self) -> RX_CLKM_DIV_YN1_W {
        RX_CLKM_DIV_YN1_W { w: self }
    }
    #[doc = "Bits 18:26"]
    #[inline(always)]
    pub fn rx_clkm_div_x(&mut self) -> RX_CLKM_DIV_X_W {
        RX_CLKM_DIV_X_W { w: self }
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn rx_clkm_div_y(&mut self) -> RX_CLKM_DIV_Y_W {
        RX_CLKM_DIV_Y_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rx_clkm_div_z(&mut self) -> RX_CLKM_DIV_Z_W {
        RX_CLKM_DIV_Z_W { w: self }
    }
}
