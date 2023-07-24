#[doc = "Register `CH4_DBG_TCR` reader"]
pub struct R(crate::R<CH4_DBG_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_DBG_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4_DBG_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4_DBG_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch4_dbg_tcr](index.html) module"]
pub struct CH4_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH4_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_dbg_tcr::R](R) reader structure"]
impl crate::Readable for CH4_DBG_TCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH4_DBG_TCR to value 0"]
impl crate::Resettable for CH4_DBG_TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
