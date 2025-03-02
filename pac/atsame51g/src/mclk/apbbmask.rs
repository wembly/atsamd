#[doc = "Register `APBBMASK` reader"]
pub struct R(crate::R<APBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBBMASK` writer"]
pub struct W(crate::W<APBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBBMASK_SPEC>;
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
impl From<crate::W<APBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_` reader - USB APB Clock Enable"]
pub struct USB__R(crate::FieldReader<bool, bool>);
impl USB__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_` writer - USB APB Clock Enable"]
pub struct USB__W<'a> {
    w: &'a mut W,
}
impl<'a> USB__W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub struct DSU__R(crate::FieldReader<bool, bool>);
impl DSU__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub struct DSU__W<'a> {
    w: &'a mut W,
}
impl<'a> DSU__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub struct NVMCTRL__R(crate::FieldReader<bool, bool>);
impl NVMCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NVMCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub struct NVMCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> NVMCTRL__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub struct PORT__R(crate::FieldReader<bool, bool>);
impl PORT__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub struct PORT__W<'a> {
    w: &'a mut W,
}
impl<'a> PORT__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub struct HMATRIX__R(crate::FieldReader<bool, bool>);
impl HMATRIX__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HMATRIX__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMATRIX__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub struct HMATRIX__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIX__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub struct EVSYS__R(crate::FieldReader<bool, bool>);
impl EVSYS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EVSYS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVSYS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub struct EVSYS__W<'a> {
    w: &'a mut W,
}
impl<'a> EVSYS__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub struct SERCOM2__R(crate::FieldReader<bool, bool>);
impl SERCOM2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub struct SERCOM2__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub struct SERCOM3__R(crate::FieldReader<bool, bool>);
impl SERCOM3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERCOM3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERCOM3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub struct SERCOM3__W<'a> {
    w: &'a mut W,
}
impl<'a> SERCOM3__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub struct TCC0__R(crate::FieldReader<bool, bool>);
impl TCC0__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub struct TCC0__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC0__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TCC1_` reader - TCC1 APB Clock Enable"]
pub struct TCC1__R(crate::FieldReader<bool, bool>);
impl TCC1__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC1_` writer - TCC1 APB Clock Enable"]
pub struct TCC1__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC1__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub struct TC2__R(crate::FieldReader<bool, bool>);
impl TC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub struct TC2__W<'a> {
    w: &'a mut W,
}
impl<'a> TC2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub struct TC3__R(crate::FieldReader<bool, bool>);
impl TC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub struct TC3__W<'a> {
    w: &'a mut W,
}
impl<'a> TC3__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RAMECC_` reader - RAMECC APB Clock Enable"]
pub struct RAMECC__R(crate::FieldReader<bool, bool>);
impl RAMECC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAMECC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMECC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMECC_` writer - RAMECC APB Clock Enable"]
pub struct RAMECC__W<'a> {
    w: &'a mut W,
}
impl<'a> RAMECC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> TC3__R {
        TC3__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    pub fn ramecc_(&self) -> RAMECC__R {
        RAMECC__R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB APB Clock Enable"]
    #[inline(always)]
    pub fn usb_(&mut self) -> USB__W {
        USB__W { w: self }
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&mut self) -> DSU__W {
        DSU__W { w: self }
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W {
        NVMCTRL__W { w: self }
    }
    #[doc = "Bit 4 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&mut self) -> PORT__W {
        PORT__W { w: self }
    }
    #[doc = "Bit 6 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&mut self) -> HMATRIX__W {
        HMATRIX__W { w: self }
    }
    #[doc = "Bit 7 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&mut self) -> EVSYS__W {
        EVSYS__W { w: self }
    }
    #[doc = "Bit 9 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&mut self) -> SERCOM2__W {
        SERCOM2__W { w: self }
    }
    #[doc = "Bit 10 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&mut self) -> SERCOM3__W {
        SERCOM3__W { w: self }
    }
    #[doc = "Bit 11 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&mut self) -> TCC0__W {
        TCC0__W { w: self }
    }
    #[doc = "Bit 12 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&mut self) -> TCC1__W {
        TCC1__W { w: self }
    }
    #[doc = "Bit 13 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&mut self) -> TC2__W {
        TC2__W { w: self }
    }
    #[doc = "Bit 14 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&mut self) -> TC3__W {
        TC3__W { w: self }
    }
    #[doc = "Bit 16 - RAMECC APB Clock Enable"]
    #[inline(always)]
    pub fn ramecc_(&mut self) -> RAMECC__W {
        RAMECC__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbbmask](index.html) module"]
pub struct APBBMASK_SPEC;
impl crate::RegisterSpec for APBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbbmask::R](R) reader structure"]
impl crate::Readable for APBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbbmask::W](W) writer structure"]
impl crate::Writable for APBBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBBMASK to value 0x0001_8056"]
impl crate::Resettable for APBBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8056
    }
}
