#[doc = "Reader of register ASSIST_DEBUG_LOG_MEM_START"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_LOG_MEM_START>;
#[doc = "Writer for register ASSIST_DEBUG_LOG_MEM_START"]
pub type W = crate::W<u32, super::ASSIST_DEBUG_LOG_MEM_START>;
#[doc = "Register ASSIST_DEBUG_LOG_MEM_START `reset()`'s with value 0"]
impl crate::ResetValue for super::ASSIST_DEBUG_LOG_MEM_START {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASSIST_DEBUG_LOG_MEM_START`"]
pub type ASSIST_DEBUG_LOG_MEM_START_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ASSIST_DEBUG_LOG_MEM_START`"]
pub struct ASSIST_DEBUG_LOG_MEM_START_W<'a> {
    w: &'a mut W,
}
impl<'a> ASSIST_DEBUG_LOG_MEM_START_W<'a> {
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
    pub fn assist_debug_log_mem_start(&self) -> ASSIST_DEBUG_LOG_MEM_START_R {
        ASSIST_DEBUG_LOG_MEM_START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_log_mem_start(&mut self) -> ASSIST_DEBUG_LOG_MEM_START_W {
        ASSIST_DEBUG_LOG_MEM_START_W { w: self }
    }
}
