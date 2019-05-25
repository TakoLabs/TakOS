all: multiboot_header.o boot.o long_mode_init.o

multiboot_header.o: multiboot_header.asm
	nasm -f elf64 -o ../$(BUILD)/$@ $^

boot.o: boot.asm
	nasm -f elf64 -o ../$(BUILD)/$@ $^

long_mode_init.o: long_mode_init.asm
	nasm -f elf64 -o ../$(BUILD)/$@ $^

clean:
	rm -f ../$(BUILD)/*.o
