// OFFSET 0x7E215000
const OFFSET_AUX: usize = 0x7E215000;

// Size 3
const AUX_IRQ: usize = 0x7E215000;
// Size 3
const AUX_ENABLES: usize = 0x7E215004;

//const UART0_FR : *const u32 = (UART0_BASE + 0x18) as *const _;

// Mini UART
const AUX_MU_IO_REG: usize = 0x7E215040;
const AUX_MU_IER_REG: usize = 0x7E215044;
const AUX_MU_IIR_REG: usize = 0x7E215048;
const AUX_MU_LCR_REG: usize = 0x7E21504C;
const AUX_MU_MCR_REG: usize = 0x7E215050;
const AUX_MU_LSR_REG: usize = 0x7E215054;
const AUX_MU_MSR_REG: usize = 0x7E215058;
const AUX_MU_SCRATCH: usize = 0x7E21505C;
const AUX_MU_CNTL_REG: usize = 0x7E215060;
const AUX_MU_STAT_REG: usize = 0x7E215064;
const AUX_MU_BAUD_REG: usize = 0x7E215068;