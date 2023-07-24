#[doc = "Register `RXOICR` reader"]
pub struct R(crate::R<RXOICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXOICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXOICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXOICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXOICR` reader - Clear-on-read receive FIFO overflow interrupt"]
pub type RXOICR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub fn rxoicr(&self) -> RXOICR_R {
        RXOICR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "RX FIFO overflow interrupt clear  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [rxoicr](index.html) module"]
pub struct RXOICR_SPEC;
impl crate::RegisterSpec for RXOICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxoicr::R](R) reader structure"]
impl crate::Readable for RXOICR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXOICR to value 0"]
impl crate::Resettable for RXOICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
