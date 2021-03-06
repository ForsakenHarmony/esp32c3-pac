#[doc = "Reader of register ASSIST_DEBUG_LOG_MEM_WRITING_ADDR"]
pub type R = crate::R<u32, super::ASSIST_DEBUG_LOG_MEM_WRITING_ADDR>;
#[doc = "Reader of field `ASSIST_DEBUG_LOG_MEM_WRITING_ADDR`"]
pub type ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn assist_debug_log_mem_writing_addr(&self) -> ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_R {
        ASSIST_DEBUG_LOG_MEM_WRITING_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
