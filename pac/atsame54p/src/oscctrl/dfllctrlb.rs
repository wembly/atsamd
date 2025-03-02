#[doc = "Register `DFLLCTRLB` reader"]
pub struct R(crate::R<DFLLCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLCTRLB` writer"]
pub struct W(crate::W<DFLLCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DFLLCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Operating Mode Selection"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operating Mode Selection"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `STABLE` reader - Stable DFLL Frequency"]
pub struct STABLE_R(crate::FieldReader<bool, bool>);
impl STABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STABLE` writer - Stable DFLL Frequency"]
pub struct STABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> STABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `LLAW` reader - Lose Lock After Wake"]
pub struct LLAW_R(crate::FieldReader<bool, bool>);
impl LLAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LLAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLAW` writer - Lose Lock After Wake"]
pub struct LLAW_W<'a> {
    w: &'a mut W,
}
impl<'a> LLAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `USBCRM` reader - USB Clock Recovery Mode"]
pub struct USBCRM_R(crate::FieldReader<bool, bool>);
impl USBCRM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USBCRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCRM` writer - USB Clock Recovery Mode"]
pub struct USBCRM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CCDIS` reader - Chill Cycle Disable"]
pub struct CCDIS_R(crate::FieldReader<bool, bool>);
impl CCDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCDIS` writer - Chill Cycle Disable"]
pub struct CCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `QLDIS` reader - Quick Lock Disable"]
pub struct QLDIS_R(crate::FieldReader<bool, bool>);
impl QLDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QLDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QLDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QLDIS` writer - Quick Lock Disable"]
pub struct QLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u8 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BPLCKC` reader - Bypass Coarse Lock"]
pub struct BPLCKC_R(crate::FieldReader<bool, bool>);
impl BPLCKC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BPLCKC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPLCKC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPLCKC` writer - Bypass Coarse Lock"]
pub struct BPLCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> BPLCKC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `WAITLOCK` reader - Wait Lock"]
pub struct WAITLOCK_R(crate::FieldReader<bool, bool>);
impl WAITLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAITLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAITLOCK` writer - Wait Lock"]
pub struct WAITLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&self) -> STABLE_R {
        STABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&self) -> LLAW_R {
        LLAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&self) -> USBCRM_R {
        USBCRM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&self) -> CCDIS_R {
        CCDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&self) -> QLDIS_R {
        QLDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&self) -> BPLCKC_R {
        BPLCKC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&self) -> WAITLOCK_R {
        WAITLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 1 - Stable DFLL Frequency"]
    #[inline(always)]
    pub fn stable(&mut self) -> STABLE_W {
        STABLE_W { w: self }
    }
    #[doc = "Bit 2 - Lose Lock After Wake"]
    #[inline(always)]
    pub fn llaw(&mut self) -> LLAW_W {
        LLAW_W { w: self }
    }
    #[doc = "Bit 3 - USB Clock Recovery Mode"]
    #[inline(always)]
    pub fn usbcrm(&mut self) -> USBCRM_W {
        USBCRM_W { w: self }
    }
    #[doc = "Bit 4 - Chill Cycle Disable"]
    #[inline(always)]
    pub fn ccdis(&mut self) -> CCDIS_W {
        CCDIS_W { w: self }
    }
    #[doc = "Bit 5 - Quick Lock Disable"]
    #[inline(always)]
    pub fn qldis(&mut self) -> QLDIS_W {
        QLDIS_W { w: self }
    }
    #[doc = "Bit 6 - Bypass Coarse Lock"]
    #[inline(always)]
    pub fn bplckc(&mut self) -> BPLCKC_W {
        BPLCKC_W { w: self }
    }
    #[doc = "Bit 7 - Wait Lock"]
    #[inline(always)]
    pub fn waitlock(&mut self) -> WAITLOCK_W {
        WAITLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL48M Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllctrlb](index.html) module"]
pub struct DFLLCTRLB_SPEC;
impl crate::RegisterSpec for DFLLCTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllctrlb::R](R) reader structure"]
impl crate::Readable for DFLLCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllctrlb::W](W) writer structure"]
impl crate::Writable for DFLLCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLLCTRLB to value 0"]
impl crate::Resettable for DFLLCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
