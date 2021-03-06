#[doc = "Reader of register DMA_INFIFO_STATUS_CH2"]
pub type R = crate::R<u32, super::DMA_INFIFO_STATUS_CH2>;
#[doc = "Reader of field `DMA_IN_BUF_HUNGRY_CH2`"]
pub type DMA_IN_BUF_HUNGRY_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_REMAIN_UNDER_4B_CH2`"]
pub type DMA_IN_REMAIN_UNDER_4B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_REMAIN_UNDER_3B_CH2`"]
pub type DMA_IN_REMAIN_UNDER_3B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_REMAIN_UNDER_2B_CH2`"]
pub type DMA_IN_REMAIN_UNDER_2B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_REMAIN_UNDER_1B_CH2`"]
pub type DMA_IN_REMAIN_UNDER_1B_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_INFIFO_CNT_CH2`"]
pub type DMA_INFIFO_CNT_CH2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMA_INFIFO_EMPTY_CH2`"]
pub type DMA_INFIFO_EMPTY_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_INFIFO_FULL_CH2`"]
pub type DMA_INFIFO_FULL_CH2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dma_in_buf_hungry_ch2(&self) -> DMA_IN_BUF_HUNGRY_CH2_R {
        DMA_IN_BUF_HUNGRY_CH2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dma_in_remain_under_4b_ch2(&self) -> DMA_IN_REMAIN_UNDER_4B_CH2_R {
        DMA_IN_REMAIN_UNDER_4B_CH2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dma_in_remain_under_3b_ch2(&self) -> DMA_IN_REMAIN_UNDER_3B_CH2_R {
        DMA_IN_REMAIN_UNDER_3B_CH2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dma_in_remain_under_2b_ch2(&self) -> DMA_IN_REMAIN_UNDER_2B_CH2_R {
        DMA_IN_REMAIN_UNDER_2B_CH2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_in_remain_under_1b_ch2(&self) -> DMA_IN_REMAIN_UNDER_1B_CH2_R {
        DMA_IN_REMAIN_UNDER_1B_CH2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dma_infifo_cnt_ch2(&self) -> DMA_INFIFO_CNT_CH2_R {
        DMA_INFIFO_CNT_CH2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_infifo_empty_ch2(&self) -> DMA_INFIFO_EMPTY_CH2_R {
        DMA_INFIFO_EMPTY_CH2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_ch2(&self) -> DMA_INFIFO_FULL_CH2_R {
        DMA_INFIFO_FULL_CH2_R::new((self.bits & 0x01) != 0)
    }
}
