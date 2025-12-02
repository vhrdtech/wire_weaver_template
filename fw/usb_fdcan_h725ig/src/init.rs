/// Corrupted content of the RTC domain due to a missed power-on reset after this domain supply voltage drop.
/// Leads to hard to debug gotchas (LSE enables by itself, PC13 / PC14 / PC15 and others set to output).
/// The solution is to reset the backup domain when RTC is not used.
/// See: http://efton.sk/STM32/gotcha/g133.html and http://efton.sk/STM32/gotcha/g62.html
pub(crate) fn reset_bkp_domain() {
    let rcc = embassy_stm32::pac::RCC;
    let pwr = embassy_stm32::pac::PWR;pwr.cr1().modify(|w| w.set_dbp(true));
    let mut cr1 = pwr.cr1().read(); // to ensure the write went through the synchronizer

    rcc.bdcr().modify(|w| w.set_bdrst(true));
    rcc.bdcr().modify(|w| w.set_bdrst(false));

    cr1.set_dbp(false);
    pwr.cr1().write_value(cr1);
}
