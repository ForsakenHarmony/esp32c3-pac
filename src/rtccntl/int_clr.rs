#[doc = "Writer for register INT_CLR"]
pub type W = crate::W<u32, super::INT_CLR>;
#[doc = "Register INT_CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BBPLL_CAL_INT_CLR`"]
pub struct BBPLL_CAL_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_CAL_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `GLITCH_DET_INT_CLR`"]
pub struct GLITCH_DET_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `XTAL32K_DEAD_INT_CLR`"]
pub struct XTAL32K_DEAD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_DEAD_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `SWD_INT_CLR`"]
pub struct SWD_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `MAIN_TIMER_INT_CLR`"]
pub struct MAIN_TIMER_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `BROWN_OUT_INT_CLR`"]
pub struct BROWN_OUT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `WDT_INT_CLR`"]
pub struct WDT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `SLP_REJECT_INT_CLR`"]
pub struct SLP_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `SLP_WAKEUP_INT_CLR`"]
pub struct SLP_WAKEUP_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bbpll_cal_int_clr(&mut self) -> BBPLL_CAL_INT_CLR_W {
        BBPLL_CAL_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn glitch_det_int_clr(&mut self) -> GLITCH_DET_INT_CLR_W {
        GLITCH_DET_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xtal32k_dead_int_clr(&mut self) -> XTAL32K_DEAD_INT_CLR_W {
        XTAL32K_DEAD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swd_int_clr(&mut self) -> SWD_INT_CLR_W {
        SWD_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn main_timer_int_clr(&mut self) -> MAIN_TIMER_INT_CLR_W {
        MAIN_TIMER_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn brown_out_int_clr(&mut self) -> BROWN_OUT_INT_CLR_W {
        BROWN_OUT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt_int_clr(&mut self) -> WDT_INT_CLR_W {
        WDT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_reject_int_clr(&mut self) -> SLP_REJECT_INT_CLR_W {
        SLP_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slp_wakeup_int_clr(&mut self) -> SLP_WAKEUP_INT_CLR_W {
        SLP_WAKEUP_INT_CLR_W { w: self }
    }
}
