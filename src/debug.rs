use memory::PAGE_SIZE;
use multiboot2::BootInformation;

pub fn debug(boot_info: &BootInformation) {
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Memory map tag required");

    //Map binary sections.
    for section in elf_sections_tag.sections() {
        if !section.is_allocated() {
            continue;
        }

        assert!(
            section.addr as usize % PAGE_SIZE == 0,
            "sections need to be page aligned"
        );

        println!("mapping section at addr: {:#x}, size: {:#x}",
                 section.addr,
                 section.size
        );
    }
}
