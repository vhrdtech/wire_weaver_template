/// Enable and zero additional RAM blocks (apart from system RAM, it is zeroed in startup code)
pub(crate) fn init_ram() {
    enable_and_zero_itcm();
    enable_and_zero_dtcm();
    enable_and_zero_sram1();
    enable_and_zero_sram2();
    enable_and_zero_sram3();
    enable_and_zero_sram4();
}

pub fn enable_and_zero_itcm() {
    unsafe {
        unsafe extern "C" {
            static mut __itcm_start: u8;
            static mut __itcm_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb3enr().modify(|w| w.set_itcm1en(true));
        rcc.ahb3lpenr().modify(|w| w.set_itcmlpen(true));
        rcc.c1_ahb3lpenr().modify(|w| w.set_itcmlpen(true));
        let count = &raw const __itcm_end as usize - &raw const __itcm_start as usize;
        core::ptr::write_bytes(&raw mut __itcm_start, 0, count);
    }
}

pub fn enable_and_zero_dtcm() {
    unsafe {
        unsafe extern "C" {
            static mut __dtcm_start: u8;
            static mut __dtcm_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb3enr().modify(|w| w.set_dtcm1en(true));
        rcc.ahb3enr().modify(|w| w.set_dtcm2en(true));
        rcc.ahb3lpenr().modify(|w| w.set_d1dtcm1lpen(true));
        rcc.ahb3lpenr().modify(|w| w.set_dtcm2lpen(true));
        rcc.c1_ahb3lpenr().modify(|w| w.set_d1dtcm1lpen(true));
        rcc.c1_ahb3lpenr().modify(|w| w.set_dtcm2lpen(true));
        let count = &raw const __dtcm_end as usize - &raw const __dtcm_start as usize;
        core::ptr::write_bytes(&raw mut __dtcm_start, 0, count);
    }
}

pub fn enable_and_zero_sram1() {
    unsafe {
        unsafe extern "C" {
            static mut __sram1_start: u8;
            static mut __sram1_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb2enr().modify(|w| w.set_sram1en(true));
        rcc.ahb2lpenr().modify(|w| w.set_sram1lpen(true));
        rcc.c1_ahb2enr().modify(|w| w.set_sram1en(true));
        rcc.c1_ahb2lpenr().modify(|w| w.set_sram1lpen(true));
        let count = &raw const __sram1_end as usize - &raw const __sram1_start as usize;
        core::ptr::write_bytes(&raw mut __sram1_start, 0, count);
    }
}

pub fn enable_and_zero_sram2() {
    unsafe {
        unsafe extern "C" {
            static mut __sram2_start: u8;
            static mut __sram2_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb2enr().modify(|w| w.set_sram2en(true));
        rcc.ahb2lpenr().modify(|w| w.set_sram2lpen(true));
        rcc.c1_ahb2enr().modify(|w| w.set_sram2en(true));
        rcc.c1_ahb2lpenr().modify(|w| w.set_sram2lpen(true));
        let count = &raw const __sram2_end as usize - &raw const __sram2_start as usize;
        core::ptr::write_bytes(&raw mut __sram2_start, 0, count);
    }
}

pub fn enable_and_zero_sram3() {
    unsafe {
        unsafe extern "C" {
            static mut __sram3_start: u8;
            static mut __sram3_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb2enr().modify(|w| w.set_sram3en(true));
        rcc.ahb2lpenr().modify(|w| w.set_sram3lpen(true));
        rcc.c1_ahb2enr().modify(|w| w.set_sram3en(true));
        rcc.c1_ahb2lpenr().modify(|w| w.set_sram3lpen(true));
        let count = &raw const __sram3_end as usize - &raw const __sram3_start as usize;
        core::ptr::write_bytes(&raw mut __sram3_start, 0, count);
    }
}

pub fn enable_and_zero_sram4() {
    unsafe {
        unsafe extern "C" {
            static mut __sram4_start: u8;
            static mut __sram4_end: u8;
        }
        let rcc = embassy_stm32::pac::RCC;
        // TODO: Check that this SRAM enable code is correct
        rcc.ahb4lpenr().modify(|w| w.set_sram4lpen(true));
        rcc.c1_ahb4lpenr().modify(|w| w.set_sram4lpen(true));
        let count = &raw const __sram4_end as usize - &raw const __sram4_start as usize;
        core::ptr::write_bytes(&raw mut __sram4_start, 0, count);
    }
}
