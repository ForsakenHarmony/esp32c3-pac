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
#[doc = "Write proxy for field `OVF_CNT_LSCH5_INT_CLR`"]
pub struct OVF_CNT_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH5_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `OVF_CNT_LSCH4_INT_CLR`"]
pub struct OVF_CNT_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `OVF_CNT_LSCH3_INT_CLR`"]
pub struct OVF_CNT_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `OVF_CNT_LSCH2_INT_CLR`"]
pub struct OVF_CNT_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `OVF_CNT_LSCH1_INT_CLR`"]
pub struct OVF_CNT_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `OVF_CNT_LSCH0_INT_CLR`"]
pub struct OVF_CNT_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH0_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH5_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH5_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH4_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH4_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH3_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH3_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH2_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH2_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH1_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH1_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `DUTY_CHNG_END_LSCH0_INT_CLR`"]
pub struct DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH0_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `LSTIMER3_OVF_INT_CLR`"]
pub struct LSTIMER3_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LSTIMER2_OVF_INT_CLR`"]
pub struct LSTIMER2_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_OVF_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `LSTIMER1_OVF_INT_CLR`"]
pub struct LSTIMER1_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_OVF_INT_CLR_W<'a> {
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
#[doc = "Write proxy for field `LSTIMER0_OVF_INT_CLR`"]
pub struct LSTIMER0_OVF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_OVF_INT_CLR_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_clr(&mut self) -> OVF_CNT_LSCH5_INT_CLR_W {
        OVF_CNT_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_clr(&mut self) -> OVF_CNT_LSCH4_INT_CLR_W {
        OVF_CNT_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_clr(&mut self) -> OVF_CNT_LSCH3_INT_CLR_W {
        OVF_CNT_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_clr(&mut self) -> OVF_CNT_LSCH2_INT_CLR_W {
        OVF_CNT_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_clr(&mut self) -> OVF_CNT_LSCH1_INT_CLR_W {
        OVF_CNT_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_clr(&mut self) -> OVF_CNT_LSCH0_INT_CLR_W {
        OVF_CNT_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_clr(&mut self) -> DUTY_CHNG_END_LSCH5_INT_CLR_W {
        DUTY_CHNG_END_LSCH5_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_clr(&mut self) -> DUTY_CHNG_END_LSCH4_INT_CLR_W {
        DUTY_CHNG_END_LSCH4_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_clr(&mut self) -> DUTY_CHNG_END_LSCH3_INT_CLR_W {
        DUTY_CHNG_END_LSCH3_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_clr(&mut self) -> DUTY_CHNG_END_LSCH2_INT_CLR_W {
        DUTY_CHNG_END_LSCH2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_clr(&mut self) -> DUTY_CHNG_END_LSCH1_INT_CLR_W {
        DUTY_CHNG_END_LSCH1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_clr(&mut self) -> DUTY_CHNG_END_LSCH0_INT_CLR_W {
        DUTY_CHNG_END_LSCH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lstimer3_ovf_int_clr(&mut self) -> LSTIMER3_OVF_INT_CLR_W {
        LSTIMER3_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lstimer2_ovf_int_clr(&mut self) -> LSTIMER2_OVF_INT_CLR_W {
        LSTIMER2_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lstimer1_ovf_int_clr(&mut self) -> LSTIMER1_OVF_INT_CLR_W {
        LSTIMER1_OVF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lstimer0_ovf_int_clr(&mut self) -> LSTIMER0_OVF_INT_CLR_W {
        LSTIMER0_OVF_INT_CLR_W { w: self }
    }
}
