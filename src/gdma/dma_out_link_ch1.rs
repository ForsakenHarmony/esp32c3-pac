#[doc = "Reader of register DMA_OUT_LINK_CH1"]
pub type R = crate::R<u32, super::DMA_OUT_LINK_CH1>;
#[doc = "Writer for register DMA_OUT_LINK_CH1"]
pub type W = crate::W<u32, super::DMA_OUT_LINK_CH1>;
#[doc = "Register DMA_OUT_LINK_CH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_OUT_LINK_CH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_OUTLINK_PARK_CH1`"]
pub type DMA_OUTLINK_PARK_CH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_OUTLINK_RESTART_CH1`"]
pub type DMA_OUTLINK_RESTART_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTLINK_RESTART_CH1`"]
pub struct DMA_OUTLINK_RESTART_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_RESTART_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DMA_OUTLINK_START_CH1`"]
pub type DMA_OUTLINK_START_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTLINK_START_CH1`"]
pub struct DMA_OUTLINK_START_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_START_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DMA_OUTLINK_STOP_CH1`"]
pub type DMA_OUTLINK_STOP_CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_OUTLINK_STOP_CH1`"]
pub struct DMA_OUTLINK_STOP_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_STOP_CH1_W<'a> {
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
#[doc = "Reader of field `DMA_OUTLINK_ADDR_CH1`"]
pub type DMA_OUTLINK_ADDR_CH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMA_OUTLINK_ADDR_CH1`"]
pub struct DMA_OUTLINK_ADDR_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_ADDR_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_outlink_park_ch1(&self) -> DMA_OUTLINK_PARK_CH1_R {
        DMA_OUTLINK_PARK_CH1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_outlink_restart_ch1(&self) -> DMA_OUTLINK_RESTART_CH1_R {
        DMA_OUTLINK_RESTART_CH1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_outlink_start_ch1(&self) -> DMA_OUTLINK_START_CH1_R {
        DMA_OUTLINK_START_CH1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_outlink_stop_ch1(&self) -> DMA_OUTLINK_STOP_CH1_R {
        DMA_OUTLINK_STOP_CH1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dma_outlink_addr_ch1(&self) -> DMA_OUTLINK_ADDR_CH1_R {
        DMA_OUTLINK_ADDR_CH1_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_outlink_restart_ch1(&mut self) -> DMA_OUTLINK_RESTART_CH1_W {
        DMA_OUTLINK_RESTART_CH1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_outlink_start_ch1(&mut self) -> DMA_OUTLINK_START_CH1_W {
        DMA_OUTLINK_START_CH1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_outlink_stop_ch1(&mut self) -> DMA_OUTLINK_STOP_CH1_W {
        DMA_OUTLINK_STOP_CH1_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dma_outlink_addr_ch1(&mut self) -> DMA_OUTLINK_ADDR_CH1_W {
        DMA_OUTLINK_ADDR_CH1_W { w: self }
    }
}
